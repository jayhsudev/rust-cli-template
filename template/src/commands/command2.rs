use miette::Result;
use tokio::time::{sleep, Duration};
use tokio_graceful_shutdown::SubsystemHandle;
use tracing::info;

pub async fn run(subsys: SubsystemHandle, arg: Option<String>) -> Result<()> {
    info!("command2 started.");
    tokio::select! {
        _ = subsys.on_shutdown_requested() => {
            info!("countdown cancelled.");
        },
        _ = countdown(arg) => {
            info!("countdown finished.");
        }
    };
    info!("command2 stopped");

    Ok(())
}

async fn countdown(arg: Option<String>) {
    if let Some(value) = arg {
        println!("arg {}", value);
    } else {
        println!("arg is None");
    }
    for i in (1..10).rev() {
        info!("command2 countdown: {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}
