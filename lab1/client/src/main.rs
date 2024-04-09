use std::io::{self, BufRead, Read, Write};
use std::net::TcpStream;

struct Client<R, W> {
    reader: R,
    writer: W,
    stream: TcpStream,
}

impl<R, W> Client<R, W>
where
    R: BufRead,
    W: Write,
{
    fn new(reader: R, writer: W, addr: String) -> Self {
        let stream = TcpStream::connect(addr).expect("Failed to connect to server");
        Self {
            reader,
            stream,
            writer,
        }
    }

    fn start(&mut self) -> anyhow::Result<()> {
        loop {
            write!(self.writer, "> ")?;

            let mut input = String::new();
            self.reader.read_line(&mut input)?;

            let trimmed = input.trim();
            let mut parts = trimmed.split_whitespace();
            let command = parts.next().unwrap_or("");

            let command = match command {
                "exit" => {
                    return Ok(());
                }
                _ => input,
            };

            self.stream.write_all(command.as_bytes())?;

            let mut buffer = [0; 512];
            self.stream.read(&mut buffer)?;

            let response = std::str::from_utf8(&buffer)?;

            writeln!(self.writer, "{}", response)?;
        }
    }
}

fn main() -> anyhow::Result<()> {
    let stdio = io::stdin();
    let reader = stdio.lock();
    let mut output = io::stdout();
    output.flush()?;

    let mut client = Client::new(reader, output, String::from("0.0.0.0:6379"));
    client.start()
}
