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
use actix_cors::Cors;
use std::env;

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
    share_file::share_file,
    share_folder::share_folder,
    trash_file::move_file_to_trash,
    restore_file::restore_file,
    trash_folder::move_folder_to_trash,
    restore_folder::restore_folder,
    space_checker::remaining_space,
    storage_status::storage_status,
    upload_file_on_folder::upload_folder,
    delete_file_from_folder::del_file_from_folder,
    search_files_and_folders::search_files_and_folders,
};

use utils::init::init_upload_dir;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ✅ Load environment variables from .env
    dotenv().ok();

    // ✅ Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // ✅ Ensure API_KEY is set
    match env::var("API_KEY") {
        Ok(_) => info!("✅ API_KEY found in environment."),
        Err(_) => {
            error!("❌ API_KEY not set in .env file!");
            panic!("API_KEY must be set in .env");
        }
    }

    // ✅ Prepare upload directory
    init_upload_dir(None);
    info!("📁 Upload directory initialized.");

    // ✅ Setup database
    let db_pool = init_db_pool().await;
    info!("📦 Database pool initialized.");

    info!("🚀 Starting server at http://127.0.0.1:8080");

    // ✅ Auth middleware
    let auth = HttpAuthentication::bearer(auth::validator);

    HttpServer::new(move || {
        // ✅ Setup CORS (Allow all origins during development)
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

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
            .service(share_file)
            .service(share_folder)
            .service(move_file_to_trash)
            .service(restore_file)
            .service(move_folder_to_trash)
            .service(restore_folder)
            .service(remaining_space)
            .service(storage_status)
            .service(upload_folder)
            .service(del_file_from_folder)
            .service(search_files_and_folders);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .service(public_scope)
            .service(protected_scope)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
