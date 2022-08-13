use axum::{
    async_trait,
    body::{self, BoxBody, Bytes, Full},
    extract::{Extension, FromRequest, RequestParts},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post, MethodRouter},
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

use std::sync::Arc;

use user::{UserContainer, UserRepo};

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

    // let both = Users::find().find_with_related(Orders).all(&db).await?;
    // print!("{:#?}", both);

    // let user_repo = Arc::new(UserRepo {});

    let user_repo = user::UserRepo::new(db.clone());
    let user_container = Arc::new(user::UserContainer::new(user_repo));
    //--------------------------

    // let helloRoute = route("/v1", get(|| async { "Hello, world" }));
    // let helloUnderworld = route("/v1/under", get(|| async { "Hello, underworld" }));
    let info_router = user::info();
    let auth_router = user::auth();
    let user_router = Router::new().merge(auth_router).merge(info_router);

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
