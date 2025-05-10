use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::register_user::RegisterInput;


#[post("s4u/user/register")]
pub async fn register_user(
    db: web::Data<PgPool>,
    input: web::Json<RegisterInput>,
) -> impl Responder {
    println!("🟢 Received register request: name={}, email={}", input.name, input.email);

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
        input.name,
        input.email,
        input.password
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => {
            println!("✅ User registered successfully: {}", input.email);
            HttpResponse::Ok().body("User registered successfully")
        }
        Err(e) => {
            println!("❌ Error registering user: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}