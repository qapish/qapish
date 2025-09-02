use ai::GpuClass;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Package {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub hardware_description: String,
    pub cpu_cores: i16,
    pub ram_gb: i16,
    pub storage_gb: i32,
    pub gpu_class: String, // We'll convert to/from GpuClass
    pub gpu_count: i16,
    pub vram_gb: i16,
    pub setup_price_usdc: i32,
    pub monthly_price_usdc: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl Package {
    pub fn gpu_class_enum(&self) -> GpuClass {
        match self.gpu_class.as_str() {
            "None" => GpuClass::None,
            "L4" => GpuClass::L4,
            "A100_40G" => GpuClass::A100_40G,
            "A100_80G" => GpuClass::A100_80G,
            "H100_80G" => GpuClass::H100_80G,
            "RTX_4090" => GpuClass::RTX_4090,
            "RTX_5090" => GpuClass::RTX_5090,
            "Radeon_8060S" => GpuClass::Radeon_8060S,
            _ => GpuClass::None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerOrder {
    pub id: Uuid,
    pub org_id: Uuid,
    pub plan_cpu_cores: i16,
    pub plan_ram_gb: i16,
    pub plan_storage_gb: i32,
    pub plan_gpu: String,
    pub pq_enabled: bool,
    pub notes: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn get_active_packages(&self) -> Result<Vec<Package>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, name, description, hardware_description,
                cpu_cores, ram_gb, storage_gb, gpu_class,
                gpu_count, vram_gb, setup_price_usdc, monthly_price_usdc,
                is_active, created_at
            FROM packages
            WHERE is_active = true
            ORDER BY setup_price_usdc ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let packages = rows
            .into_iter()
            .map(|row| Package {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                hardware_description: row.get("hardware_description"),
                cpu_cores: row.get("cpu_cores"),
                ram_gb: row.get("ram_gb"),
                storage_gb: row.get("storage_gb"),
                gpu_class: row.get("gpu_class"),
                gpu_count: row.get("gpu_count"),
                vram_gb: row.get("vram_gb"),
                setup_price_usdc: row.get("setup_price_usdc"),
                monthly_price_usdc: row.get("monthly_price_usdc"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(packages)
    }

    pub async fn get_package_by_id(&self, package_id: Uuid) -> Result<Option<Package>> {
        let row = sqlx::query(
            r#"
            SELECT
                id, name, description, hardware_description,
                cpu_cores, ram_gb, storage_gb, gpu_class,
                gpu_count, vram_gb, setup_price_usdc, monthly_price_usdc,
                is_active, created_at
            FROM packages
            WHERE id = $1 AND is_active = true
            "#,
        )
        .bind(package_id)
        .fetch_optional(&self.pool)
        .await?;

        let package = row.map(|r| Package {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            hardware_description: r.get("hardware_description"),
            cpu_cores: r.get("cpu_cores"),
            ram_gb: r.get("ram_gb"),
            storage_gb: r.get("storage_gb"),
            gpu_class: r.get("gpu_class"),
            gpu_count: r.get("gpu_count"),
            vram_gb: r.get("vram_gb"),
            setup_price_usdc: r.get("setup_price_usdc"),
            monthly_price_usdc: r.get("monthly_price_usdc"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
        });

        Ok(package)
    }

    pub async fn create_server_order(
        &self,
        org_id: Uuid,
        package: &Package,
        pq_enabled: bool,
        notes: Option<String>,
    ) -> Result<Uuid> {
        let order_id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO server_orders
            (id, org_id, plan_cpu_cores, plan_ram_gb, plan_storage_gb, plan_gpu, pq_enabled, notes, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'queued')
            "#
        )
        .bind(order_id)
        .bind(org_id)
        .bind(package.cpu_cores)
        .bind(package.ram_gb)
        .bind(package.storage_gb)
        .bind(&package.gpu_class)
        .bind(pq_enabled)
        .bind(&notes)
        .execute(&self.pool)
        .await?;

        Ok(order_id)
    }

    pub async fn get_orders_for_org(&self, org_id: Uuid) -> Result<Vec<ServerOrder>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, org_id, plan_cpu_cores, plan_ram_gb, plan_storage_gb,
                plan_gpu, pq_enabled, notes, status, created_at
            FROM server_orders
            WHERE org_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        let orders = rows
            .into_iter()
            .map(|row| ServerOrder {
                id: row.get("id"),
                org_id: row.get("org_id"),
                plan_cpu_cores: row.get("plan_cpu_cores"),
                plan_ram_gb: row.get("plan_ram_gb"),
                plan_storage_gb: row.get("plan_storage_gb"),
                plan_gpu: row.get("plan_gpu"),
                pq_enabled: row.get("pq_enabled"),
                notes: row.get("notes"),
                status: row.get("status"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(orders)
    }

    pub async fn create_organization(&self, name: &str) -> Result<Uuid> {
        let org_id = Uuid::new_v4();

        sqlx::query("INSERT INTO organizations (id, name) VALUES ($1, $2)")
            .bind(org_id)
            .bind(name)
            .execute(&self.pool)
            .await?;

        Ok(org_id)
    }

    pub async fn create_user(&self, org_id: Uuid, email: &str, pwd_hash: &str) -> Result<Uuid> {
        let user_id = Uuid::new_v4();

        sqlx::query("INSERT INTO users (id, org_id, email, pwd_hash) VALUES ($1, $2, $3, $4)")
            .bind(user_id)
            .bind(org_id)
            .bind(email)
            .bind(pwd_hash)
            .execute(&self.pool)
            .await?;

        Ok(user_id)
    }
}
