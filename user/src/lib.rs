mod delivery;
pub mod domain;
mod repository;
pub mod usecase;

pub mod router {
    use crate::{
        delivery::http::handler::{auth, create_user, get_info},
        domain::UserContainer,
        repository::mysql::user_repo::UserRepo,
        usecase::user_ucase::UserUcase,
    };
    use axum::{
        extract::Extension,
        routing::{get, post},
        Router,
    };

    use pkg::db::ORM;
    use std::sync::Arc;

    /**
     * new handler
     */
    pub fn new(orm: Arc<dyn ORM>) -> Router {
        let user_repo = UserRepo::new(orm);
        let user_ucase = UserUcase::new(user_repo);
        let user_container = UserContainer::new(user_ucase);

        let user_router = Router::new()
            .route("/login", post(auth))
            .route("/", get(get_info).post(create_user));

        Router::new()
            .nest("/v1/user", user_router)
            .layer(Extension(user_container))
    }
}
