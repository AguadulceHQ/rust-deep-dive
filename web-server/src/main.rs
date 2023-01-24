use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // 7878 is Rust typed on a phone lol
    // bind method returns a new TcpListener instance
    // it's called bind and not new because we generally bind to a port
    // it returns Result<T, E> for now we unwrap e.g. if the port is laready bound
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming method on TcpListener returns an iterator that gives us a sequence of streams of type TcpStream
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // this prints in the terminal althoguh the page shows connection reset as we are not writing data
        // println!("Blazingly fast web server folks 🦀");

        connection_handler(stream);
    }
}

fn connection_handler(mut stream: TcpStream) {
    // wraps a mutable reference to the stream into a BufReader instance
    // BufReader adds buffering by managing calls to std::io::Read trait's methods for us
    let buf_reader = BufReader::new(&mut stream);
    // collect the lines of the request
    // we collect those lines in a vector by using Vec<_> type annotation
    let http_request: Vec<_> = buf_reader
        // returns an iterator of Result<String, std::io::Error> and it splits the stream of data when it sees a newline
        .lines()
        // we map and unwrap each Result to get the actual string
        // we stop the program if we have some invalid data
        .map(|result| result.unwrap())
        // the browser signals the end of an HTTP request by sending two newline chars in a row
        // so we take lines until there is an empty string
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request received: {:#?}", http_request);

    // send back message's data
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // as_bytes converts the string data into bytes
    // write_all on stream takes &[u8] and sends those bytes directly down the connection
    // this could fail, a real app should handle it better
    stream.write_all(response.as_bytes()).unwrap();
}
