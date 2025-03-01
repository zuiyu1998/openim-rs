use crate::database::seq_user::SeqUserDataBase;
use abi::{
    async_trait::async_trait,
    mongodb::{
        bson::{doc, Document},
        Collection, Database, IndexModel,
    },
    Result,
};

pub struct SeqUserMongodb {
    coll: Collection<Document>,
}

impl SeqUserMongodb {
    pub async fn new(database: &Database, name: &str) -> Result<Self> {
        let coll = database.collection(name);

        let index_model = IndexModel::builder()
            .keys(doc! {"user_id": 1, "conversation_id":1})
            .build();

        coll.create_index(index_model).await?;

        Ok(Self { coll })
    }

    pub async fn get_seq(&self, conversation_id: &str, user_id: &str) -> Result<i64> {
        let query = doc! {
        "user_id":         user_id,
        "conversation_id": conversation_id,
        };

        if let Some(docu) = self.coll.find_one(query).await? {
            if let Some(raw) = docu.get("read_seq") {
                return Ok(raw.as_i64().unwrap());
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }

    pub async fn set_seq(
        &self,
        conversation_id: &str,
        user_id: &str,
        seq: i64,
        field: &str,
    ) -> Result<()> {
        let query = doc! {
        "user_id":         user_id,
        "conversation_id": conversation_id,
        };

        let mut insert = doc! {
        "user_id":         user_id,
        "conversation_id": conversation_id,
        "min_seq":         0,
        "max_seq":         0,
        "read_seq":        0,
        };

        insert.remove(field);

        let update = doc! {
            "$set": {
                field: seq,
            },
            "$setOnInsert": insert,
        };

        self.coll.update_one(query, update).upsert(true).await?;
        Ok(())
    }
}

#[async_trait]
impl SeqUserDataBase for SeqUserMongodb {
    async fn set_user_read_seq(
        &self,
        conversation_id: &str,
        user_id: &str,
        seq: i64,
    ) -> Result<()> {
        let db_seq = self.get_user_read_seq(conversation_id, user_id).await?;

        if db_seq > seq {
            return Ok(());
        }

        self.set_seq(conversation_id, user_id, seq, "read_seq")
            .await?;
        Ok(())
    }

    async fn get_user_read_seq(&self, conversation_id: &str, user_id: &str) -> Result<i64> {
        self.get_seq(conversation_id, user_id).await
    }
}
