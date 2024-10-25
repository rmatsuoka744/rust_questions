use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let durations: Vec<Duration> = vec![
        Duration::from_secs(1),
        Duration::from_secs(2),
        Duration::from_secs(3),
    ];

    let tasks: Vec<_> = durations
        .into_iter()
        .enumerate()
        .map(|(i, duration)| {
            task::spawn(async move {
                sleep(duration).await;
                println!("This is Task {}", i + 1);
            })
        })
        .collect();

    for task in tasks {
        task.await.unwrap();
    }
}
