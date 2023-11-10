use std::hash::Hash;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tonic::Status;
use schemas::kv::data_service_client::DataServiceClient;

pub struct Client {
    client: DataServiceClient<tonic::transport::Channel>,
}

impl Client {
    pub async fn new(addr: String) -> Result<Self, tonic::transport::Error> {
        let client = DataServiceClient::connect(addr).await?;
        Ok(Self { client })
    }


    pub async fn get<K, V>(&mut self, key: K) -> Result<Option<V>, Status>
        where
            K: Clone + Serialize + Send + Sync + 'static,
            V: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        let key = bincode::serialize(&key)
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
        let request = schemas::kv::GetRequest { key };
        let response = self.client.get(request).await?;
        let value = response.into_inner().value;
        if value.is_empty() {
            Ok(None)
        } else {
            let v = bincode::deserialize(&value)
                .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
            Ok(Some(v))
        }
    }

    pub async fn set<K, V>(&mut self, key: K, value: V) -> Result<(), Status>
        where
            K: Clone + Serialize + Send + Sync + 'static,
            V: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        let key = bincode::serialize(&key)
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
        let value = bincode::serialize(&value)
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
        let request = schemas::kv::PutRequest { key, value };
        self.client.put(request).await?;
        Ok(())
    }
}