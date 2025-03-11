use crate::{
    cache::{msg::MsgCache, redis::consts::get_msg_cache_key},
    model::MsgInfoModel,
};

use abi::{
    async_trait::async_trait,
    redis::{self, AsyncCommands},
    serde_json, Result,
};

const MSG_CACHE_TIMEOUT: i64 = 86400;

pub struct MsgCacheRedis {
    client: redis::Client,
}

impl MsgCacheRedis {
    pub fn new(client: redis::Client) -> Self {
        MsgCacheRedis { client }
    }
}

#[async_trait]
impl MsgCache for MsgCacheRedis {
    async fn set_message_by_seqs(
        &self,
        conversation_id: &str,
        msgs: Vec<MsgInfoModel>,
    ) -> Result<()> {
        for msg in msgs.into_iter() {
            let msg_str = serde_json::to_string(&msg)?;
            let mut conn = self.client.get_multiplexed_async_connection().await?;

            let () = conn
                .hset(
                    get_msg_cache_key(conversation_id, msg.msg.seq),
                    "value",
                    msg_str,
                )
                .await?;

            let () = conn
                .expire(
                    get_msg_cache_key(conversation_id, msg.msg.seq),
                    MSG_CACHE_TIMEOUT,
                )
                .await?;
        }

        Ok(())
    }
}
