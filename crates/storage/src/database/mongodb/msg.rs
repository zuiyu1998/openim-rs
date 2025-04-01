use abi::mongodb::bson::doc;
use abi::mongodb::results::UpdateResult;
use abi::mongodb::{Collection, Database};
use abi::{async_trait::async_trait, Result};
use entity::msg::{MsgDataModel, MsgDocModel};

use crate::database::msg::MsgRepo;

pub struct MsgRepoMongodb {
    coll: Collection<MsgDocModel>,
}

impl MsgRepoMongodb {
    pub fn new(database: &Database) -> Self {
        let coll = database.collection(MsgDocModel::table_name());

        MsgRepoMongodb { coll }
    }
}

#[async_trait]
impl MsgRepo for MsgRepoMongodb {
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
