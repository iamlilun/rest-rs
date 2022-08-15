use anyhow::{Error, Result};
use axum::{extract::Extension, Router};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use pkg::db::ORM;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sea_orm::{DatabaseConnection, DbErr};

use std::sync::Arc;

use user::delivery::http::handler::new as new_user_handler;
use user::domain::UserContainer;
use user::repository::mysql::user_repo::UserRepo;
use user::usecase::user_ucase::UserUcase;

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

    //----- user -----------
    let user_repo = UserRepo::new(Arc::new(mysql));
    let user_ucase = UserUcase::new(user_repo);
    let user_container = UserContainer::new(user_ucase);
    let user_router = new_user_handler();
    //--------------------------
    
    
    let main_router = Router::new()
        .nest("/v1/user", user_router)
        .layer(Extension(user_container));

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
