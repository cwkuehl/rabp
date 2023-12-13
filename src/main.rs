mod api;
mod base;
mod extractors;
mod middlewares;
mod types;

use actix_web::{web, App, HttpServer};
use basis::functions;
use dotenv::dotenv;
use log::{info, LevelFilter};
use std::{fs::File, io::BufReader, sync::Mutex};
//use r2d2_sqlite::SqliteConnectionManager;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::builder()
        .filter_module("rustls", LevelFilter::Warn)
        .init();
    let config = types::Config::default();
    let serverconfig = load_rustls_config(&config);
    let info = format!(
        "Starting rabp api webserver Rust Actix Budget Program at http{}://localhost:{} with sqlite db at {}",
        functions::iif(serverconfig.is_some(), "s", ""),
        config.port, config.sqlite_db
    );
    info!("{}", info);
    let auth0_config = extractors::Auth0Config::default();
    let manager =
        diesel::r2d2::ConnectionManager::<diesel::SqliteConnection>::new(config.sqlite_db.clone());
    let pool = diesel::r2d2::Pool::builder()
        .max_size(3)
        .test_on_check_out(true)
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");
    let undopool = web::Data::new(Mutex::new(base::UndoPool::new()));
    let s = HttpServer::new(move || {
        App::new()
            .app_data(auth0_config.clone())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::clone(&undopool))
            .wrap(middlewares::cors(&config.client_origin_url))
            .wrap(middlewares::err_handlers())
            .wrap(middlewares::security_headers())
            .wrap(middlewares::logger())
            // .wrap(middlewares::cookie_session())
            .service(api::routes())
            .wrap(middlewares::SayHi)
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
