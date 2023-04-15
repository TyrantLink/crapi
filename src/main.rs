#[macro_use] extern crate rocket;
use rocket::serde::json::{json,Value};

mod dad_bot;


#[get("/")]
fn index() -> String {
	String::from("Hello, world!")
}

#[get("/dad_bot?<message>&<name>")]
fn dad_bot_api(message: &str, name:&str) -> Value {
	let result = dad_bot::dad_bot(message,name);
	json!({"response":result})
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index,dad_bot_api])
}