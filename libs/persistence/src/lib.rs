use ai::GpuClass;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DepreciationRule {
    pub id: i32,
    pub package_id: Uuid,
    pub final_depreciated_percentage: f64,
    pub full_depreciation_hours: i32,
    pub depreciation_curve: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PackageProvenance {
    pub id: i32,
    pub package_id: Uuid,
    pub provenance_type: String,
    pub usage_hours: i32,
    pub quantity_available: i32,
    pub is_active: bool,
    pub calculated_price_usdc: Option<i32>,
    pub discount_percentage: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PackageImage {
    pub package_id: Uuid,
    pub filename: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Package {
    pub id: Uuid,
    pub name: String,
    pub sku: Option<String>,
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
    pub availability_type: String,
    pub availability_value: Option<i32>,
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
                id, name, sku, description, hardware_description,
                cpu_cores, ram_gb, storage_gb, gpu_class::text as gpu_class,
                gpu_count, vram_gb, setup_price_usdc, monthly_price_usdc,
                availability_type, availability_value,
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
                sku: row.get("sku"),
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
                availability_type: row.get("availability_type"),
                availability_value: row.get("availability_value"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(packages)
    }

    pub async fn get_depreciation_rules(&self) -> Result<Vec<DepreciationRule>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, package_id, final_depreciated_percentage::float8 as final_depreciated_percentage,
                full_depreciation_hours, depreciation_curve,
                created_at, updated_at
            FROM package_depreciation_rules
            ORDER BY package_id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let rules = rows
            .into_iter()
            .map(|row| DepreciationRule {
                id: row.get("id"),
                package_id: row.get("package_id"),
                final_depreciated_percentage: row.get("final_depreciated_percentage"),
                full_depreciation_hours: row.get("full_depreciation_hours"),
                depreciation_curve: row.get("depreciation_curve"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(rules)
    }

    pub async fn get_depreciation_rule_by_package_id(
        &self,
        package_id: Uuid,
    ) -> Result<Option<DepreciationRule>> {
        let row = sqlx::query(
            r#"
            SELECT
                id, package_id, final_depreciated_percentage::float8 as final_depreciated_percentage,
                full_depreciation_hours, depreciation_curve,
                created_at, updated_at
            FROM package_depreciation_rules
            WHERE package_id = $1
            "#,
        )
        .bind(package_id)
        .fetch_optional(&self.pool)
        .await?;

        let rule = row.map(|r| DepreciationRule {
            id: r.get("id"),
            package_id: r.get("package_id"),
            final_depreciated_percentage: r.get("final_depreciated_percentage"),
            full_depreciation_hours: r.get("full_depreciation_hours"),
            depreciation_curve: r.get("depreciation_curve"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        });

        Ok(rule)
    }

    pub async fn get_package_provenances(
        &self,
        package_id: Uuid,
    ) -> Result<Vec<PackageProvenance>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, package_id, provenance_type, usage_hours,
                quantity_available, is_active, calculated_price_usdc,
                discount_percentage::float8 as discount_percentage, created_at, updated_at
            FROM package_provenance
            WHERE package_id = $1 AND is_active = true
            ORDER BY usage_hours ASC
            "#,
        )
        .bind(package_id)
        .fetch_all(&self.pool)
        .await?;

        let provenances = rows
            .into_iter()
            .map(|row| PackageProvenance {
                id: row.get("id"),
                package_id: row.get("package_id"),
                provenance_type: row.get("provenance_type"),
                usage_hours: row.get("usage_hours"),
                quantity_available: row.get("quantity_available"),
                is_active: row.get("is_active"),
                calculated_price_usdc: row.get("calculated_price_usdc"),
                discount_percentage: row.get("discount_percentage"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(provenances)
    }

    pub async fn get_all_package_provenances(&self) -> Result<Vec<PackageProvenance>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, package_id, provenance_type, usage_hours,
                quantity_available, is_active, calculated_price_usdc,
                discount_percentage::float8 as discount_percentage, created_at, updated_at
            FROM package_provenance
            WHERE is_active = true
            ORDER BY package_id, usage_hours ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let provenances = rows
            .into_iter()
            .map(|row| PackageProvenance {
                id: row.get("id"),
                package_id: row.get("package_id"),
                provenance_type: row.get("provenance_type"),
                usage_hours: row.get("usage_hours"),
                quantity_available: row.get("quantity_available"),
                is_active: row.get("is_active"),
                calculated_price_usdc: row.get("calculated_price_usdc"),
                discount_percentage: row.get("discount_percentage"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(provenances)
    }

    pub async fn get_package_by_id(&self, package_id: Uuid) -> Result<Option<Package>> {
        let row = sqlx::query(
            r#"
            SELECT
                id, name, sku, description, hardware_description,
                cpu_cores, ram_gb, storage_gb, gpu_class::text as gpu_class,
                gpu_count, vram_gb, setup_price_usdc, monthly_price_usdc,
                availability_type, availability_value,
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
            sku: r.get("sku"),
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
            availability_type: r.get("availability_type"),
            availability_value: r.get("availability_value"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
        });

        Ok(package)
    }

    pub async fn get_package_by_sku(&self, sku: &str) -> Result<Option<Package>> {
        let row = sqlx::query(
            r#"
            SELECT
                id, name, sku, description, hardware_description,
                cpu_cores, ram_gb, storage_gb, gpu_class::text as gpu_class,
                gpu_count, vram_gb, setup_price_usdc, monthly_price_usdc,
                availability_type, availability_value,
                is_active, created_at
            FROM packages
            WHERE sku = $1 AND is_active = true
            "#,
        )
        .bind(sku)
        .fetch_optional(&self.pool)
        .await?;

        let package = row.map(|r| Package {
            id: r.get("id"),
            name: r.get("name"),
            sku: r.get("sku"),
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
            availability_type: r.get("availability_type"),
            availability_value: r.get("availability_value"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
        });

        Ok(package)
    }

    pub async fn get_package_images(&self, package_id: Uuid) -> Result<Vec<PackageImage>> {
        let rows = sqlx::query(
            r#"
            SELECT filename, title, description
            FROM package_images
            WHERE package_id = $1
            ORDER BY sort_order ASC
            "#,
        )
        .bind(package_id)
        .fetch_all(&self.pool)
        .await?;

        let images = rows
            .into_iter()
            .map(|row| PackageImage {
                package_id,
                filename: row.get::<String, _>("filename"),
                title: row.get::<String, _>("title"),
                description: row.get::<String, _>("description"),
            })
            .collect();

        Ok(images)
    }

    pub async fn create_order(
        &self,
        org_id: Uuid,
        cpu_cores: i16,
        ram_gb: i16,
        storage_gb: i32,
        gpu: String,
        pq_enabled: bool,
        notes: Option<String>,
    ) -> Result<ServerOrder> {
        let order_id = Uuid::new_v4();

        let query = sqlx::query(
            r#"
            INSERT INTO server_orders (id, org_id, plan_cpu_cores, plan_ram_gb, plan_storage_gb, plan_gpu, pq_enabled, notes, status)
            VALUES ($1, $2, $3, $4, $5, $6::gpu_class, $7, $8, 'queued')
            RETURNING id, org_id, plan_cpu_cores, plan_ram_gb, plan_storage_gb, plan_gpu::text as plan_gpu, pq_enabled, notes, status, created_at
            "#,
        )
        .bind(order_id)
        .bind(org_id)
        .bind(cpu_cores)
        .bind(ram_gb)
        .bind(storage_gb)
        .bind(gpu)
        .bind(pq_enabled)
        .bind(notes)
        .fetch_one(&self.pool)
        .await?;

        Ok(ServerOrder {
            id: query.get("id"),
            org_id: query.get("org_id"),
            plan_cpu_cores: query.get("plan_cpu_cores"),
            plan_ram_gb: query.get("plan_ram_gb"),
            plan_storage_gb: query.get("plan_storage_gb"),
            plan_gpu: query.get("plan_gpu"),
            pq_enabled: query.get("pq_enabled"),
            notes: query.get("notes"),
            status: query.get("status"),
            created_at: query.get("created_at"),
        })
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
            VALUES ($1, $2, $3, $4, $5, $6::gpu_class, $7, $8, 'queued')
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
                plan_gpu::text as plan_gpu, pq_enabled, notes, status, created_at
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
