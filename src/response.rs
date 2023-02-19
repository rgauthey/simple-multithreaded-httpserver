use std::fs;
use std::path::Path;

use crate::request;

pub fn build_response(request: String) -> String {
    let request_parts = request::parse_request(request);

    let method = request_parts.get(0).unwrap().as_str();
    let path = request_parts.get(1).unwrap().as_str();

    let (status_line, mut contents) = if method == "GET" && path == "/" {
        let path = Path::new("pages/index.html");
        if path.is_file() {
            ("HTTP/1.1 200 OK\r\n\r\n", fs::read_to_string(path).unwrap())
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404 Not Found".to_string())
        }
    } else {
        let path = Path::new(&path[1..]); // remove leading "/"
        if path.is_file() {
            ("HTTP/1.1 200 OK\r\n\r\n", fs::read_to_string(path).unwrap())
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404 Not Found".to_string())
        }
    };

    contents.insert_str(0, status_line);

    contents
}
