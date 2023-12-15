use ensemble::{
    types::{DateTime, Uuid},
    Model,
};

#[derive(Debug, Model, macros::Rocket)]
pub struct User {
    #[model(uuid)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
