use anyhow::{Error, Ok};
use bollard::{container::LogsOptions, Docker};
use clap::Parser;
use colored::Colorize;
use core::{panic, time::Duration};
use futures::stream::StreamExt;
use std::{env, ops::Add};
use tokio::time;

mod undertaker;

const DAY: Duration = Duration::from_secs(86400);

/// Container Logs Necromancer
#[derive(Debug, Parser)]
#[command(name = "last_words")]
#[command(about = "Container-logs necromancer CLI", long_about = None)]
struct Cli {
    #[arg(value_name = "CONTAINER_NAME")]
    container_name: String,

    #[arg(short, long, default_value = "white")]
    color: String,

    #[arg(short, long, default_value = "")]
    stop_string: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();

    if !env::var("DOCKER_HOST").is_ok() {
        println!("Default DOCKER_HOST not set. Setting unix:///var/run/docker.sock");
        env::set_var("DOCKER_HOST", "unix:///var/run/docker.sock");
    }

    let docker = Docker::connect_with_defaults().unwrap();

    println!("last words for docker container {}...", args.container_name);

    let mut container_name = String::from("");
    let maybe_container_name =
        crate::undertaker::get_full_container_name(args.container_name.clone(), &docker).await;

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
        let log_line = item.unwrap().to_string();
        print!("{}", log_line.color(args.color.clone()));
        if args.stop_string.len() > 1 && log_line.contains(args.stop_string.as_str()) {
            println!("read until stop line containing: {}", args.stop_string);
            break;
        }
    }

    Ok(())
}
