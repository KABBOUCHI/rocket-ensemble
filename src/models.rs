use ensemble::{
    types::{DateTime, Uuid},
    Model,
};
use rocket::{request::FromParam, response::Responder, Response, http::ContentType};
use rocket::serde::json::Json;

#[derive(Debug, Model)]
pub struct User {
    #[model(uuid)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl<'r> FromParam<'r> for User {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let id = param.parse::<Uuid>().map_err(|_| "Model Not Found")?;

        rocket::tokio::task::block_in_place(move || {
            rocket::tokio::runtime::Handle::current()
                .block_on(async move { User::find(id).await.map_err(|_| "Model Not Found") })
        })
    }
}

impl<'r> Responder<'r, 'static> for User {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(request)
    }
}
