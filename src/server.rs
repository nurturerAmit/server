pub struct Server {
    address:String,
    port:i32
}
impl Server {
    pub fn new(address:String,port:i32)-> Self{
        Self {
            address,
            port
        }
    }
}


enum HTTPMethods {
    GET,
    POST,
    DELETE,
    PUT,
    CONNECT,
    HEAD,
    TRACE,
    PATCH
}