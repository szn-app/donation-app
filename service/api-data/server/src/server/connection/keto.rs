use crate::utility::retry_async_operation;
use std::error::Error;
use std::time::Duration;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct KetoEndpointConfig {
    pub read: String,
    pub write: String,
}

#[derive(Debug, Clone)]
pub struct KetoChannelGroup {
    pub read: Channel,
    pub write: Channel,
}

pub async fn create_grpc_connection(
    read: &str,
    write: &str,
) -> Result<KetoChannelGroup, Box<dyn Error + Send + Sync>> {
    let read = connect(read).await?;
    let write = connect(write).await?;

    Ok(KetoChannelGroup { read, write })
}

async fn connect(address: &str) -> Result<Channel, Box<dyn Error + Send + Sync>> {
    // Ensure address has the correct scheme
    let address = if address.starts_with("http://") || address.starts_with("https://") {
        address.to_string()
    } else {
        format!("http://{}", address)
    };

    retry_async_operation(
        || {
            let addr = address.clone();
            async move {
                Channel::from_shared(addr.clone())?
                    .connect()
                    .await
                    .map_err(|e| format!("gRPC connect error: {}", e).into())
            }
        },
        10,
        Duration::from_millis(500),
    )
    .await
}
