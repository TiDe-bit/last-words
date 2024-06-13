use anyhow::{Error, Ok};
use bollard::{container::LogsOptions, Docker};
use core::{panic, time::Duration};
use futures::stream::StreamExt;
use std::{
    env,
    ops::{Add, Deref},
};
use tokio::time;

mod undertaker;

const DAY: Duration = Duration::from_secs(86400);

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);

    let docker = Docker::connect_with_socket_defaults().unwrap();

    println!("last words for docker container {}", args[1]);

    let mut container_name = String::from("");
    let maybe_container_name =
        crate::undertaker::get_full_container_name(args[1].clone(), &docker).await;

    if maybe_container_name.is_err() {
        panic!(
            "couldn't actually resolve container name from docker. {:#?}",
            maybe_container_name.err()
        );
    } else {
        container_name = maybe_container_name.ok().unwrap();
    }

    println!("resolved container name {:#?}", container_name);

    if container_name.starts_with("/") {
        container_name.remove(0);
    }

    let log_stream = docker.logs(
        &container_name,
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

    let _ = time::sleep(Duration::from_millis(10));
    let mut stream = log_stream;
    while let Some(item) = stream.next().await {
        print!("{}", item.unwrap())
    }

    Ok(())
}
