mod handlers;
mod models;
mod utils;

use actix_web::{App, HttpServer};
use handlers::{upload::upload, list::list_files, download::download_file, delete_file::delete_file};
use utils::init::init_upload_dir;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    init_upload_dir(); // ensure upload directory exists

    println!("🚀 Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(upload)
            .service(list_files)
            .service(download_file)
            .service(delete_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
