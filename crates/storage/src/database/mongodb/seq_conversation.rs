use crate::database::seq_conversation::SeqConversationRepo;
use abi::{
    async_trait::async_trait,
    mongodb::{
        bson::{doc, Document},
        options::ReturnDocument,
        Collection, Database, IndexModel,
    },
    ErrorKind, Result,
};

pub struct SeqConversationMongodb {
    coll: Collection<Document>,
}

impl SeqConversationMongodb {
    pub async fn new(database: &Database, name: &str) -> Result<Self> {
        let coll = database.collection(name);

        let index_model = IndexModel::builder()
            .keys(doc! {"conversation_id":1})
            .build();

        coll.create_index(index_model).await?;

        Ok(Self { coll })
    }

    pub async fn set_seq(&self, conversation_id: &str, seq: i64, field: &str) -> Result<()> {
        let query = doc! {
        "conversation_id": conversation_id,
        };

        let mut insert = doc! {
        "conversation_id": conversation_id,
        "min_seq":         0,
        "max_seq":         0,
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
impl SeqConversationRepo for SeqConversationMongodb {
    async fn malloc(&self, conversation_id: &str, size: i64) -> Result<i64> {
        if size < 0 {
            return Err(ErrorKind::SizeIsSmall.into());
        }

        if size == 0 {
            return self.get_max_seq(conversation_id).await;
        }

        let query = doc! {
        "conversation_id": conversation_id,
        };

        let query_result = doc! {
            "_id": 0,
            "max_seq": 1
        };

        let update = doc! {
            "$inc": {
                "max_seq": size,
            },
            "$set": {
                "min_seq": 0
            },
        };

        if let Some(docu) = self
            .coll
            .find_one_and_update(query, update)
            .upsert(true)
            .return_document(ReturnDocument::After)
            .projection(query_result)
            .await?
        {
            if let Some(raw) = docu.get("max_seq") {
                let last_seq = raw.as_i64().unwrap();
                Ok(last_seq - size)
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }

    async fn get_max_seq(&self, conversation_id: &str) -> Result<i64> {
        let query = doc! {
        "conversation_id": conversation_id,
        };

        let query_result = doc! {
            "_id": 0,
            "max_seq": 1
        };

        let max_seq = self
            .coll
            .find_one(query)
            .projection(query_result)
            .await?
            .and_then(|doc| doc.get("max_seq").and_then(|bson| bson.as_i64()))
            .unwrap_or_default();

        Ok(max_seq)
    }
}

#[cfg(test)]
mod test {
    use abi::tokio;

    #[tokio::test]
    async fn test_seq_conversation_monodb() {
        use super::SeqConversationMongodb;
        use crate::database::{
            mongodb::{new_mongo_database, MongodbConfig},
            seq_conversation::SeqConversationRepo,
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
        let seq = 10;

        let database = new_mongo_database(&config).await.unwrap();

        let seq_conversation =
            SeqConversationMongodb::new(&database, &config.seq_conversation_name)
                .await
                .unwrap();

        seq_conversation
            .malloc(&conversation_id, seq)
            .await
            .unwrap();

        let value = seq_conversation
            .get_max_seq(&conversation_id)
            .await
            .unwrap();

        assert_eq!(value, 10);
    }
}
