use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;                                        
use std::fs;

fn main() {
    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader= BufReader::new(&mut stream);
    let request_line = buf_reader
    .lines()
    .next()
    .unwrap()
    .unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 Ok", "hello.html"),
        "GET /sleep HTTP/1.1" => { 
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 Ok", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
        
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}
    "
    );


    stream.write_all(response.as_bytes()).unwrap();
    // let http_request: Vec<_> = buf_reader
    // .lines()
    // .map(|result| result.unwrap())
    // .take_while(|line | !line.is_empty())
    // .collect();

    

}