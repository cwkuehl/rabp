mod api;
mod base;
mod extractors;
mod middlewares;
mod types;

use base::functions;
use dotenv::dotenv;
use std::{fs::File, io::BufReader};
// use actix_files::Files;
use actix_web::{
    //    http::header::ContentType,
    //     http::header::HeaderValue,
    //    middleware, web,
    App,
    //     HttpRequest, HttpResponse,
    HttpServer,
};
use log::info;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

// /// simple handle
// async fn index(req: HttpRequest) -> HttpResponse {
//     debug!("{req:?}");

//     HttpResponse::Ok().content_type(ContentType::html()).body(
//         "<!DOCTYPE html><html><body>\
//             <p>Welcome to your TLS-secured homepage!</p>\
//         </body></html>",
//     )
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = types::Config::default();
    let serverconfig = load_rustls_config(&config);
    let info = format!(
        "Starting rabp api webserver Rust Actix Budget Program at http{}://localhost:{}",
        functions::iif(serverconfig.is_some(), "s", ""),
        config.port
    );
    info!("{}", info);
    let auth0_config = extractors::Auth0Config::default();
    let s = HttpServer::new(move || {
        App::new()
            .app_data(auth0_config.clone())
            .wrap(middlewares::cors(&config.client_origin_url))
            .wrap(middlewares::err_handlers())
            .wrap(middlewares::security_headers())
            .wrap(middlewares::logger())
            .service(api::routes())
    })
    .workers(2);
    if let Some(sc) = serverconfig {
        s.bind_rustls_021(format!("{}:{}", config.host, config.port), sc)?
            .run()
            .await
    } else {
        s.bind((config.host, config.port))?.run().await
    }
}

fn load_rustls_config(conf: &types::Config) -> Option<rustls::ServerConfig> {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    if conf.tls_certs.len() <= 0 || conf.tls_key.len() <= 0 {
        return None;
    }

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(conf.tls_certs.as_str()).unwrap());
    let key_file = &mut BufReader::new(File::open(conf.tls_key.as_str()).unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    Some(config.with_single_cert(cert_chain, keys.remove(0)).unwrap())
}
