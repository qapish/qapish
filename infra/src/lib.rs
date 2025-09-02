use ai::{CreateOrderRequest, CreateOrderResponse, GpuClass, OrderSummary, Package};
use anyhow::Result;
use persistence::Database;
use tracing::info;
use uuid::Uuid;

pub struct InfraState {
    db: Option<Database>,
    // TODO: Add demo org_id for now - in real app this would come from JWT
    demo_org_id: Uuid,
    demo_mode: bool,
}

impl InfraState {
    pub async fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/qapish".to_string());

        // Try to connect to database, fallback to demo mode
        let (db, demo_mode) = match Database::new(&database_url).await {
            Ok(database) => (Some(database), false),
            Err(_) => {
                info!("Database connection failed, running in demo mode");
                (None, true)
            }
        };

        // Create a demo organization for testing
        let demo_org_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")
            .unwrap_or_else(|_| Uuid::new_v4());

        Ok(Self {
            db,
            demo_org_id,
            demo_mode,
        })
    }

    pub async fn get_packages(&self) -> Result<Vec<Package>> {
        if self.demo_mode {
            // Return hardcoded demo packages
            let packages = vec![
                Package {
                    id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
                    name: "Midrange Consumer".to_string(),
                    description: "Perfect for development and small-scale inference workloads"
                        .to_string(),
                    hardware_description:
                        "GMKtek X2 (or similar) with AMD Radeon 8060S 96GB and 32GB system RAM"
                            .to_string(),
                    cpu_cores: 16,
                    ram_gb: 32,
                    storage_gb: 2000,
                    gpu_class: GpuClass::Radeon_8060S,
                    gpu_count: 1,
                    vram_gb: 96,
                    setup_price_usdc: 3000,
                    monthly_price_usdc: 200,
                },
                Package {
                    id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap(),
                    name: "Top Consumer".to_string(),
                    description: "High-performance setup for demanding AI applications".to_string(),
                    hardware_description:
                        "Ryzen 9950, 64GB RAM, dual 32GB RTX 5090s, liquid cooling".to_string(),
                    cpu_cores: 16,
                    ram_gb: 64,
                    storage_gb: 4000,
                    gpu_class: GpuClass::RTX_5090,
                    gpu_count: 2,
                    vram_gb: 64,
                    setup_price_usdc: 20000,
                    monthly_price_usdc: 500,
                },
                Package {
                    id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440003").unwrap(),
                    name: "Pro Server".to_string(),
                    description: "Enterprise-grade AI infrastructure for production workloads"
                        .to_string(),
                    hardware_description: "Dual H100 80GB with enterprise cooling and redundancy"
                        .to_string(),
                    cpu_cores: 32,
                    ram_gb: 128,
                    storage_gb: 8000,
                    gpu_class: GpuClass::H100_80G,
                    gpu_count: 2,
                    vram_gb: 160,
                    setup_price_usdc: 100000,
                    monthly_price_usdc: 1000,
                },
            ];
            return Ok(packages);
        }

        let db_packages = self.db.as_ref().unwrap().get_active_packages().await?;

        // Convert from persistence::Package to ai::Package
        let packages = db_packages
            .into_iter()
            .map(|p| {
                let gpu_class = p.gpu_class_enum();
                Package {
                    id: p.id,
                    name: p.name,
                    description: p.description,
                    hardware_description: p.hardware_description,
                    cpu_cores: p.cpu_cores as u16,
                    ram_gb: p.ram_gb as u16,
                    storage_gb: p.storage_gb as u32,
                    gpu_class,
                    gpu_count: p.gpu_count as u16,
                    vram_gb: p.vram_gb as u16,
                    setup_price_usdc: p.setup_price_usdc as u32,
                    monthly_price_usdc: p.monthly_price_usdc as u32,
                }
            })
            .collect();

        Ok(packages)
    }

    pub async fn create_order(&self, _req: CreateOrderRequest) -> Result<CreateOrderResponse> {
        // For now, create a basic order - in a real app we'd validate the plan against packages
        let order_id = Uuid::new_v4();

        info!("Queued provisioning for order {order_id}");
        // TODO: spawn provisioning workflow here (systemd/K8s/Nomad/etc.)

        Ok(CreateOrderResponse {
            order_id,
            status: "queued".into(),
        })
    }

    pub async fn list_orders(&self) -> Result<Vec<OrderSummary>> {
        if self.demo_mode {
            // Return demo orders
            return Ok(vec![OrderSummary {
                id: Uuid::new_v4(),
                plan: ai::Plan {
                    cpu_cores: 16,
                    ram_gb: 64,
                    storage_gb: 4000,
                    gpu: GpuClass::RTX_5090,
                },
                status: "provisioning".to_string(),
            }]);
        }

        let db_orders = self
            .db
            .as_ref()
            .unwrap()
            .get_orders_for_org(self.demo_org_id)
            .await?;

        // Convert from persistence::ServerOrder to ai::OrderSummary
        let orders = db_orders
            .into_iter()
            .map(|o| {
                let gpu_class = match o.plan_gpu.as_str() {
                    "None" => GpuClass::None,
                    "L4" => GpuClass::L4,
                    "A100_40G" => GpuClass::A100_40G,
                    "A100_80G" => GpuClass::A100_80G,
                    "H100_80G" => GpuClass::H100_80G,
                    "RTX_4090" => GpuClass::RTX_4090,
                    "RTX_5090" => GpuClass::RTX_5090,
                    "Radeon_8060S" => GpuClass::Radeon_8060S,
                    _ => GpuClass::None,
                };

                OrderSummary {
                    id: o.id,
                    plan: ai::Plan {
                        cpu_cores: o.plan_cpu_cores as u16,
                        ram_gb: o.plan_ram_gb as u16,
                        storage_gb: o.plan_storage_gb as u32,
                        gpu: gpu_class,
                    },
                    status: o.status,
                }
            })
            .collect();

        Ok(orders)
    }
}
