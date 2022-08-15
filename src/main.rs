use anyhow::{Error, Result};
use axum::Router;
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use pkg::db::ORM;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sea_orm::{DatabaseConnection, DbErr};

use std::sync::Arc;

use user::router::new as new_user_router;

//migrate run migrate
async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(&db, None).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_consume_body_in_extractor_or_middleware=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    //------- db connect ----------
    let mysql = pkg::db::Mysql::new().await?;
    let db = mysql.get_db().await;
    migrate(db).await?;

    let mysql = Arc::new(mysql);

    //----- user -----------
    let user_router = new_user_router(mysql); // v1/user

    //--------------------------

    let main_router = Router::new().merge(user_router);
    //--------------------------

    let app = Router::new().nest("/api", main_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::debug!("listening on {}", addr);
    println!("web listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
