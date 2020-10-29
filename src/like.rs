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

struct Like {}

impl Handler<Response<Body>> for Like {
    fn run(&mut self, _: Request, _: Context) -> 
        Result<Response<Body>, HandlerError> {
        Ok("Like".into_response())
    }
}

#[tokio::main]
async fn main() -> Result<(), HandlerError> {
    let handler = Like {};
    start(handler, None);
    Ok(())
}