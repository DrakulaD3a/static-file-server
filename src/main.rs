use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

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

    let url_path = http_request[0].split(' ').nth(1).unwrap();
    let paths = std::fs::read_dir(format!(".{url_path}")).unwrap();

    let mut files_list = String::new();

    for path in paths {
        let file_type = path.as_ref().unwrap().file_type().unwrap();
        let file_name = path.unwrap().file_name();
        let file = file_name.to_str().unwrap();

        files_list = format!("{files_list}<li><a href=\"{file}\">{file}</a></li>");
    }

    let status_line = "HTTP/1.1 200 OK";

    let body = format!("<h1>File Server</h1><ul>{files_list}</ul>");

    let response = format!(
        "{status_line}

<!DOCTYPE html>
<html>
<head>
    <title>File Server</title>
</head>
<body>
    {body}
</body>
</html>"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
