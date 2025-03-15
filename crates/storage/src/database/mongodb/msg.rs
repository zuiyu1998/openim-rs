use abi::mongodb::{Collection, Database};
use abi::{async_trait::async_trait, Result};
use entity::msg::MsgDocModel;

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
}
