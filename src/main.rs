#[macro_use] extern crate rocket;
mod dad_bot;


#[get("/")]
fn index() -> String {
	String::from("Hello, world!")
}

#[get("/dad_bot?<message>&<name>")]
fn dad_bot_api(message: &str, name:&str) -> String {
	return dad_bot::dad_bot(message,name);
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index,dad_bot_api])
}