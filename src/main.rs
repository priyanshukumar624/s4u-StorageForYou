mod handlers;
mod models;
mod utils;
mod database;
mod auth;

use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use env_logger;
use database::pg_admin4::init_db_pool; 

use handlers::{
    upload_file::upload,
    list_file::list_files,
    download_file::download_file,
    delete_file::delete_file,
    register_user::register_user,
    login_user::login_user,
    create_folder::create_folder,
    delete_folder::delete_folder,       
    rename_folder::rename_folder,
    list_folder::list_folders
};

use utils::init::init_upload_dir;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Verify API_KEY is set
    std::env::var("API_KEY").expect("API_KEY must be set in .env");

    init_upload_dir(None);
    let db_pool = init_db_pool().await;

    println!("🚀 Starting server at http://127.0.0.1:8080");

    // Create auth middleware
    let auth = HttpAuthentication::bearer(auth::validator);

    HttpServer::new(move || {
        // Public routes (no auth)
        let public_scope = web::scope("/public")
            .service(register_user)
            .service(login_user);

        // Protected routes (require API key)
        let protected_scope = web::scope("/protected")
            .wrap(auth.clone())
            .service(upload)
            .service(list_files)
            .service(download_file)
            .service(delete_file)
            .service(create_folder)
            .service(delete_folder)
            .service(rename_folder)
            .service(list_folders);

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(public_scope)
            .service(protected_scope)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}