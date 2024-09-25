/// Finaly project:
/// Plan:
/// 1. Learn about TCP and HTTP.
/// 2. List for TCP connections on a socket.
/// 3. Parse a small number of HTTP request.
/// 4. Creat a proper HTTP response.
/// 5. Improve the throughput of our server with a thread pool.
/// 
/// 2024-09-09: Stop at end of Section 20.1; next time, start Section 20.2.
/// 2024-09-10: Start Section 20.2
/// 2024-09-10: Start at "A Worker Struct Responsible..."
/// 2024-09-25: Scan Section 20.3, "Graceful Shutdown", then pivot to spend
/// time on fundamental learnings.

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use final_project_chapter_20::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let pool = ThreadPool::new(4);

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
