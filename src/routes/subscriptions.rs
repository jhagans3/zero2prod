use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgConnection;
use std::ops::Deref;
use std::sync::Arc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Let's start simple: we always return a 200 OK
// There is a bit of cerenomy here to get our hands on a &PgConnection.
// web::Data<Arc<PgConnection>> is equivalent to Arc<Arc<PgConnection>>
// Therefore connection.get_ref() returns a &Arc<PgConnection>
// which we can then deref to a &PgConnection.
// We could have avoided the double Arc wrapping using .app_data()
// instead of .data() in src/startup.rs
pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<Arc<PgConnection>>,
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref().deref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}
