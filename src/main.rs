use http::Request;
use http::HTTPMethods;

mod server;
use server::Server;




#[tokio::main]           
async fn main() {
    let server: Server = Server::new("127.0.0.1".to_string(),8080);
    // print!("{}",server.address);
}







