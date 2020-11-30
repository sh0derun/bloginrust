use std::io::{Read, Write};
use std::net::TcpListener;

struct Method;
impl Method{
    const GET: &'static str = "GET";
    const POST: &'static str = "POST";
    const PUT: &'static str = "PUT";
    const DELETE: &'static str = "DELETE";
    const HEAD: &'static str = "HEAD";
    const CONNECT: &'static str = "CONNECT";
    const TRACE: &'static str = "TRACE";
}

struct StatusCode;
impl StatusCode{
    const OK: (u16, &'static str)                    = (200, "OK");
    const NOT_FOUND: (u16, &'static str)             = (404, "Not Found");
    const NOT_ALLOWED: (u16, &'static str)           = (405, "Method Not Allowed");
    const INTERNAL_SERVER_ERROR: (u16, &'static str) = (500, "Internal Server Error");
}

#[derive(Debug)]
struct Request {
    method: String,
    resource: String,
    http_version: String,
}

impl Request {
    fn new() -> Request {
        Request {
            method: String::new(),
            resource: String::new(),
            http_version: String::new(),
        }
    }

    fn parse_request(request: String) -> Request {
        let headers: Vec<&str> = request.split("\n").collect();
        let request_parts: Vec<&str> = headers[0].split(" ").collect();
        return Request {
            method: request_parts[0].to_string(),
            resource: request_parts[1].to_string(),
            http_version: request_parts[2].to_string(),
        };
    }

    fn handle_request(&self) {
        //TODO: better way ?
        match self.method.as_ref() {
            Method::GET => {
                println!("this is a GET method !");
                println!("{:?}", self);
            }
            Method::POST => println!("unimplemented POST method !"),
            Method::PUT => println!("unimplemented PUT method !"),
            Method::DELETE => println!("unimplemented DELETE method !"),
            Method::CONNECT => println!("unimplemented CONNECT method !"),
            Method::TRACE => println!("unimplemented TRACE method !"),
            Method::HEAD => println!("unimplemented HEAD method !"),
            _ => println!("out of methods !"),
        }
    }

}

#[derive(Debug)]
struct Response{
    http_version: String,
    status_code: u16,
    status_name: String,
    content_type: String,
    content: String
}

impl Response{
    fn new() {
        //TODO
    }
    fn build_response(){
        //TODO
    }
}

fn main() {
    let server_lstnr = TcpListener::bind("0.0.0.0:80").unwrap();
    loop {
        match server_lstnr.accept() {
            Ok((mut client, address)) => {
                println!("new client :{}", address);
                let res = "HTTP/1.1 200 OK
                                Content-Type: text/html\n
                                <html>
                                    <head>
                                        <title>hello world 5</title>
                                    </head>
                                    <body>
                                        <script>for(let i = 0; i < 5; ++i){document.writeln('<h1>hello world!</h1>');}</script>
                                    </body>
                                </html>";

                let mut req_buf = [0u8; 1024 * 4];
                match client.read(&mut req_buf) {
                    Ok(_) => {
                        let request: Request = Request::parse_request(String::from_utf8_lossy(&req_buf).to_string());
                        request.handle_request();
                    }
                    Err(e) => println!("Failed receiving request: {}", e),
                }

                match client.write(res.as_bytes()) {
                    Ok(_) => println!("Response sent"),
                    Err(e) => println!("Failed sending response: {}", e),
                }
            }
            Err(e) => {
                println!("couldn't establish connection with the client : {}", e);
            }
        }
    }
}
