use deadpool_lapin::{Manager, Pool, PoolError};
use futures::{join, StreamExt};
use lapin::{options::*, types::FieldTable, ConnectionProperties};
use std::result::Result as StdResult;
use std::time::Duration;
use thiserror::Error as ThisError;
use tokio_amqp::*;

type RMQResult<T> = StdResult<T, PoolError>;
type Result<T> = StdResult<T, Error>;
type Connection = deadpool::managed::Object<Manager>;

#[derive(ThisError, Debug)]
enum Error {
    #[error("rabbitmq error: {0}")]
    RMQError(#[from] lapin::Error),
    #[error("rabbitmq pool error: {0}")]
    RMQPoolError(#[from] PoolError),
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://testrabbit:0passrabbitmq1@127.0.0.1:5678/%2f".into());

    let options = ConnectionProperties::default().with_tokio();
    let max_pool_size = 10;
    let pool = create_pool(addr, max_pool_size, options);
    let _ = join!(rmq_listen(pool.clone()));
    Ok(())
}

fn create_pool(addr: String, max_pool_size: usize, options: ConnectionProperties) -> Pool {
    let manager = Manager::new(addr, options);
    Pool::new(manager, max_pool_size)
}

async fn get_conn(pool: Pool) -> RMQResult<Connection> {
    let connection = pool.get().await?;
    Ok(connection)
}

async fn rmq_listen(pool: Pool) -> Result<()> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        println!("connecting rabbitmq consumer...");
        match consume(pool.clone()).await {
            Ok(_) => println!("rabbitmq listen returned"),
            Err(e) => eprintln!("rabbitmq listen had an error: {}", e),
        };
    }
}

async fn consume(pool: Pool) -> Result<()> {
    let rmq_conn = get_conn(pool).await.map_err(|e| {
        eprintln!("couldn't get rabbitmq connection: {}", e);
        e
    })?;
    let channel = rmq_conn.create_channel().await?;

    let queue = channel
        .queue_declare(
            "hello-queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    println!("Declared queue {:?}", queue);

    let mut consumer = channel
        .basic_consume(
            "hello-queue",
            "hello_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("consumer connected, waiting for messages");
    while let Some(delivery) = consumer.next().await {
        if let Ok((channel, delivery)) = delivery {
            let msg = std::str::from_utf8(&delivery.data).unwrap().to_string();
            println!("received msg: {}", msg);
            channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await?
        }
    }
    Ok(())
}
