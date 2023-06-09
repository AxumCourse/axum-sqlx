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
        .route("/add", get(handler::add_ui).post(handler::add))
        .route("/edit/:id", get(handler::edit_ui).post(handler::edit))
        .route("/del/:id", get(handler::del))
        .route("/real_del/:id", get(handler::real_del))
        .route("/tran", get(handler::tran_ui).post(handler::tran))
        .route("/select_in", get(handler::select_in))
        .layer(Extension(Arc::new(AppState {
            pool: Arc::new(pool),
        })));

    tracing::info!("服务器运行于: {}", &cfg.web.addr);

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
