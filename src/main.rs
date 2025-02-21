use bollard::container::{InspectContainerOptions, StatsOptions,MemoryStatsStats};
use bollard::Docker;
use futures::StreamExt;
use std::process;
use tokio;
use docker_monitor::send_email;
use log4rs;
use log::{info,warn};

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let container_name = "mysql";
    let docker = Docker::connect_with_local_defaults().unwrap_or_else(|error| {
        println!("Failed to connect to docker: {}", error);
        process::exit(1)
    });

    let inspect = docker
        .inspect_container(container_name, None::<InspectContainerOptions>)
        .await
        .unwrap_or_else(|error| {
            println!("Failed to inspect container {}: {}", container_name, error);
            process::exit(1)
        });
    let shm_size_total = inspect.host_config.unwrap().shm_size.unwrap() / (1024^2) ; //单位 MB

    info!("shm_size_total: {} MB", shm_size_total);

    let mut stats_stream = docker.stats(container_name, None::<StatsOptions>);

    while let Some(result) = stats_stream.next().await {
        match result {
            Ok(stats) => {
                match stats.memory_stats.stats.unwrap() {
                    MemoryStatsStats::V1(_) => {}
                    MemoryStatsStats::V2(result) => {
                        let shm_usage = result.shmem / (1024 ^ 2);
                        info!{"shm_usage:{} MB", shm_usage};
                        let shm_usage_percent = shm_usage as f32 / shm_size_total as f32;
                        info!("shm_usage_percent: {}%\n", shm_usage_percent*100.0);
                        if shm_usage_percent > 0.9 {
                            warn!("shm_usage_percent:{}%", shm_usage_percent*100.0);
                            let _ = send_email();
                            break
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error fetching stats: {:?}", e);
            }
        }
    }
}
