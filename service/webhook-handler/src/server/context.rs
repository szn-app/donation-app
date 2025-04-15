use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaResult;
use rdkafka::producer::FutureProducer;

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
    kafka_config
        .set("bootstrap.servers", endpoint)
        .set("security.protocol", "sasl_plaintext")
        .set("sasl.mechanism", "SCRAM-SHA-512") // Matches your JAAS config
        .set("sasl.username", username)
        .set("sasl.password", password);

    kafka_config.create()
}

pub async fn create_grpc_channel(
    address: &str,
) -> Result<Channel, Box<dyn std::error::Error + Send + Sync>> {
    let channel = Channel::from_shared(address.to_string())?.connect().await?;
    Ok(channel)
}
