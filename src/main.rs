use actix_route_rate_limiter::{LimiterBuilder, RateLimiter};
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{web::Data, App, HttpServer};
use chrono::Duration;
use log::LevelFilter;
use std::sync::Arc;

use config::Config;
use mailing::relay_message;

pub mod config;
pub mod form;
pub mod mailing;

/// Start the Actix web server and bind it to the listen_address provided in Config.toml.
/// Rate limiting middleware is used to limit individual IP addresses to 4 requests per day.
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let config = Config::load_from_file("Config.toml")
        .expect("Failed to load configuration file Config.toml");
    let config_arc = Arc::new(config.clone());

    let limiter = LimiterBuilder::new()
        .with_duration(Duration::days(1))
        .with_num_requests(4)
        .build();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config_arc.clone()))
            .wrap(Logger::default())
            .wrap(RateLimiter::new(Arc::clone(&limiter)))
            .wrap(DefaultHeaders::new().add(("X-Content-Type-Options", "nosniff")))
            .wrap(DefaultHeaders::new().add(("X-Robots-Tag", "noindex, nofollow")))
            .service(relay_message)
    })
    .bind(&config.listen_address)?
    .run()
    .await
}
