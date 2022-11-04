use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "enr")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub node_id: Vec<u8>,
    pub sequence_number: i32,
    pub raw: Vec<u8>,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Node,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Node => Entity::belongs_to(super::node::Entity)
                .from(Column::NodeId)
                .to(super::node::Column::Id)
                .into(),
        }
    }
}

impl Related<super::node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Node.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}