use axum::{extract::Extension, Router};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use std::env;
use std::net::SocketAddr;
use std::result::Result;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sea_orm::{Database, DatabaseConnection, DbErr};

use std::sync::Arc;

use user::handler::new as new_user_handler;

//migrate run migrate
async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(&db, None).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_consume_body_in_extractor_or_middleware=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    //------- db connect ----------
    let db_url = env::var("DATABASE_URL").unwrap();
    let db = Database::connect(&db_url).await?;
    migrate(&db).await?;

    //----- user -----------
    let user_repo = user::UserRepo::new(db.clone());
    let user_ucase = user::usecase::UserUcase::new(user_repo);
    let user_container = Arc::new(user::UserContainer::new(user_ucase));

    //--------------------------

    let user_router = new_user_handler();

    let main_router = Router::new()
        .nest("/v1/user", user_router)
        .layer(Extension(user_container));

    let app = Router::new().nest("/api", main_router);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
