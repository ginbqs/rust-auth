// use std::sync::Arc;

// use axum::{
//     extract::{Path, Query, State},
//     http::StatusCode,
//     response::IntoResponse,
//     Json
// };

use std::sync::Arc;
use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json
};
use serde_json::json;

// use create::{
//     model::{UserModel, UserModelResponse},
//     schema::{CreateUserSchema,FilterOptions,UpdateUserSchema},
//     AppState
// };
use crate::{
    model::{UserModel, UserModelResponse},
    schema::{CreateUserSchema, FilterOptions, UpdateUserSchema},
    AppState
};
pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}


pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"INSERT INTO users(id,name,username,email) VALUES (?,?,?,?)"#)
        .bind(&id)
        .bind(&body.name)
        .bind(&body.username)
        .bind(&body.email)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "Error",
                "message": "User already exists"
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status":"error","message":format!("{:?}",err)})),
        ));
    }

    let user = sqlx::query_as::<_,UserModel>(r#"select * from users where id = ?"#)
        .bind(&id)
        .fetch_one(&data.db)
        .await
        .map_err(|e|{
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status":"error","message":format!("{:?}",e)})),
                )
        })?;
    let user_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "user":to_user_response(&user)
        })
    });
    Ok(Json(user_response))
}
pub async fn _user_list_handler(
    State(data): State<Arc<AppState>>,
    opts: Option<Query<FilterOptions>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;


    let users = sqlx::query_as!(
        UserModel,
        r#"SELECT * FROM users order by id desc limit ? offset ?"#,
        limit as i32,
        offset as i32
    )
        .fetch_all(&data.db)
        .await
        .map_err(|e|{
            let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }",e),
        });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    // let users = sqlx::query_as!(
    //     UserModel,
    //     r#"SELECT * FROM users ORDER by id desc LIMIT ? OFFSET ?"#,
    //     limit as i32,
    //     offset as i32
    // )
    //     .fetch_all(&data.db)
    //     .await
    //     .map_err(|e| {
    //         let error_response = serde_json::json!({
    //         "status": "error",
    //         "message": format!("Database error: { }", e),
    //     });
    //         (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    //     })?;


    // let users = sqlx::query_as::<_,UserModel>(r#"SELECT * from users order by id desc limit ? offset ?"#)
    //     .bind(limit as i32)
    //     .bind(offset as i32)
    //     .fetch_all(&data.db)
    //     .await
    //     .map_err(|e| {
    //         let error_response = serde_json::json!({
    //             "status" : "error",
    //             "message": format!("Database error : {}",e),
    //         });
    //         (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    //     })?;
    let user_responses = users
        .iter()
        .map(|user| to_user_response(&user))
        .collect::<Vec<UserModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": user_responses.len(),
        "user": user_responses
    });
    Ok(Json(json_response))
}

pub async fn get_user_handler(
    Path(id) :Path<String>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    let query_result = sqlx::query_as!(
        UserModel,
        r#"Select * from users where id = ?"#,
        &id
    )
    .fetch_one(&data.db)
    .await;

    // let query_result = sqlx::query_as!(
    //     UserModel,
    //     r#"SELECT * FROM users WHERE id = ?"#,
    //     &id
    // )
    // .fetch_one(&data.db)
    // .await;
    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({
                "status": "ok",
                "data": serde_json::json!({
                    "user": to_user_response(&user)
                })
            });
            return Ok(Json(user_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("User with Id {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status":"error", "message": format!("{:?}", e)})),
                ));
        }
    };

}
pub async fn delete_user_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    let query_result = sqlx::query!(
        r#"Delete  from users where id = ?"#,
        &id
    )
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            )
        })?;
    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("User with Id {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }
    Ok(StatusCode::OK)
}

pub async fn edit_user_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateUserSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        UserModel,
        r#"Select * from users where id = ?"#,
        &id
    ).fetch_one(&data.db)
        .await;
    let user = match query_result {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("User with ID: {} not found",id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        },
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)})),
                ));
        }
    };
    let is_verified = body.is_verified.unwrap_or(user.is_verified != 0);
    let i8_is_verified = is_verified as i8;

    let update_result =
        sqlx::query(r#"update users set name = ?, username = ?, is_verified = ? where id = ?"#)
        .bind(&body.name.unwrap_or_else(|| user.name))
        .bind(&body.username.unwrap_or_else(|| user.username))
        .bind(i8_is_verified)
        .bind(&id)
        .execute(&data.db)
            .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}",e)})),
                )
        })?;
    if update_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("User with Id {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND,Json(error_response)));
    }
    let updated_user = sqlx::query_as!(
        UserModel,
        r#"select * from users where id  = ? "#,
        &id
    )
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status":"error","message": format!("{:?}", e)})),
                )
        })?;
    let user_response = serde_json::json!({
        "status": "ok",
        "data": serde_json::json!({
            "user": to_user_response(&updated_user)
        })
    });

    Ok(Json(user_response))

}

fn to_user_response(user:&UserModel) -> UserModelResponse{
    UserModelResponse{
        id: user.id.to_owned(),
        name: user.name.to_owned(),
        username: user.username.to_owned(),
        email: user.email.to_owned(),
        is_verified: user.is_verified != 0,
        // token: user.token.to_owned(),
        // token_refresh: user.token_refresh.to_owned(),
        // password: user.password.to_owned(),
        created_at: user.created_at.unwrap(),
        updated_at: user.updated_at.unwrap(),
    }
}