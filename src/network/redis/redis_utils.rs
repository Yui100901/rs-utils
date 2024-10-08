use redis::{Client, Commands, Connection, RedisResult};
use std::time::Duration;



lazy_static::lazy_static! {
    static ref GLOBAL_REDIS_CLIENT: Client = new_redis_client(Config::new());
}

fn new_redis_client(config: Config) -> Client {
    let client = Client::open(config.redis.url.as_str()).expect("Invalid Redis URL");
    let mut con = client.get_connection().expect("Failed to connect to Redis");

    // 设置连接选项
    let _: () = redis::cmd("CONFIG")
        .arg("SET")
        .arg("timeout")
        .arg(config.redis.dial_timeout.to_string())
        .query(&mut con)
        .expect("Failed to set dial timeout");

    let _: () = redis::cmd("CONFIG")
        .arg("SET")
        .arg("read_timeout")
        .arg(config.redis.read_timeout.to_string())
        .query(&mut con)
        .expect("Failed to set read timeout");

    let _: () = redis::cmd("CONFIG")
        .arg("SET")
        .arg("write_timeout")
        .arg(config.redis.write_timeout.to_string())
        .query(&mut con)
        .expect("Failed to set write timeout");

    client
}
