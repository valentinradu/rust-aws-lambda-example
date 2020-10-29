use lambda_http::{
    start,
    Handler,
    Request,
    Response,
    IntoResponse,
    Body
};
use lambda_runtime::{
    Context,
    error::HandlerError
};

struct Article {}

impl Handler<Response<Body>> for Article {
    fn run(&mut self, _: Request, _: Context) -> 
        Result<Response<Body>, HandlerError> {
        Ok("Article".into_response())
    }
}

#[tokio::main]
async fn main() -> Result<(), HandlerError> {
    let handler = Article {};
    start(handler, None);
    Ok(())
}