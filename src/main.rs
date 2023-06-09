use std::{
    fs::{read_dir, read_to_string},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(e) => panic!("Failed to open TcpListener: {e}"),
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to receive incoming stream: {e}");
                continue;
            }
        };

        handle_connnection(stream);
    }
}

fn handle_connnection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(Result::unwrap)
        .take_while(|line| !line.is_empty())
        .collect();

    let url_path = format!(
        "./{}",
        http_request[0].split(' ').nth(1).map_or_else(
            || {
                eprintln!("Invalid request");
                ""
            },
            |request| request.trim_start_matches('/')
        )
    );
    let path = Path::new(url_path.as_str());

    let response = if path.is_dir() {
        let mut files_list = String::new();

        let paths = match read_dir(path) {
            Ok(paths) => paths,
            Err(e) => {
                eprintln!("Failed to read directory: {e}");
                return;
            }
        };
        for content in paths {
            let file_name = match content {
                Ok(file) => file.file_name(),
                Err(e) => {
                    eprintln!("Failed to read file inside this directory: {e}");
                    continue;
                }
            };
            let file = file_name.to_str().unwrap_or("");

            let link_path = path.as_os_str().to_str().unwrap_or("").trim_start_matches('.').trim_start_matches('/');

            files_list = format!(
                "{files_list}<li><a href=\"/{}{}{file}\">{file}</a></li>",
                path.as_os_str()
                    .to_str()
                    .unwrap_or("")
                    .trim_start_matches('.')
                    .trim_start_matches('/'),
                if link_path.is_empty() { "" } else { "/" },
            );
        }

        format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html><html><head><title>File Server</title></head><body><h1>File Server</h1><ul>{files_list}</ul></body></html>")
    } else {
        match read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                let msg = format!("Failed to read file: {e}");
                eprintln!("{msg}");
                msg
            }
        }
    };

    match stream.write_all(response.as_bytes()) {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to write response: {e}"),
    };
}
