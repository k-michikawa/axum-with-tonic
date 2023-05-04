mod for_http {
    use axum::{http::StatusCode, response::IntoResponse, Json};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    pub struct EchoRequest {
        message: String,
    }

    #[derive(Serialize)]
    pub struct EchoResponse {
        message: String,
    }

    pub async fn echo_for_json(Json(request): Json<EchoRequest>) -> impl IntoResponse {
        let message = request.message;
        (
            StatusCode::OK,
            Json(EchoResponse {
                message: format!("Hello, {}!", message),
            }),
        )
    }
}

mod for_grpc {
    mod pb {
        tonic::include_proto!("examples");
    }

    // re-export
    pub use pb::echo_server::EchoServer;

    pub struct GrpcEchoService;

    #[tonic::async_trait]
    impl pb::echo_server::Echo for GrpcEchoService {
        async fn unary_echo(
            &self,
            request: tonic::Request<pb::EchoRequest>,
        ) -> Result<tonic::Response<pb::EchoResponse>, tonic::Status> {
            let message = request.into_inner().message;
            Ok(tonic::Response::new(pb::EchoResponse {
                message: format!("Hello, {}!", message),
            }))
        }
    }
}

use http::header::CONTENT_TYPE;
use std::net::SocketAddr;
use tower::{make::Shared, steer::Steer, ServiceExt};

#[tokio::main]
async fn main() {
    let http = axum::Router::new()
        .route("/echo", axum::routing::post(for_http::echo_for_json))
        .map_err(axum::BoxError::from)
        .boxed_clone();
    let grpc = tonic::transport::Server::builder()
        .add_service(for_grpc::EchoServer::new(for_grpc::GrpcEchoService))
        .into_service()
        .map_response(|r| r.map(axum::body::boxed))
        .boxed_clone();
    let http_grpc = Steer::new(
        vec![http, grpc],
        |req: &http::Request<hyper::Body>, _svcs: &[_]| {
            if req.headers().get(CONTENT_TYPE).map(|v| v.as_bytes()) != Some(b"application/grpc") {
                0
            } else {
                1
            }
        },
    );
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(Shared::new(http_grpc))
        .await
        .expect("サーバーの起動に失敗しました");
}
