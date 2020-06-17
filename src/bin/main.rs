use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use dava_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}


fn get_content(request: String) -> String
{
        println!("{}",request);
        let contents = fs::read_to_string("hello.html").unwrap();
        return contents
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    
    let get = b"GET";
    
    
    if buffer.starts_with(get)
    {
       
        
        let str_req = String::from_utf8_lossy(&buffer[4..]).split("\r\n").collect::<Vec<&str>>()[0].to_string();
        
        let file_name = &str_req[0..(str_req.len() - 8)];
        
        let contents = get_content(file_name.to_string());
        
        let response = format!("HTTP/1.1 200 OK\r\n\r\n {}",contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush();
    }

}