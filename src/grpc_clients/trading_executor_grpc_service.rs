use std::{sync::Arc, time::Duration};

use my_grpc_extensions::{GrpcChannel, GrpcClientSettings};
use tonic::transport::Channel;

use crate::trading_executor::{
    trading_executor_grpc_service_client::TradingExecutorGrpcServiceClient,
    TradingExecutorActivePositionGrpcModel, TradingExecutorGetActivePositionsGrpcRequest, TradingExecutorOpenPositionGrpcRequest, TradingExecutorOpenPositionGrpcResponse, TradingExecutorClosePositionGrpcRequest, TradingExecutorClosePositionGrpcResponse,
};

struct TradingExecutorSettingsGrpcUrl(String);

impl TradingExecutorSettingsGrpcUrl {
    pub fn new(url: String) -> Self {
        Self(url)
    }
}

#[tonic::async_trait]
impl GrpcClientSettings for TradingExecutorSettingsGrpcUrl {
    async fn get_grpc_url(&self, _: &'static str) -> String {
        self.0.clone()
    }
}

pub struct TradingExecutorGrpcClient {
    channel: GrpcChannel,
    timeout: Duration,
}

impl TradingExecutorGrpcClient {
    pub async fn new(grpc_address: String) -> Self {
        Self {
            channel: GrpcChannel::new(
                Arc::new(TradingExecutorSettingsGrpcUrl::new(grpc_address)),
                "trading_executor_grpc_service",
                Duration::from_secs(10),
            ),
            timeout: Duration::from_secs(2),
        }
    }

    async fn create_grpc_service(&self) -> TradingExecutorGrpcServiceClient<Channel> {
        return TradingExecutorGrpcServiceClient::new(self.channel.get_channel().await.unwrap());
    }

    pub async fn open_position(&self, request: TradingExecutorOpenPositionGrpcRequest) -> TradingExecutorOpenPositionGrpcResponse{
        let mut grpc_client = self.create_grpc_service().await;

        return grpc_client.open_position(request).await.unwrap().into_inner();
    }

    pub async fn close_position(&self, request: TradingExecutorClosePositionGrpcRequest) -> TradingExecutorClosePositionGrpcResponse{
        let mut grpc_client = self.create_grpc_service().await;

        return grpc_client.close_position(request).await.unwrap().into_inner();
    }

    pub async fn get_active_positions(
        &self,
        request: TradingExecutorGetActivePositionsGrpcRequest,
    ) -> Vec<TradingExecutorActivePositionGrpcModel> {
        let mut grpc_client = self.create_grpc_service().await;

        let response = grpc_client
            .get_account_active_positions(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();

        let accounts = my_grpc_extensions::read_grpc_stream::as_vec(response, self.timeout)
            .await
            .unwrap();

        if let Some(accounts) = accounts {
            return accounts;
        }

        return vec![];
    }
}
