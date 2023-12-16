use ensemble::{
    types::{DateTime, Uuid},
    Model,
};

#[derive(Debug, Model, macros::Rocket)]
pub struct Post {
    #[model(uuid)]
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
