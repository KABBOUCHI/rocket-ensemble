#[macro_use]
extern crate rocket;

use ensemble::Model;
use models::Post;
use rocket::{
    form::Form,
    response::{Flash, Redirect}, request::FlashMessage,
};
use rocket_dyn_templates::{context, Template};

mod migrations;
mod models;

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/posts")
}

#[get("/posts")]
async fn list_posts(flash: Option<FlashMessage<'_>>) -> Template {
    let posts = Post::all()
        .await
        .expect("Failed to get all posts from the database.");

    dbg!(&flash);

    Template::render(
        "posts/index",
        context! {
            posts,
            flash,
        },
    )
}

#[get("/posts/<post>")]
async fn show_post(post: Post) -> Template {
    Template::render(
        "posts/show",
        context! {
            post
        },
    )
}

#[derive(Debug, FromForm)]
struct PostForm<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

#[post("/posts", data = "<post>")]
async fn create_post(post: Form<PostForm<'_>>) -> Result<Redirect, Flash<Redirect>> {
    if post.title.is_empty() || post.content.is_empty() {
        return Err(Flash::error(
            Redirect::to("/posts"),
            "Title and content cannot be empty.",
        ));
    };

    Post::create(Post {
        title: post.title.to_string(),
        content: post.content.to_string(),
        ..Default::default()
    })
    .await
    .expect("Failed to create post.");

    Ok(Redirect::to("/posts"))
}

#[delete("/posts/<post>")]
async fn delete_post(post: Post) -> Redirect {
    post.delete()
        .await
        .expect("Failed to delete post from the database.");

    Redirect::to("/posts")
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    ensemble::setup(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .expect("Failed to set up database pool.");

    ensemble::migrate!(migrations::CreatePostsTable)
        .await
        .expect("Failed to run migrations.");

    let template_fairing = Template::custom(|engines| {
        let _reg = &mut engines.tera;
    });

    rocket::build().attach(template_fairing).mount(
        "/",
        routes![index, list_posts, show_post, create_post, delete_post],
    )
}
