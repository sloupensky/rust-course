use prometheus::{Counter, Gauge, Registry};
use thiserror::Error;

const MESSAGES_SENT_NAME: &str = "messages_sent_counter";
const MESSAGES_SENT_DESC: &str = "Counter tracking active clients";
const ACTIVE_CLIENTS_NAME: &str = "active_client_gauge";
const ACTIVE_CLIENTS_DESC: &str = "Counter tracking active clients";

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Metric creation failed `{0}`")]
    MetricCreationFailed(String),
    #[error("Metric creation failed `{0}`")]
    MetricRegistrationFailed(String),

}

pub struct AppMetrics {
    pub sent_messages: Counter,
    pub active_clients: Gauge,
}

impl AppMetrics {
    pub fn initialize(registry: &Registry) -> Self {
        let sent_messages = Counter::new(MESSAGES_SENT_NAME, MESSAGES_SENT_DESC)
            .expect("Failed to create sent messages counter");
        let active_clients = Gauge::new(ACTIVE_CLIENTS_NAME, ACTIVE_CLIENTS_DESC)
            .expect("Failed to create active client counter");

        registry.register(Box::new(sent_messages.clone())).expect("Failed to register metrics for sent messages");
        registry.register(Box::new(active_clients.clone())).expect("Failed to register metrics for active clients");

        Self {
            sent_messages,
            active_clients
        }
    }
}
