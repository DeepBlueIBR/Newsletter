use actix_web::{ web, HttpResponse };
use sqlx::PgPool;
use chrono::{ DateTime, Utc };
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;



#[derive(serde::Deserialize)]
pub struct FormData{
    email: String,
    name: String
}


pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let now: DateTime<Utc> = Utc::now();

    // Convert chrono::DateTime<Utc> to time::OffsetDateTime
    let now_offset: OffsetDateTime = OffsetDateTime::from_unix_timestamp(now.timestamp()).unwrap();
    
    match sqlx::query!(
    r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    now_offset
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
