use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn get_path(http_one: &str) -> Result<String, ()> {
    let heads: Vec<&str> = http_one.split_whitespace().collect();
    if heads.len() < 2 {
        return Err(());
    }
    Ok(heads[1].to_string())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let Some(Ok(request_line)) = buf_reader.lines().next() else {
        let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
        return;
    };

    let path = get_path(&request_line);

    let (status_line, contents) = match path.as_deref() {
        Ok("/") => (
            "HTTP/1.1 200 OK",
            fs::read_to_string("static/hello.html").unwrap(),
        ),
        Ok("/sleep") => {
            thread::sleep(Duration::from_secs(5));
            (
                "HTTP/1.1 200 OK",
                fs::read_to_string("static/hello.html").unwrap(),
            )
        }
        Ok(_) => (
            "HTTP/1.1 404 NOT FOUND",
            fs::read_to_string("static/404.html").unwrap(),
        ),
        Err(_) => ("HTTP/1.1 500 Internal Server Error", String::new()),
    };

    if let Ok(path) = &path {
        println!("path: {}\nstatus: {}", path, status_line);
    }

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
