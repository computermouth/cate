use fastly::http::StatusCode;
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {
	
	println!("ok");
	
	match Request::get("https://hurricane-horn-tip.glitch.me")
		.send("glitch")
	{
		Ok(res) => Ok(res),
		Err(_) => Ok(Response::from_status(StatusCode::INTERNAL_SERVER_ERROR))
	}
	
    //~ Ok(Response::from_status(StatusCode::OK))
}
