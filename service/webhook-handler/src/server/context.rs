use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaResult;
use rdkafka::producer::FutureProducer;
use std::time::Duration;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct AppConfig {
    pub api_data_endpoint: String,
    pub kafka_message_queue_endpoint: String,
    pub kafka_sasl_username: String,
    pub kafka_sasl_password: String,
}

pub fn create_kafka_producer(
    endpoint: &str,
    username: &str,
    password: &str,
) -> KafkaResult<FutureProducer> {
    let mut kafka_config = ClientConfig::new();

    // https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md
    kafka_config.set("bootstrap.servers", endpoint);
    // kafka_config
    //     .set("bootstrap.servers", endpoint)
    //     .set("security.protocol", "sasl_plaintext")
    //     .set("sasl.mechanism", "SCRAM-SHA-512") // Matches your JAAS config
    //     .set("sasl.username", username)
    //     .set("sasl.password", password);

    kafka_config.create()
}

pub async fn create_grpc_channel(
    address: &str,
) -> Result<Channel, Box<dyn std::error::Error + Send + Sync>> {
    let mut attempts = 0;
    let max_retries: u32 = 100;
    let mut delay = Duration::from_millis(300);

    while attempts <= max_retries {
        println!(
            "Attempting to connect to gRPC channel at: {} (Attempt {})",
            address,
            attempts + 1
        );
        match Channel::from_shared(address.to_string()) {
            Ok(builder) => match builder.connect().await {
                Ok(channel) => return Ok(channel),
                Err(e) => {
                    eprintln!("Error connecting to channel: {}", e);
                    if attempts < max_retries {
                        println!("Retrying in {:?}", delay);
                        tokio::time::sleep(delay).await;
                        delay *= 2; // Simple exponential backoff
                    } else {
                        return Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error creating channel: {}", e);
                if attempts < max_retries {
                    println!("Retrying in {:?}", delay);
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Simple exponential backoff
                } else {
                    return Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>);
                }
            }
        }
        attempts += 1;
    }

    Err("Max retries exceeded without successful connection".into())
}
