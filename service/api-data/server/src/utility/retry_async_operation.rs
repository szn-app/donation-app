use std::{error::Error, future::Future, time::Duration};
use tokio::time::sleep;
use tonic::transport::Channel;

/// A generic retry function for async operations.
pub async fn retry_async_operation<F, Fut, T, E>(
    mut action: F,
    max_retries: usize,
    mut delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    for attempt in 0..=max_retries {
        println!("Attempt {}...", attempt + 1);
        match action().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                eprintln!("Error: {}", e);
                if attempt < max_retries {
                    println!("Retrying in {:?}...", delay);
                    sleep(delay).await;
                    delay *= 2;
                } else {
                    return Err(e);
                }
            }
        }
    }
    unreachable!("Loop should return before reaching here");
}
