use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
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

        connection_handler_with_sleep(stream);
    }
}

fn connection_handler_with_sleep(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // we read just the first line of HTTP request
    // so that we don't read the entire request into a vector
    // we use next() to fetch the first item of the iteartor
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // destructure values to a tuple
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /slow HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let page = fs::read_to_string(filename).unwrap();
    let length = page.len();

    // format macro to add file's contents as the body of the success response
    // to create a valid HTTP response we add the header Content-Length
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{page}");

    stream.write_all(response.as_bytes()).unwrap();
}
