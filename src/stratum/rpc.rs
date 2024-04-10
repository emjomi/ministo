pub mod request;
pub mod response;

use request::Request;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    io::{self, BufReader, BufWriter, Write},
    net::TcpStream,
};

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
