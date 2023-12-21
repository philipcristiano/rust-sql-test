use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::str;


#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config_file: String,
    #[arg(short, long, value_enum, default_value = "DEBUG")]
    log_level: tracing::Level,
    #[arg(long, action)]
    log_json: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct AppConfig {
    database_url: String,
}


use std::process::ExitCode;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgPool;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub manager_id: i32,
}

async fn  user_by_id(exec: &PgPool, id: i32) -> anyhow::Result<User> {
    Ok(sqlx::query_as!(
        User,
        "select * from users where id = $1;",
        id
    )
    .fetch_one(exec)
    .await?)
}

#[tokio::main]
async fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let args = Args::parse();
    service_conventions::tracing::setup(args.log_level);

    let app_config = read_app_config(args.config_file);

    // Start by making a database connection.
    tracing::info!("connecting to {}", app_config.database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&app_config.database_url).await?;
    let user =  user_by_id(&pool, 1).await?;
    println!("{user:?}");

    Ok(ExitCode::SUCCESS)
}

fn read_app_config(path: String) -> AppConfig {
    let config_file_error_msg = format!("Could not read config file {}", path);
    let config_file_contents = fs::read_to_string(path).expect(&config_file_error_msg);
    let app_config: AppConfig = toml::from_str(&config_file_contents).expect("Problems parsing config file");

    app_config
}
