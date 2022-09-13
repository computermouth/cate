use fastly::http::StatusCode;
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {
	
	println!("ok");
	
	match Request::get("https://gist.githubusercontent.com/computermouth/7b5460b0c78b3ad018ff29e97733c2df/raw")
		.send("gist")
	{
		Ok(res) => Ok(res),
		Err(_) => Ok(Response::from_status(StatusCode::INTERNAL_SERVER_ERROR))
	}
	
    //~ Ok(Response::from_status(StatusCode::OK))
}
