use std::net::TcpListener;

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
        println!("Blazingly fast web server folks 🦀");
    }
}
