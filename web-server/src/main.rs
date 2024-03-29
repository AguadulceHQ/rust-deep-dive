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

    // status
    let status_line = "HTTP/1.1 200 OK";
    let page = fs::read_to_string("hello.html").unwrap();
    let length = page.len();

    // format macro to add file's contents as the body of the success response
    // to create a valid HTTP response we add the header Content-Length
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{page}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn connection_handler_with_validation(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // we read just the first line of HTTP request
    // so that we don't read the entire request into a vector
    // we use next() to fetch the first item of the iteartor
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // destructure values to a tuple
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let page = fs::read_to_string(filename).unwrap();
    let length = page.len();

    // format macro to add file's contents as the body of the success response
    // to create a valid HTTP response we add the header Content-Length
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{page}");

    stream.write_all(response.as_bytes()).unwrap();
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
