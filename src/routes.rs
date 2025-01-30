use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router
};

use crate::{
  handler::{
      health_check_handler,
      create_user_handler,
      // user_list_handler,
      get_user_handler,
      delete_user_handler
  },
  AppState
};

pub fn create_router(app_state:Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health_check",get(health_check_handler))
        .route("/api/users",post(create_user_handler))
        // .route("/api/users",get(user_list_handler))
        .route(
            "/api/users/{id}",
            get(get_user_handler)
                .delete(delete_user_handler)
        )
        .with_state(app_state)
}