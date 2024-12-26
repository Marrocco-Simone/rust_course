use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buffer_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let header = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let content_lenght = contents.len();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",header, content_lenght, contents);

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream_result in listener.incoming() {
        let stream = stream_result.unwrap();

        handle_connection(stream);
    }
}
