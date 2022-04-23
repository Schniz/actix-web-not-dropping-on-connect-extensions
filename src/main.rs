use actix_web::{get, HttpRequest, Responder};

#[get("/")]
async fn index(request: HttpRequest) -> impl Responder {
    format!("{:?}", request.conn_data::<Droppable>())
}

#[derive(Debug)]
struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    {
        println!("Creating a Droppable and immediately dropping it as an example..");
        // example
        let _droppable = Droppable;
    };

    actix_web::HttpServer::new(|| actix_web::App::new().service(index))
        .on_connect(|_conn, storage| {
            println!("new incoming connection");
            let droppable = Droppable;
            dbg!(&droppable);
            storage.insert(droppable);
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;
    Ok(())
}
