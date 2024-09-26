use actix_web::{ web, HttpResponse };
use sqlx::PgPool;
use chrono::{ DateTime, Utc };
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use tracing::Instrument;



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
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscribe_email = %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database."
    );
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
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!(
                "Failed to execute query: {:?}", e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
