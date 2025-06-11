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

impl KetoChannelGroup {
    // NOTE: ASYNC consrtuctor may fail // TODO: switch to connect_lazy with connection validation method
    pub async fn new(
        read: &str,
        write: &str,
    ) -> Result<KetoChannelGroup, Box<dyn Error + Send + Sync>> {
        let read = connect(read).await?;
        let write = connect(write).await?;

        Ok(KetoChannelGroup { read, write })
    }

    /// Creates a mock KetoChannelGroup for testing purposes
    pub fn new_mock() -> KetoChannelGroup {
        // Create an endpoint that will never actually be used
        // but will provide a Channel instance for testing
        let mock_endpoint = "http://localhost:0";

        // Using Channel::from_static which doesn't actually connect until used
        let read_channel = Channel::from_static(mock_endpoint).connect_lazy();

        let write_channel = Channel::from_static(mock_endpoint).connect_lazy();

        KetoChannelGroup {
            read: read_channel,
            write: write_channel,
        }
    }
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
