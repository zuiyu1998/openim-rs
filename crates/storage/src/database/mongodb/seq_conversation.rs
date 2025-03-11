use crate::database::seq_conversation::SeqConversationDataBase;
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
impl SeqConversationDataBase for SeqConversationMongodb {
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

        if let Some(docu) = self.coll.find_one(query).projection(query_result).await? {
            if let Some(raw) = docu.get("max_seq") {
                return Ok(raw.as_i64().unwrap());
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }
}
