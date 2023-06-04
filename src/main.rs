use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use axum_sqlx::{config::Config, handler, model::state::AppState};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = Config::from_env()
        .map_err(|e| tracing::error!("初始化配置失败：{}", e.to_string()))
        .unwrap();
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(cfg.mysql.maxcons)
        .connect(&cfg.mysql.dsn)
        .await
        .map_err(|e| tracing::error!("数据库连接失败：{}", e.to_string()))
        .unwrap();

    let app = Router::new()
        .route("/", get(handler::index))
        .route("/detail/:id", get(handler::detail))
        .layer(Extension(Arc::new(AppState {
            pool: Arc::new(pool),
        })));

    tracing::info!("服务器运行于: {}", &cfg.web.addr);

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
