use ai::*;
use axum::extract::Path;
use axum::{
    extract::State,
    http::{Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use infra::InfraState;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    infra: Arc<InfraState>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    // logging
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let infra = InfraState::new().await?;
    let state = AppState {
        infra: Arc::new(infra),
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    // static (dist from web) served at /
    let spa = ServeDir::new("./web/dist");

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/auth/signup", post(signup))
        .route("/api/auth/login", post(login))
        .route("/api/packages", get(list_packages))
        .route("/api/packages/:sku", get(get_package_by_sku))
        .route("/api/orders", get(list_orders).post(create_order))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        // fallback to SPA
        .fallback_service(spa);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    info!("API listening on http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

async fn health() -> &'static str {
    "ok"
}

async fn signup(Json(_req): Json<AuthSignupRequest>) -> Json<AuthTokenResponse> {
    // TODO: persist user + hash password; mTLS / PQC later
    Json(AuthTokenResponse {
        token: "demo-signup-token".into(),
    })
}

async fn login(Json(_req): Json<AuthLoginRequest>) -> Json<AuthTokenResponse> {
    // TODO: verify password; issue JWT
    Json(AuthTokenResponse {
        token: "demo-login-token".into(),
    })
}

async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<Json<CreateOrderResponse>, (StatusCode, String)> {
    state
        .infra
        .create_order(req)
        .await
        .map(Json)
        .map_err(internal_err)
}

async fn list_orders(
    State(state): State<AppState>,
) -> Result<Json<Vec<OrderSummary>>, (StatusCode, String)> {
    state
        .infra
        .get_orders()
        .await
        .map(Json)
        .map_err(internal_err)
}

async fn list_packages(
    State(state): State<AppState>,
) -> Result<Json<Vec<ai::Package>>, (StatusCode, String)> {
    state
        .infra
        .get_packages()
        .await
        .map(Json)
        .map_err(internal_err)
}

async fn get_package_by_sku(
    State(state): State<AppState>,
    Path(sku): Path<String>,
) -> Result<Json<Package>, (StatusCode, String)> {
    match state.infra.get_package_by_sku(&sku).await {
        Ok(Some(package)) => Ok(Json(package)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Package not found".to_string())),
        Err(e) => Err(internal_err(e)),
    }
}

fn internal_err(e: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
