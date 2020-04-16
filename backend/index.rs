use now_lambda::{lambda, error::NowError, IntoResponse, Request, Response};
use http::StatusCode;
use std::error::Error;
use postgres::{Client, NoTls};
 
fn handler(_: Request) -> Result<impl IntoResponse, NowError> {
    let mut client = Client::connect(
        "postgres://xkfojeci:wcl8cQW_74voK_CUlshaDDewwyG8Af46@drona.db.elephantsql.com:5432/xkfojeci", NoTls)
        .expect("ERROR: connecting");

    client.batch_execute("
        CREATE TABLE person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
    ").expect("ERROR: creating table");

    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    ).expect("ERROR: inserting into table");

    for row in client.query("SELECT id, name, data FROM person", &[]).expect("ERROR: selecting from table") {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body("all good!")
        .expect("Internal Server Error");
 
        Ok(response)
}
 
// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}