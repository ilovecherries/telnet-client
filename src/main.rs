use std::io::{self};
use std::net::TcpStream;

const ADDR: &str = "freechess.org:5000";

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect(ADDR)?;
    let stdout = io::stdout();
    let mut stdin = io::stdin();

    loop {
        let (tcp_reader, tcp_writer) = &mut (&stream, &stream);
        let (_reader, stdout_writer) = &mut (&stdout, &stdout);
        io::copy(tcp_reader, stdout_writer)?;
        // lol this doesnt fucking work, im just uploading this for future reference
        io::copy(&mut stdin, tcp_writer)?;
    }

    Ok(())
}
