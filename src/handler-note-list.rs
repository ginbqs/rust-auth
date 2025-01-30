pub async fn user_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>
){
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let page = (opts.page.unwrap_or(1) - 1) * limit;


    let users = sqlx::query_as!(
        UserModel,
        r#"SELECT * FROM users order by id desc limit ? offset ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map(|e|{
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }",e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_responses = users
    .iter()
    .map(|user| to_user_response(&user))
    .collect<Vec<UserModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": user_responses.len(),
        "user": user_responses 
    });
    Ok(Json(json_response))
}
