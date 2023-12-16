use ensemble::migrations::{Error, Migration, Schema};

#[derive(Debug, Default)]
pub struct CreatePostsTable;

#[ensemble::async_trait]
impl Migration for CreatePostsTable {
    async fn up(&self) -> Result<(), Error> {
        Schema::create("posts", |table| {
            table.uuid();
            table.string("title");
            table.text("content");
            table.timestamps();
        })
        .await
    }

    async fn down(&self) -> Result<(), Error> {
        Schema::drop("posts").await
    }
}
