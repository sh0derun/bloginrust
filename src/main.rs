use std::net::{TcpListener};
use std::io::{Write, Read};

fn main() {
    let server_lstnr = TcpListener::bind("0.0.0.0:80").unwrap();
    loop{
        match server_lstnr.accept() {
            Ok((mut client, address))=>{
                println!("new client :{}",address);
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

                println!("{}",res);

                let mut req_buf = [0u8; 1024];
                match client.read(&mut req_buf) {
                    Ok(_) => println!("Request\n{}", String::from_utf8_lossy(&req_buf)),
                    Err(e) => println!("Failed receiving request: {}", e),
                }
                
                match client.write(res.as_bytes()) {
                    Ok(_) => println!("Response sent"),
                    Err(e) => println!("Failed sending response: {}", e),
                }

            },
            Err(e)=>{
                println!("couldn't establish connection with the client : {}", e);
            }
        }
    }
    println!("server died !");
}