use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::register_user::RegisterInput;

#[post("s4u/users/auth/register")]
pub async fn register_user(
    db: web::Data<PgPool>,
    input: web::Json<RegisterInput>,
) -> impl Responder {
    println!("🟢 Received register request: name={}, email={}", input.name, input.email);

    // First check if user exists
    let user_exists = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as exists",
        input.email
    )
    .fetch_one(db.get_ref())
    .await;

    match user_exists {
        Ok(record) if record.exists.unwrap_or(false) => {
            println!("❌ User already exists: {}", input.email);
            return HttpResponse::Conflict().json(serde_json::json!({
                "error": "User already exists",
                "email": input.email
            }));
        }
        Err(e) => {
            println!("❌ Error checking user existence: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database error: {}", e)
            }));
        }
        _ => () // Continue if user doesn't exist
    }

    // Proceed with registration if user doesn't exist
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
            HttpResponse::Ok().json(serde_json::json!({
                "message": "User registered successfully",
                "email": input.email
            }))
        }
        Err(e) => {
            println!("❌ Error registering user: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Registration failed: {}", e)
            }))
        }
    }
}