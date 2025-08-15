## relay-mailer 

An Actix Web server leveraging the [lettre](https://docs.rs/lettre/latest/lettre/) crate to receive, validate, and sanitize form field data and relay it to an email server. 

## Setup
It is necessary to provide a `Config.toml` file containing the following: 
- user: a string describing the username for the relay server
- pwd: a string describing the password for the relay server (this will usually be an app password similar to an API key)
- forward_address: a string describing the email address to forward messages to
- server: a string describing the server URL matching the username and pwd
- listen_address: a string describing the address and port the server will listen on

## Example Config.toml file that matches this schema
```toml
user = "bob@mail.com"
pwd = "04 08 0F 10 17 2A"
forward_address = "alice@mail.com"
server = "smtp.mail.com" 
listen_address = "0.0.0.0:8080"
```

## The form data expected by the server is as follows: 
- name: a string describing the contact's name with a minimum length of 1 and a maximum length of 30
- country: a string describing the contact's country with a minimum length of 1 and a maximum length of 30
- email: a string describing the contact's email address that must be a valid email
- message: a string representation of the contact's message with a minimum length of 1
- language: an optional string describing the contact's preferred language

## The server is rate limited to four requests per 24 hours by default
```rust
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let config = Config::load_from_file("Config.toml").expect("Failed to load configuration file Config.toml");
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
```
