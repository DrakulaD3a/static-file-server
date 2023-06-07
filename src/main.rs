use clap::Parser;

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 7878)]
    port: u16,
}

fn main() {
    let args = Args::parse();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connnection(stream);
    }
}

fn handle_connnection(mut stream: TcpStream) {
    let paths = std::fs::read_dir("./").unwrap();

    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(Result::unwrap)
        .take_while(|line| !line.is_empty())
        .collect();

    let mut files_list = String::new();

    for path in paths {
        let file_type = path.as_ref().unwrap().file_type().unwrap();
        let file_name = path.unwrap().file_name();
        let file = file_name.to_str().unwrap();

        files_list = if file_type.is_dir() {
            format!("{files_list}<li><a href=\"{file}\">{file}</a></li>")
        } else {
            format!("{files_list}<li>{file}</li>")
        };
    }

    let response = format!(
        "HTTP/1.1 200 OK

<!DOCTYPE html>
<html>
<head>
    <title>File Server</title>
</head>
<body>
    <h1>File Server</h1>
    <ul>
        {files_list}
    </ul>
</body>
</html>
");

    stream.write_all(response.as_bytes()).unwrap();
}
