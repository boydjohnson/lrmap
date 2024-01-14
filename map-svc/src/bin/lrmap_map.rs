use map_svc::{get_router, RedisClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_url = std::env::var("REDIS")?;

    let redis = redis::Client::open(redis_url)?;

    let con = redis.get_async_connection().await?;

    let app = get_router().with_state(RedisClient::new(con));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
