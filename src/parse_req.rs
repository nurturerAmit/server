use std::collections::HashMap;
use::std::net::TcpListener;
use::std::net::TcpStream;
use::std::io::prelude::*;
use url::Url;
use urlencoding::decode;

#[allow(dead_code)]

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    for request_stream in listner.incoming(){
        let request_stream = request_stream.unwrap();
        tokio::spawn(async move { return_json_resp(request_stream).await});
    }

}

async fn return_json_resp (mut stream:TcpStream){
    let hash_query = parse_req(stream);
    println!("{:?}",hash_query);

}

fn parse_req (mut stream:TcpStream)->HashMap<String, String>{
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let full_stream_request = String::from_utf8_lossy(& buffer);
    let mut base_string = String::from("https://baseurl.com");
    let query_params = full_stream_request.split(' ').nth(1).unwrap();
    let decoded = decode(&query_params[1..]).expect("UTF-8");
    base_string.push_str(&decoded);
    let b = base_string.replace('>', "?");
    let parsed_url = Url::parse(&b).unwrap();
    let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
    hash_query
}