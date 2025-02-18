use abi::{
    protocol::pb::openim_msg::{msg_server::Msg, SendMsgReq, SendMsgResp},
    tonic::{async_trait, Request, Response, Status},
};

use super::MsgRpcServer;

#[async_trait]
impl Msg for MsgRpcServer {
    async fn send_msg(
        &self,
        request: Request<SendMsgReq>,
    ) -> Result<Response<SendMsgResp>, Status> {
        let req = request.into_inner();

        let resp = self.send_msg(req).await?;
        
        Ok(Response::new(resp))
    }
}
