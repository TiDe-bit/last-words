use anyhow::{Error, Ok, Result};
use std::time::Duration;
use tokio::time::{self};

use bollard::container::ListContainersOptions;
use bollard::Docker;
#[cfg(unix)]

pub(crate) async fn get_full_container_name(
    partial_container_name: String,
) -> Result<String, Error> {
    use std::collections::hash_map;

    let docker = Docker::connect_with_socket_defaults()?;

    dbg!("searching for {}", partial_container_name.clone());

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

        dbg!("containers: {:?}", containers.clone().len());
        for container in containers {
            dbg!("checking container: {:?}", container.names.clone());
            for container_name in container.names.iter() {
                dbg!(
                    "container name: {:?}, searched name: {:?}",
                    container_name,
                    partial_container_name.clone()
                );
                dbg!(
                    "found: {}",
                    container_name
                        .clone()
                        .first()
                        .unwrap()
                        .contains(&partial_container_name)
                );

                if container_name
                    .first()
                    .unwrap()
                    .contains(&partial_container_name)
                {
                    dbg!("found container: {:?}", container_name.clone());
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

