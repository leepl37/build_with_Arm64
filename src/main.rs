#![allow(unused_imports)]
mod function_files;
mod function_play_and_stop;
mod function_playlist;
mod function_status;
mod function_ticker;
mod functions;
mod html_struct;
mod json_struct;
mod render;
mod render_function;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use tera::{Context, Tera};
use log::{error, info, warn, debug, trace};
use log4rs;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
            .data(tera)
            .service(actix_files::Files::new("/assets", ".").show_files_listing())
            //login
            .service(test_test)
            .service(render::login)
            .service(render::login_process)
            .service(render::upload)
            .service(render::delete)
            .service(render::create)
            .service(render::create_post)
            .service(render::play_and_stop_playlist_post)
            .service(render::test)
            .service(render::test2)
            .service(render::edit_playlist_post)
            .service(render::ticker_setting_post)
            .service(render::create_playlist_post)
            .service(render::testtest)
    })
    .bind("0.0.0.0:8383")?
    .run()
    .await
}
