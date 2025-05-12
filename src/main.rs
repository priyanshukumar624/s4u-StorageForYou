mod handlers;
mod models;
mod utils;
mod database;
mod auth;

use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use env_logger::Env;
use log::{info, error};
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
    list_folder::list_folders,
     share_file::share_file,      // ✅ Add this
    share_folder::share_folder,
    trash_file::move_file_to_trash,// ✅ Add this
    restore_file::restore_file,// ✅ Uncomment if you implement this
    trash_folder::move_folder_to_trash,
    restore_folder::restore_folder,
    space_checker::remaining_space,
    storage_status::storage_status,
    upload_file_on_folder::upload_folder,
    delete_file_from_folder::del_file_from_folder
};

use utils::init::init_upload_dir;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ✅ Load environment variables from .env
    dotenv().ok();

    // ✅ Initialize logger with default level fallback
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // ✅ Check API_KEY and log explicitly
    match std::env::var("API_KEY") {
        Ok(_) => info!("✅ API_KEY found in environment."),
        Err(_) => {
            error!("❌ API_KEY not set in .env file!");
            panic!("API_KEY must be set in .env");
        }
    }

    // ✅ Initialize upload directory
    init_upload_dir(None);
    info!("📁 Upload directory initialized.");

    // ✅ Initialize database pool
    let db_pool = init_db_pool().await;
    info!("📦 Database pool initialized.");

    info!("🚀 Starting server at http://127.0.0.1:8080");

    // ✅ Auth middleware
    let auth = HttpAuthentication::bearer(auth::validator);

    HttpServer::new(move || {
        let public_scope = web::scope("/public")
            .service(register_user)
            .service(login_user);

        let protected_scope = web::scope("/protected")
            .wrap(auth.clone())
            .service(upload)
            .service(list_files)
            .service(download_file)
            .service(delete_file)
            .service(create_folder)
            .service(delete_folder)
            .service(rename_folder)
            .service(list_folders)
            .service(share_file)     // ✅ Add this
            .service(share_folder)  
            .service(move_file_to_trash)// ✅ Add this
            .service(restore_file)
            .service(move_folder_to_trash)
            .service(restore_folder)
            .service(remaining_space)
            .service(storage_status)
            .service(upload_folder)
            .service(del_file_from_folder);

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(public_scope)
            .service(protected_scope)
    })
    .bind("192.168.1.8:8080")?
    .run()
    .await
}
