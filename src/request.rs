pub fn parse_request(request: String) -> Vec<String> {
    let mut lines = request.lines();
    let request_line = lines.next().unwrap().to_owned();

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap().to_owned();
    let path = parts.next().unwrap().to_owned();

    vec![method, path]
}
