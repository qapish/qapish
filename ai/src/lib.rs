use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GpuClass {
    None,
    L4,
    A100_40G,
    A100_80G,
    H100_80G,
    RTX_4090,
    RTX_5090,
    Radeon_8060S,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageImage {
    pub filename: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepreciationRule {
    pub package_id: Uuid,
    pub final_depreciated_percentage: f64, // Percentage of original price after full depreciation (e.g., 25.0)
    pub full_depreciation_hours: u32,      // Hours for full depreciation (e.g., 26280 for 3 years)
    pub depreciation_curve: DepreciationCurve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DepreciationCurve {
    Linear,
    Exponential,
    Stepped,
}

impl DepreciationRule {
    /// Calculate the depreciated price based on usage hours
    pub fn calculate_depreciated_price(&self, original_price: u32, usage_hours: u32) -> u32 {
        if usage_hours == 0 {
            return original_price;
        }

        if usage_hours >= self.full_depreciation_hours {
            // At or beyond full depreciation
            return (original_price as f64 * self.final_depreciated_percentage / 100.0) as u32;
        }

        let depreciation_factor = match self.depreciation_curve {
            DepreciationCurve::Linear => {
                // Linear depreciation: price decreases linearly with usage
                let usage_ratio = usage_hours as f64 / self.full_depreciation_hours as f64;
                let price_range = 100.0 - self.final_depreciated_percentage;
                100.0 - (price_range * usage_ratio)
            }
            DepreciationCurve::Exponential => {
                // Exponential depreciation: faster initial depreciation
                let usage_ratio = usage_hours as f64 / self.full_depreciation_hours as f64;
                let k = -(1.0_f64.ln() - (self.final_depreciated_percentage / 100.0).ln());
                100.0 * (-k * usage_ratio).exp()
            }
            DepreciationCurve::Stepped => {
                // Stepped depreciation: discrete steps (e.g., every 6 months)
                let steps = 6; // Number of depreciation steps
                let usage_ratio = usage_hours as f64 / self.full_depreciation_hours as f64;
                let step = (usage_ratio * steps as f64).floor() as u32;
                let price_range = 100.0 - self.final_depreciated_percentage;
                100.0 - (price_range * step as f64 / steps as f64)
            }
        };

        (original_price as f64 * depreciation_factor / 100.0) as u32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Availability {
    Preorder,
    InStock,
    Build { hours: u16 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Provenance {
    New,
    Used { hours: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceOption {
    pub provenance_type: Provenance,
    pub quantity_available: u32,
    pub calculated_price: u32,
    pub discount_percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: Uuid,
    pub name: String,
    pub sku: String,
    pub description: String,
    pub hardware_description: String,
    pub cpu_cores: u16,
    pub ram_gb: u16,
    pub storage_gb: u32,
    pub gpu_class: GpuClass,
    pub gpu_count: u16,
    pub vram_gb: u16,
    pub setup_price_usdc: u32,   // Original price for new equipment
    pub monthly_price_usdc: u32, // Monthly hosting price (not affected by depreciation)
    pub depreciation_rule: Option<DepreciationRule>, // Depreciation rule for this package
    pub images: Vec<PackageImage>,
    pub availability: Availability,
    pub provenances: Vec<ProvenanceOption>, // Multiple provenance options available
    pub min_price_usdc: Option<u32>,        // Lowest price among all provenance options
    pub max_price_usdc: Option<u32>,        // Highest price (usually new equipment)
}

impl Package {
    /// Calculate price range from all provenance options
    pub fn calculate_price_range(&mut self) {
        if self.provenances.is_empty() {
            self.min_price_usdc = Some(self.setup_price_usdc);
            self.max_price_usdc = Some(self.setup_price_usdc);
            return;
        }

        let prices: Vec<u32> = self
            .provenances
            .iter()
            .map(|p| p.calculated_price)
            .collect();
        self.min_price_usdc = prices.iter().min().cloned();
        self.max_price_usdc = prices.iter().max().cloned();
    }

    /// Get the lowest available price
    pub fn get_min_price(&self) -> u32 {
        self.min_price_usdc.unwrap_or(self.setup_price_usdc)
    }

    /// Get the highest available price (usually new equipment)
    pub fn get_max_price(&self) -> u32 {
        self.max_price_usdc.unwrap_or(self.setup_price_usdc)
    }

    /// Check if package has used equipment options
    pub fn has_used_options(&self) -> bool {
        self.provenances
            .iter()
            .any(|p| matches!(p.provenance_type, Provenance::Used { .. }))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerOrder {
    pub id: Uuid,
    pub plan: Plan,
    pub pq_enabled: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub cpu_cores: u16,
    pub ram_gb: u16,
    pub storage_gb: u32,
    pub gpu: GpuClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub plan: Plan,
    pub pq_enabled: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: Uuid,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokenResponse {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    pub id: Uuid,
    pub plan: Plan,
    pub status: String,
}
