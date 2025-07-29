fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        webserver::handle_client(stream?);
    }

    Ok(())
}