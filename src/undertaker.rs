use anyhow::{Error, Ok, Result};
use std::time::Duration;
use tokio::time::{self};

use bollard::container::ListContainersOptions;
use bollard::Docker;
#[cfg(unix)]

pub(crate) async fn get_full_container_name(
    partial_container_name: String,
    docker: &Docker,
) -> Result<String, Error> {
    use std::collections::hash_map;

    loop {
        timeout().await;
        let containers = docker
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                // ToDo: filter running
                filters: hash_map::HashMap::new(),
                limit: None,
                size: false,
            }))
            .await?;

        for container in containers {
            for container_name in container.names.iter() {
                if container_name
                    .first()
                    .unwrap()
                    .contains(&partial_container_name)
                {
                    return Ok((*container_name).concat());
                }
            }
        }
    }

    async fn timeout() {
        let ten_millis = Duration::from_millis(10);
        time::sleep(ten_millis).await;
    }
}
