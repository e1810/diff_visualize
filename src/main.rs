use askama::Template;
use actix_web::{App, HttpServer, get, post, web, ResponseError, HttpResponse};
use thiserror::Error;
use serde::Deserialize;
use actix_files as fs;

mod find_diff;

#[derive(Error, Debug)]
enum MyError {
	#[error("Failed to render HTML")]
	AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
	s: String,
	t: String,
	edit_distance: i32,
	diff: String,
}

#[get("/")]
async fn index_get() -> Result<HttpResponse, MyError> {
	let response_body = IndexTemplate{
		s: "".to_string(),
		t: "".to_string(),
		edit_distance: 0,
		diff: "".to_string(),
	}.render()?;
	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(response_body)
	)
}


#[derive(Deserialize)]
struct Submission {
	text1: String,
	text2: String,
}

#[post("/")]
async fn index_post(params: web::Form<Submission>) -> Result<HttpResponse, MyError> {
	let (di, ds) = find_diff::edit_distance(params.text1.clone(), params.text2.clone());
	let response_body = IndexTemplate{
		s: params.text1.clone(),
		t: params.text2.clone(),
		edit_distance: di,
		diff: ds,
	}.render()?;
	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(response_body)
	)
}


#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
	HttpServer::new(move || {
		App::new()
			.service(fs::Files::new("/static", "static/").show_files_listing())
			.service(index_get)
			.service(index_post)
	})
	.bind("0.0.0.0:8888")?
	.run()
	.await?;
	Ok(())
}
