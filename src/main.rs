#[macro_use]
extern crate lazy_static;

use r2d2::{Pool, PooledConnection};
use sqlx::prelude::*;
use tide::{Request, Response};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    //    env_logger::init();
    let db = DB::new().await;
    let mut app = tide::with_state(db);
    app.at("/").get(hello);
    app.at("/redis").get(redis);
    app.at("/my").get(my);
    app.at("/pg").get(pg);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn hello(_req: Request<DB>) -> tide::Result {
    //    Ok(Response::new(http_types::StatusCode::Ok).body_string("Tide".to_string()))
    Ok(Response::from("Tide"))
}

async fn redis(_req: Request<DB>) -> std::result::Result<tide::Response, http_types::Error> {
    use r2d2_redis::redis::Commands;
    let mut conn = get_redis_connection();
    match conn.get::<&str, String>("key") {
        Ok(s) => Ok(Response::from(s)),
        Err(e) => std::result::Result::Err(http_types::Error::new(
            http_types::StatusCode::InternalServerError,
            e,
        )),
    }
}

async fn my(req: Request<DB>) -> std::result::Result<tide::Response, http_types::Error> {
    match sqlx::query!(r###"SELECT version() v"###)
        .fetch_one(&req.state().my_pool)
        .await
        .map(|rec| rec.v)
    {
        Ok(s) => Ok(Response::from(s)),
        Err(e) => std::result::Result::Err(http_types::Error::new(
            http_types::StatusCode::InternalServerError,
            e,
        )),
    }
}

async fn pg(req: Request<DB>) -> std::result::Result<tide::Response, http_types::Error> {
    use sqlx_core::postgres::PgCursor;
    let mut cursor: PgCursor = sqlx::query(r#"SELECT version() v"#).fetch(&req.state().pg_pool);
    let row = cursor.next().await;
    match row {
        Ok(r) => Ok(Response::from(r.unwrap().get::<&str, &str>("v"))),
        Err(e) => std::result::Result::Err(http_types::Error::new(
            http_types::StatusCode::InternalServerError,
            e,
        )),
    }
}

pub const MAX_POOL_SIZE: u32 = 64;
pub const MIN_POOL_SIZE: u32 = 8;
pub const REDIS_POOL_SIZE: u32 = 100;

lazy_static! {
    pub static ref REDIS_POOL: Pool<r2d2_redis::RedisConnectionManager> = {
        dotenv::dotenv().ok();
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let manager = r2d2_redis::RedisConnectionManager::new(redis_url).unwrap();
        r2d2::Pool::builder()
            .max_size(REDIS_POOL_SIZE)
            .build(manager)
            .expect("Failed to create redis pool.")
    };

    // Used to update core data into redis master, such as person, role and dept etc.
    // pub static ref MASTER_REDIS_POOL: Pool<r2d2_redis::RedisConnectionManager> = {
    //     dotenv().ok();
    //     let redis_url = env::var("MASTER_REDIS_URL").expect("MASTER_REDIS_URL must be set");
    //     let manager = r2d2_redis::RedisConnectionManager::new(redis_url).unwrap();
    //     r2d2::Pool::builder()
    //         .max_size(REDIS_POOL_SIZE)
    //         .build(manager)
    //         .expect("Failed to create master redis pool.")
    // };

}

pub fn get_redis_connection() -> PooledConnection<r2d2_redis::RedisConnectionManager> {
    REDIS_POOL.clone().get().unwrap()
}

pub struct DB {
    my_pool: sqlx::MySqlPool,
    pg_pool: sqlx::PgPool,
}

impl DB {
    async fn new() -> DB {
        let url = std::env::var("MYSQL_URL").expect("MYSQL_URL must be set");
        let max_pool_size: u32 = std::env::var("MYSQL_MAX_POOL_SIZE")
            .unwrap_or_else(|_| MAX_POOL_SIZE.to_string())
            .parse::<u32>()
            .unwrap_or(MAX_POOL_SIZE);
        let min_pool_size: u32 = std::env::var("MYSQL_MIN_POOL_SIZE")
            .unwrap_or_else(|_| MIN_POOL_SIZE.to_string())
            .parse::<u32>()
            .unwrap_or(MIN_POOL_SIZE);

        let my_pool: sqlx::MySqlPool = sqlx::Pool::builder()
            .max_size(max_pool_size)
            .min_size(min_pool_size)
            .build(&url)
            .await
            .unwrap();
        let url = std::env::var("PG_URL").expect("PG_URL must be set");

        let pg_pool: sqlx::PgPool = sqlx::Pool::builder()
            .max_size(max_pool_size)
            .min_size(min_pool_size)
            .build(&url)
            .await
            .unwrap();
        DB { my_pool, pg_pool }
    }
}
