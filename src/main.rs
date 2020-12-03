use std::io::{Read, Write};
use std::net::TcpListener;
use std::fs::File;
use std::fmt::Formatter;

const KBYTE: u16 = 1024;
const MBYTE: u32 = KBYTE as u32*1000;

struct Method;
impl Method{
    const GET: &'static str     = "GET";
    const POST: &'static str    = "POST";
    const PUT: &'static str     = "PUT";
    const DELETE: &'static str  = "DELETE";
    const HEAD: &'static str    = "HEAD";
    const CONNECT: &'static str = "CONNECT";
    const TRACE: &'static str   = "TRACE";
}

#[derive(Debug)]
struct Status;
impl Status{
    const OK: (u16, &'static str)                    = (200, "OK");
    const NOT_FOUND: (u16, &'static str)             = (404, "Not Found");
    const NOT_ALLOWED: (u16, &'static str)           = (405, "Method Not Allowed");
    const INTERNAL_SERVER_ERROR: (u16, &'static str) = (500, "Internal Server Error");
}

#[derive(Debug)]
struct ContentType;
impl ContentType{
    const HTML: &'static str  = "text/html";
    const PLAIN: &'static str = "text/plain";
    const JPEG: &'static str  = "image/jpeg";
    const PNG: &'static str   = "image/png";
    const json: &'static str  = "application/json";
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

    fn handle_request(&self, response: &mut Response) {
        //TODO: better way ?
        match self.method.as_ref() {
            Method::GET => {
                println!("this is a GET method !");
                println!("{:?}", self);
                response.content_type = ContentType::HTML.to_string();
                response.content = Response::load_content("./src/public/index.html");
                response.status = Status::OK;
                response.error = String::new();
            },
            Method::POST => {println!("unimplemented POST method !");}
            Method::PUT => {println!("unimplemented PUT method !");}
            Method::DELETE => {println!("unimplemented DELETE method !");}
            Method::CONNECT => {println!("unimplemented CONNECT method !");}
            Method::TRACE => {println!("unimplemented TRACE method !");}
            Method::HEAD => {println!("unimplemented HEAD method !");}
            _ => {println!("out of methods !");}
        }
    }

}

#[derive(Debug)]
struct Response{
    http_version: String,
    status: (u16, &'static str),
    content_type: String,
    content: String,
    error: String
}

impl Response{
    fn new() {
        //TODO
    }

    fn load_content(file_path: &str) -> String{
        //TODO : we don't want to panic if there is a problem
        let mut file = File::open(file_path).expect(&format!("cannot open file {}",file_path));
        let mut content = String::new();
        file.read_to_string(&mut content).expect("cannot read from the file !");
        return content;
    }
}

impl Default for Response {
    fn default() -> Self {
        Response{
            http_version: "HTTP/1.1".to_string(),
            content_type: ContentType::PLAIN.to_string(),
            content:      String::from("dummy response"),
            status:       Status::OK,
            error:        String::new()
        }
    }
}


impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result{
        write!(f, "{} {} {}\nContent-Type: {}\n{}", self.http_version, self.status.0, self.status.1, self.content_type, self.content)
    }
}

fn main() {
    let server_lstnr = TcpListener::bind("0.0.0.0:80").unwrap();

    loop {
        match server_lstnr.accept() {
            Ok((mut client, address)) => {
                println!("new client :{}", address);

                //TODO : better way to read the request
                let mut req_buf = [0u8; KBYTE as usize * 4];
                let mut response: Response = Default::default();
                
                match client.read(&mut req_buf) {
                    Ok(_) => {
                        println!("{}", String::from_utf8_lossy(&req_buf).to_string());
                        let request: Request = Request::parse_request(String::from_utf8_lossy(&req_buf).to_string());
                        request.handle_request(&mut response);
                    }
                    Err(e) => println!("Failed receiving request: {}", e),
                }

                match client.write(response.content.as_bytes()) {
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
