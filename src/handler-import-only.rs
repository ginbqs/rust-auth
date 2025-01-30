use std::sync::Arc;

use Axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json
};

use serde_json::json;

use create::{
    model::{UserModel, UserModelResponse},
    schema::{CreateUserSchema, FilterOptions, UpdateUserSchema},
    AppState
};

fn to_user_response(user: &UserModel) -> UserModelResponse {
    UserModelResponse{
        id: user.id.to_owned(),
        name: user.name.to_owned(),
        username: user.username.to_owned(),
        email: user.email.to_owned(),
        is_verified: user.is_verified != 0,
        token: user.token.to_owned(),
        token_refresh: user.token_refresh.to_owned(),
        password: user.password.to_owned(),
        created_at: user.created_at.to_owned(),
        updated_at: user.updated_at.to_owned(),
    }
}