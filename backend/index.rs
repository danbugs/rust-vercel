use http::{StatusCode};
use now_lambda::{error::NowError, IntoResponse, Request, Response};
 
fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    println!("{:#?}", req);
    let res = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .body("<p>ok</p>")
        .expect("Internal Server Error");
 
        Ok(res)
}
