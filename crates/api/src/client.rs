use grpc::collection_client::CollectionClient;
use grpc::scheduler_client::SchedulerClient;
use serde::{Deserialize, Serialize};
use tonic::transport::{Channel, Endpoint};

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub server_address: String,
}

impl ClientConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            server_address: self.server_address, // always use our own address
        }
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            // TODO mature and figure a better default port
            server_address: "http://[::1]:42069".into(),
        }
    }
}
pub struct Client {
    pub collection: CollectionClient<Channel>,
    pub scheduler: SchedulerClient<Channel>,
}

impl Client {
    async fn connect(config: ClientConfig) -> Result<Client, Box<dyn std::error::Error>> {
        let config = config.merge(Default::default());
        let server_address = Endpoint::from_shared(config.server_address)?;

        let collection = CollectionClient::connect(server_address.clone()).await?;
        let scheduler = SchedulerClient::connect(server_address).await?;

        Ok(Client {
            collection,
            scheduler,
        })
    }
}
