use anyhow::{Error, Ok};
use bollard::{container::LogsOptions, Docker};
use core::time::Duration;
use futures::stream::StreamExt;
use std::{
    env,
    io::{stdout, Write},
    ops::Add,
};

mod undertaker;

const DAY: Duration = Duration::from_secs(86400);

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);
    println!("last words for container {:?}", args[1]);
    let docker = Docker::connect_with_socket_defaults();
    let container_name = crate::undertaker::get_full_container_name(args[1].clone())
        .await
        .unwrap();
    let log_stream = docker.unwrap().logs(
        container_name.strip_prefix("/").unwrap(),
        Some(LogsOptions::<String> {
            follow: true,
            stdout: true,
            stderr: true,
            since: 0,
            timestamps: false,
            tail: String::from("all"),
            until: chrono::Utc::now().add(DAY).timestamp(),
        }),
    );
    let mut stream = log_stream;
    while let Some(item) = stream.next().await {
        stdout().write_all(item?.as_ref()).unwrap();
    }
    Ok(())
}
