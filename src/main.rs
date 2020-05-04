use tide::{Request, Response};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/").get(hello);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn hello(_req: Request<()>) -> tide::Result {
    //    Ok(Response::new(http_types::StatusCode::Ok).body_string("Tide".to_string()))
    Ok(Response::from("Tide"))
}
