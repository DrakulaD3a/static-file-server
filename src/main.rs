use std::{
    fs::read_dir,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
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

    let url_path = format!(".{}", http_request[0].split(' ').nth(1).unwrap());
    let path = Path::new(url_path.as_str());

    let body = if path.is_dir() {
        // TODO: Show all the files and subfolders
        let mut files_list = String::new();

        let paths = read_dir(path).unwrap();
        for content in paths {
            let file_name = content.unwrap().file_name();
            let file = file_name.to_str().unwrap();

            files_list = format!("{files_list}<li><a href=\"{}/{file}\">{file}</a></li>", path.as_os_str().to_str().unwrap());
        }

        format!("<h1>File Server</h1><ul>{files_list}</ul>")
    } else {
        // TODO: Show the file
        std::fs::read_to_string(path).unwrap()
    };

    let status_line = "HTTP/1.1 200 OK";

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
