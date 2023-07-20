use::std::fs;
use::std::net::TcpListener;
use::std::net::TcpStream;
use::std::io::prelude::*;
use::tokio::time::sleep;
use::std::time::Duration;


#[tokio::main]                                // by default its multithreaded.
// #[tokio::main(flavour="currnet_thread")]   // operate on single thread.
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    for request_stream in listner.incoming(){
        let request_stream = request_stream.unwrap();
        tokio::spawn(async move { return_html_resp(request_stream).await});
    }

}

async fn return_html_resp (mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let ideal_req = b"GET / HTTP/1.1\r\n";
    let is_404 = if buffer.starts_with(ideal_req) {false}else{true};
    let (file_path,status,reason) =  if is_404 {
         sleep(Duration::from_secs(10)).await;

        ("404.html",404,"NOT FOUND")
    }else{("index.html",200,"OK")};
    // th(Duration::from_secs(10));
    // thread::sleep(Duration::from_secs(5));
    let file = fs::read_to_string(file_path).unwrap();
    let resp = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",status,reason,file.len(),file);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap()
}
