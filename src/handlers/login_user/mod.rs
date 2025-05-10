use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::login_user::LoginInput;
use crate::models::login_user::User;



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
        Ok(Some(_user)) => {
            println!("✅ Login successful for {}", input.email);
            HttpResponse::Ok().body("Login successful")
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
