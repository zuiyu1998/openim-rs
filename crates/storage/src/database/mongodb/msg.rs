use abi::mongodb::bson::doc;
use abi::mongodb::results::UpdateResult;
use abi::mongodb::{Collection, Database};
use abi::{async_trait::async_trait, Result};
use entity::msg::{MsgDataModel, MsgDocModel};

use crate::database::msg::MsgRepo;

pub struct MsgMongodb {
    coll: Collection<MsgDocModel>,
}

impl MsgMongodb {
    pub fn new(database: &Database) -> Self {
        let coll = database.collection(MsgDocModel::table_name());

        MsgMongodb { coll }
    }
}

#[async_trait]
impl MsgRepo for MsgMongodb {
    async fn create(&self, model: MsgDocModel) -> Result<()> {
        self.coll.insert_many(vec![model]).await?;
        Ok(())
    }

    async fn update_with_msg(
        &self,
        doc_id: &str,
        index: usize,
        value: &MsgDataModel,
    ) -> Result<UpdateResult> {
        let field = format!("msgs.{}", index);
        let query = doc! {"doc_id": doc_id};
        let value = bson::to_bson(&value).unwrap();
        let update = doc! {"$set": {
            field: value
        }};

        let res = self.coll.update_one(query, update).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use abi::tokio;
    use entity::msg::{MsgDataModel, MsgDocModel};

    #[tokio::test]
    async fn test_seq_conversation_monodb() {
        use super::MsgMongodb;
        use crate::database::{
            mongodb::{new_mongo_database, MongodbConfig},
            msg::MsgRepo,
        };
        use fake::{Fake, Faker};

        let config = MongodbConfig {
            host: "127.0.0.1".to_string(),
            port: 27017,
            user: "openIM".to_string(),
            password: "openIM123".to_string(),
            database: "test".to_string(),
            seq_user_name: "seq_user".to_string(),
            seq_conversation_name: "seq_conversation_name".to_string(),
        };

        let database = new_mongo_database(&config).await.unwrap();

        let msg = MsgMongodb::new(&database);

        let doc: MsgDocModel = Faker.fake();

        let doc_id = doc.doc_id.to_string();

        if doc.msgs.is_empty() {
            return;
        }

        let len = doc.msgs.len();

        let msg_data: MsgDataModel = Faker.fake();

        msg.create(doc).await.unwrap();
        msg.update_with_msg(&doc_id, 0, &msg_data).await.unwrap();

        msg.update_with_msg(&doc_id, len + 2, &msg_data).await.unwrap();
    }
}
