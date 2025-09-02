use ai::{CreateOrderRequest, CreateOrderResponse, OrderSummary};
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

#[derive(Default)]
pub struct InfraState {
    // demo in-memory store; swap for DB or control-plane calls
    orders: RwLock<HashMap<Uuid, OrderSummary>>,
}

impl InfraState {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn create_order(&self, req: CreateOrderRequest) -> Result<CreateOrderResponse> {
        let id = Uuid::new_v4();
        let summary = OrderSummary {
            id,
            plan: req.plan,
            status: "provisioning".into(),
        };
        self.orders.write().await.insert(id, summary);
        info!("Queued provisioning for order {id}");
        // TODO: spawn provisioning workflow here (systemd/K8s/Nomad/etc.)
        Ok(CreateOrderResponse {
            order_id: id,
            status: "queued".into(),
        })
    }

    pub async fn list_orders(&self) -> Result<Vec<OrderSummary>> {
        Ok(self.orders.read().await.values().cloned().collect())
    }
}
