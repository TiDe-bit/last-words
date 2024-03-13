use std::{time::Duration};

use anyhow::{Error, Ok, Result};
use tokio::time::{self};

pub(crate) async fn get_full_container_name(
    partial_container_name: String,
    tries: usize,
) -> Result<String, Error> {
    let docker = docker_api::Docker::new("tcp://127.0.0.1:80")?;

    loop {
        if tries > 5000 {
            break;
        }
        let containers = docker.containers().list(&Default::default()).await?;
        for container in containers {
        let found_container_vec = container
            .names
            .as_slice()
            .iter()
            .find(
                   |&container_name| container_name.starts_with(&[partial_container_name.to_owned()])
                );
        return Ok(found_container_vec.unwrap().concat());
    }
    let ten_millis = Duration::from_millis(10);

    time::sleep(ten_millis);
    }

    panic!("could not find container with name {}", partial_container_name)
}
