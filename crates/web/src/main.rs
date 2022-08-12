use axum::{
    async_trait,
    body::{self, BoxBody, Bytes, Full},
    extract::{FromRequest, RequestParts},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use std::env;
use std::net::SocketAddr;
use std::result::Result;

use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use entity::{orders, prelude::*, users};
use sea_orm::{
    prelude::*, ActiveValue, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr,
    QueryOrder, Set, Statement,
};

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

    let both = Users::find().find_with_related(Orders).all(&db).await?;
    print!("{:#?}", both);

    //--------------------------
    let app = Router::new().route("/", get(|| async { "Hello, world" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}