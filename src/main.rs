#[macro_use]
extern crate lazy_static;

use r2d2::{Pool, PooledConnection};
use tide::{Request, Response};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    //    env_logger::init();
    let mut app = tide::new();
    app.at("/").get(hello);
    app.at("/redis").get(redis);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn hello(_req: Request<()>) -> tide::Result {
    //    Ok(Response::new(http_types::StatusCode::Ok).body_string("Tide".to_string()))
    Ok(Response::from("Tide"))
}

async fn redis(_req: Request<()>) -> std::result::Result<tide::Response, http_types::Error> {
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
