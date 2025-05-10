use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::login_user::{LoginInput, User};
use crate::utils::jwt::create_jwt;
use serde_json::json;

#[post("s4u/user/auth/login")]
pub async fn login_user(
    db: web::Data<PgPool>,
    input: web::Json<LoginInput>,
) -> impl Responder {
    println!("🔐 Login attempt for email: {}", input.email);

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1 AND password = $2",
        input.email,
        input.password
    )
    .fetch_optional(db.get_ref())
    .await;

    match user {
        Ok(Some(user)) => {
            println!("✅ Login successful for {}", input.email);

            // Generate JWT token
           match create_jwt(&user.email) {
                Ok(token) => HttpResponse::Ok().json(json!({
                    "message": "Login successful",
                    "token": token
                })),
                Err(err) => {
                    println!("❌ JWT creation failed: {}", err);
                    HttpResponse::InternalServerError().body("Token generation failed")
                }
            }
        }
        Ok(None) => {
            println!("⚠️ Invalid credentials for {}", input.email);
            HttpResponse::Unauthorized().body("Invalid email or password")
        }
        Err(e) => {
            println!("❌ Database error during login: {}", e);
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}
