mod request;
mod response;

pub use request::*;
pub use response::*;
use std::io;
use std::io::{BufReader, BufWriter, Write};

use serde::{de::DeserializeOwned, Serialize};
use std::net::TcpStream;

pub fn send<S: Serialize>(
    writer: &mut BufWriter<TcpStream>,
    request: &Request<S>,
) -> io::Result<()> {
    serde_json::to_writer(&mut *writer, request)?;
    writeln!(writer)?;
    writer.flush()?;
    Ok(())
}

pub fn recv<D: DeserializeOwned>(reader: &mut BufReader<TcpStream>) -> serde_json::Result<D> {
    let mut de = serde_json::Deserializer::from_reader(reader);
    D::deserialize(&mut de)
}
