use crate::database::seq_user::SeqUserRepo;
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

        let seq = self
            .coll
            .find_one(query)
            .await?
            .and_then(|doc| doc.get("read_seq").and_then(|bson| bson.as_i64()))
            .unwrap_or_default();

        Ok(seq)
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
impl SeqUserRepo for SeqUserMongodb {
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

#[cfg(test)]
mod test {
    use abi::tokio;

    #[tokio::test]
    async fn test_seq_user_monodb() {
        use super::SeqUserMongodb;
        use crate::database::{
            mongodb::{new_mongo_database, MongodbConfig},
            seq_user::SeqUserRepo,
        };

        let config = MongodbConfig {
            host: "127.0.0.1".to_string(),
            port: 27017,
            user: "openIM".to_string(),
            password: "openIM123".to_string(),
            database: "test".to_string(),
            seq_user_name: "seq_user".to_string(),
            seq_conversation_name: "seq_conversation_name".to_string(),
        };

        let conversation_id = "test_conversation".to_string();
        let user_id = "1".to_string();
        let seq = 10;

        let database = new_mongo_database(&config).await.unwrap();

        let seq_user = SeqUserMongodb::new(&database, &config.seq_user_name)
            .await
            .unwrap();

        seq_user
            .set_user_read_seq(&conversation_id, &user_id, seq)
            .await
            .unwrap();

        let value = seq_user.get_seq(&conversation_id, &user_id).await.unwrap();

        assert_eq!(value, 10);
    }
}
