pub async fn get_user_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(UserModel, r#"Select * from users where id = ?"#, &id)
        .fetch_one(&data.db).await;


}