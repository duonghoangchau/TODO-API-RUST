use std::net::SocketAddr;

use todo_api::{
    app_state::AppState,
    config::{app::AppConfig, database::DatabaseConfig, env::EnvConfig},
    db::connection::create_pool,
    routes::create_router,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    EnvConfig::validate().expect("Missing required environment variables");

    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env().expect("Failed to load app config");
    let db_config = DatabaseConfig::from_env().expect("Failed to load database config");
    let db_pool = create_pool(&db_config)
        .await
        .expect("Failed to connect to PostgreSQL");
    let app_state = AppState::new(config.clone(), db_pool);
    let app = create_router(app_state);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Invalid address");

    tracing::info!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind server");

    axum::serve(listener, app).await.expect("Server failed");
}
