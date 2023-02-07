use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use server_rust2::ThreadPool;

fn main() {
    println!("started");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(10) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}
fn handle_connection(mut s: TcpStream) {
    let buf_reader = BufReader::new(&mut s);
    let http_req: Vec<String> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let req_line = &http_req[0];
    println!("{:?}", req_line);

    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /apple HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "apple.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    s.write_all(response.as_bytes()).unwrap();

    //生成一个请求参数文件a.txt
    let mut all_req = OpenOptions::new().append(true).open("a.txt").unwrap();
    all_req.write_all(req_line.as_bytes()).unwrap();
    all_req.write_all("\r\n".as_bytes()).unwrap();
}
