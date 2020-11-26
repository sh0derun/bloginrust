use std::net::{TcpListener};
use std::io::{Write, Read};

struct Request{
    method: String,
    resource: String,
    http_version: String
}

impl Request{
    fn new()-> Request{
        Request{
            method: String::new(),
            resource: String::new(),
            http_version: String::new()
        }
    }
}

fn parse_request(request: String)->Request{
    let headers:Vec<&str> = request.split("\n").collect();
    let request_parts:Vec<&str> = headers[0].split(" ").collect();
    return Request{method      :request_parts[0].to_string(), 
                   resource    :request_parts[1].to_string(), 
                   http_version:request_parts[2].to_string()};
}

fn handle_request(request: Request){
    match request.method.as_ref() {
        "GET" => println!("this is a GET method !"),
        "POST" => println!("unimplemented POST method !"),
        "PUT" => println!("unimplemented PUT method !"),
        "DELETE" => println!("unimplemented DELETE method !"),
        "CONNECT" => println!("unimplemented CONNECT method !"),
        "TRACE" => println!("unimplemented TRACE method !"),
        "HEAD" => println!("unimplemented HEAD method !"),
        _ => println!("out of methods !"),
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
                        let request: Request = parse_request(String::from_utf8_lossy(&req_buf).to_string());
                        handle_request(request);
                    },
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
