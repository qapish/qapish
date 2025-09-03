use ai::{
    Availability, CreateOrderRequest, CreateOrderResponse, GpuClass, OrderSummary, Package,
    PackageImage, Provenance,
};
use anyhow::Result;
use persistence::Database;
use tracing::info;
use uuid::Uuid;

pub struct InfraState {
    db: Database,
    // TODO: Add demo org_id for now - in real app this would come from JWT
    demo_org_id: Uuid,
}

impl InfraState {
    pub async fn new() -> Result<Self> {
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql:///qapish".to_string());

        // Connect to database (required)
        let db = Database::new(&database_url).await?;
        info!("Connected to database");

        // Create a demo organization for testing
        let demo_org_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")
            .unwrap_or_else(|_| Uuid::new_v4());

        Ok(Self { db, demo_org_id })
    }

    pub async fn get_packages(&self) -> Result<Vec<Package>> {
        // Fetch packages from database
        let db_packages = self.db.get_active_packages().await?;

        // Fetch all provenance options at once for efficiency
        let all_provenances = self.db.get_all_package_provenances().await?;

        // Convert from persistence::Package to ai::Package
        let mut packages = Vec::new();

        for p in db_packages {
            // Get depreciation rule for this package
            let depreciation_rule = self.db.get_depreciation_rule_by_package_id(p.id).await?;

            // Convert depreciation rule
            let ai_depreciation_rule = depreciation_rule.map(|dr| ai::DepreciationRule {
                package_id: dr.package_id,
                final_depreciated_percentage: dr.final_depreciated_percentage,
                full_depreciation_hours: dr.full_depreciation_hours as u32,
                depreciation_curve: match dr.depreciation_curve.as_str() {
                    "exponential" => ai::DepreciationCurve::Exponential,
                    "stepped" => ai::DepreciationCurve::Stepped,
                    _ => ai::DepreciationCurve::Linear,
                },
            });

            // Parse availability
            let availability = match p.availability_type.as_str() {
                "preorder" => Availability::Preorder,
                "in_stock" => Availability::InStock,
                "build" => Availability::Build {
                    hours: p.availability_value.unwrap_or(48) as u16,
                },
                _ => Availability::InStock,
            };

            // Get provenance options for this package
            let package_provenances: Vec<_> = all_provenances
                .iter()
                .filter(|prov| prov.package_id == p.id)
                .collect();

            // Convert provenance options
            let mut provenance_options = Vec::new();
            for prov in package_provenances {
                let provenance_type = match prov.provenance_type.as_str() {
                    "new" => Provenance::New,
                    "used" => Provenance::Used {
                        hours: prov.usage_hours as u32,
                    },
                    _ => Provenance::New,
                };

                provenance_options.push(ai::ProvenanceOption {
                    provenance_type,
                    quantity_available: prov.quantity_available as u32,
                    calculated_price: prov.calculated_price_usdc.unwrap_or(p.setup_price_usdc)
                        as u32,
                    discount_percentage: prov.discount_percentage,
                });
            }

            // Load package images
            let db_images = self.db.get_package_images(p.id).await?;
            let images = db_images
                .into_iter()
                .map(|img| PackageImage {
                    filename: img.filename,
                    title: img.title,
                    description: img.description,
                })
                .collect();

            // Create the package
            let gpu_class = p.gpu_class_enum();
            let mut package = Package {
                id: p.id,
                name: p.name,
                sku: p.sku.unwrap_or_default(),
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
                depreciation_rule: ai_depreciation_rule,
                images,
                availability,
                provenances: provenance_options,
                min_price_usdc: None,
                max_price_usdc: None,
            };

            // Calculate price range from all provenance options
            package.calculate_price_range();

            packages.push(package);
        }

        Ok(packages)
    }

    pub async fn get_package_by_sku(&self, sku: &str) -> Result<Option<Package>> {
        let db_package = self.db.get_package_by_sku(sku).await?;

        match db_package {
            Some(p) => {
                // Get depreciation rule for this package
                let depreciation_rule = self.db.get_depreciation_rule_by_package_id(p.id).await?;

                // Convert depreciation rule
                let ai_depreciation_rule = depreciation_rule.map(|dr| ai::DepreciationRule {
                    package_id: dr.package_id,
                    final_depreciated_percentage: dr.final_depreciated_percentage,
                    full_depreciation_hours: dr.full_depreciation_hours as u32,
                    depreciation_curve: match dr.depreciation_curve.as_str() {
                        "exponential" => ai::DepreciationCurve::Exponential,
                        "stepped" => ai::DepreciationCurve::Stepped,
                        _ => ai::DepreciationCurve::Linear,
                    },
                });

                // Parse availability
                let availability = match p.availability_type.as_str() {
                    "preorder" => Availability::Preorder,
                    "in_stock" => Availability::InStock,
                    "build" => Availability::Build {
                        hours: p.availability_value.unwrap_or(48) as u16,
                    },
                    _ => Availability::InStock,
                };

                // Get provenance options for this package
                let package_provenances = self.db.get_package_provenances(p.id).await?;

                // Convert provenance options
                let mut provenance_options = Vec::new();
                for prov in package_provenances {
                    let provenance_type = match prov.provenance_type.as_str() {
                        "new" => Provenance::New,
                        "used" => Provenance::Used {
                            hours: prov.usage_hours as u32,
                        },
                        _ => Provenance::New,
                    };

                    provenance_options.push(ai::ProvenanceOption {
                        provenance_type,
                        quantity_available: prov.quantity_available as u32,
                        calculated_price: prov.calculated_price_usdc.unwrap_or(p.setup_price_usdc)
                            as u32,
                        discount_percentage: prov.discount_percentage,
                    });
                }

                // Load package images
                let db_images = self.db.get_package_images(p.id).await?;
                let images = db_images
                    .into_iter()
                    .map(|img| PackageImage {
                        filename: img.filename,
                        title: img.title,
                        description: img.description,
                    })
                    .collect();

                // Create the package
                let gpu_class = p.gpu_class_enum();
                let mut package = Package {
                    id: p.id,
                    name: p.name,
                    sku: p.sku.unwrap_or_default(),
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
                    depreciation_rule: ai_depreciation_rule,
                    images,
                    availability,
                    provenances: provenance_options,
                    min_price_usdc: None,
                    max_price_usdc: None,
                };

                // Calculate price range from all provenance options
                package.calculate_price_range();

                Ok(Some(package))
            }
            None => Ok(None),
        }
    }

    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<CreateOrderResponse> {
        // Convert GPU class to string for database
        let gpu_string = format!("{:?}", request.plan.gpu);

        // Create the order in the database
        let order = self
            .db
            .create_order(
                self.demo_org_id,
                request.plan.cpu_cores as i16,
                request.plan.ram_gb as i16,
                request.plan.storage_gb as i32,
                gpu_string,
                request.pq_enabled,
                request.notes,
            )
            .await?;

        Ok(CreateOrderResponse {
            order_id: order.id,
            status: order.status,
        })
    }

    pub async fn get_orders(&self) -> Result<Vec<OrderSummary>> {
        let db_orders = self.db.get_orders_for_org(self.demo_org_id).await?;

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
