mod api;
mod base;
mod extractors;
mod middlewares;
mod types;

use actix_web::{web, App, HttpServer};
use basis::functions;
use dotenv::dotenv;
use log::{info, LevelFilter};
// use rustls_pki_types::pem::PemObject;
// use rustls_pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use std::{fs::File, io::BufReader, sync::Mutex};
//use r2d2_sqlite::SqliteConnectionManager;
use rustls::{Certificate, PrivateKey, server::ServerConfig};
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
    let cert_chain = load_certificates_from_pem(conf.tls_certs.as_str()).unwrap();
    let keys = load_private_key_from_file(conf.tls_key.as_str()).unwrap();

    Some(config.with_single_cert(cert_chain, keys).unwrap())

    // rustls 0.23.31
    // let key = PrivatePkcs8KeyDer::from_pem_file(conf.tls_key.as_str()).unwrap();
    // let certs: Vec<_> = CertificateDer::pem_file_iter(conf.tls_certs.as_str())
    //         .unwrap().filter_map(|c| c.ok()).collect();
    // Some(config.with_single_cert(certs, PrivateKeyDer::Pkcs8(key)).unwrap())
}

/// Load private key with rustls-pemfile
fn load_private_key_from_file(path: &str) -> Result<PrivateKey, Box<dyn std::error::Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut keys: Vec<_> = pkcs8_private_keys(&mut reader)
        .filter_map(|c| c.ok()).collect();
    match keys.len() {
        0 => Err(format!("No PKCS8-encoded private key found in {path}").into()),
        1 => Ok(PrivateKey(keys.remove(0).secret_pkcs8_der().iter().map(|x| *x).collect())),
        _ => Err(format!("More than one PKCS8-encoded private key found in {path}").into()),
    }
}

/// Load certificates with rustls-pemfile
fn load_certificates_from_pem(path: &str) -> std::io::Result<Vec<Certificate>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs: Vec<_> = certs(&mut reader).filter_map(|c| c.ok()).collect();
    Ok(certs.iter().map(|a| Certificate((*a).to_vec())).collect())
    //Ok(certs.into_iter().map(Certificate).collect())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
