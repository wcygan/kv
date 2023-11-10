use schemas::kv::data_service_server::DataService;
use schemas::kv::{DeleteRequest, GetRequest, GetResponse, PutRequest};
use tonic::{Request, Response, Status};

pub struct DataServiceImpl;

#[tonic::async_trait]
impl DataService for DataServiceImpl {
    async fn put(&self, request: Request<PutRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        todo!()
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<()>, Status> {
        todo!()
    }
}
