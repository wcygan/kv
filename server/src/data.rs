use std::sync::Arc;
use schemas::kv::data_service_server::DataService;
use schemas::kv::{DeleteRequest, GetRequest, GetResponse, PutRequest};
use tonic::{Code, Request, Response, Status};
use wal::Storage;

pub struct DataServiceImpl {
    storage: Arc<Storage>
}

impl DataServiceImpl {
    pub fn new(storage: Arc<Storage>) -> Self {
        Self { storage }
    }
}

#[tonic::async_trait]
impl DataService for DataServiceImpl {
    async fn put(&self, request: Request<PutRequest>) -> Result<Response<()>, Status> {
        let PutRequest { key, value }  = request.into_inner();
        let result = self.storage.put(key, value).await;

        match result {
            Ok(_) => Ok(Response::new(())),
            Err(_) => Err(Status::new(Code::Internal, "Failed to put value")),
        }
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let GetRequest { key } = request.into_inner();
        let result = self.storage.get(key).await;

        match result {
            Ok(Some(value)) => Ok(Response::new(GetResponse { value })),
            Ok(None) => {Err(Status::new(Code::Internal, "Failed to get value"))},
            Err(_) => Err(Status::new(Code::Internal, "Failed to get value")),
        }
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<()>, Status> {
        let DeleteRequest { key } = request.into_inner();
        let result = self.storage.delete(key).await;

        match result {
            Ok(_) => Ok(Response::new(())),
            Err(_) => Err(Status::new(Code::Internal, "Failed to delete value")),
        }
    }
}
