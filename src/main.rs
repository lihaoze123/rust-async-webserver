use std::time::Duration;

use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream}, time::sleep,
};

async fn handle_connection(mut stream: TcpStream) -> Result<(), anyhow::Error> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next_line().await?;

    let (status_line, filename) = match request_line {
        Some(x) if x == "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        Some(x) if x == "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(2)).await;
            ("HTTP/1.1 200 OK", "sleep.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).await?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        handle_connection(socket).await?;
    }
}
