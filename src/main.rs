mod handlers;
mod models;
mod utils;
mod database;

use actix_web::{App, HttpServer, web};
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
};

use utils::init::init_upload_dir;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    init_upload_dir();

    let db_pool = init_db_pool().await;

    println!("🚀 Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(upload)
            .service(list_files)
            .service(download_file)
            .service(delete_file)
            .service(register_user)
            .service(login_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
