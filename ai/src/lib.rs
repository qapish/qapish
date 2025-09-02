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
pub struct Package {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub hardware_description: String,
    pub cpu_cores: u16,
    pub ram_gb: u16,
    pub storage_gb: u32,
    pub gpu_class: GpuClass,
    pub gpu_count: u16,
    pub vram_gb: u16,
    pub setup_price_usdc: u32,
    pub monthly_price_usdc: u32,
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
