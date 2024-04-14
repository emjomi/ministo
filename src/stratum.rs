mod rpc;

use crate::{job::Job, share::Share};
use rpc::{
    request::{KeepAlivedParams, LoginParams, Request, SubmitParams},
    response::{LoginResult, Response, StatusResult},
};
use serde::Deserialize;
use std::{
    io::{self, BufReader, BufWriter},
    net::TcpStream,
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum PoolMessage {
    Response(Response<StatusResult>),
    NewJob(Request<Job>),
}

pub struct Stratum {
    login_id: String,
    writer: BufWriter<TcpStream>,
    job_rx: Receiver<Job>,
}

impl Stratum {
    #[tracing::instrument]
    pub fn login(url: &str, user: &str, pass: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(url)?;
        stream.set_read_timeout(None)?;
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut writer = BufWriter::new(stream.try_clone()?);

        let (job_tx, job_rx) = mpsc::channel();

        rpc::send(
            &mut writer,
            &Request::<LoginParams>::new(LoginParams {
                login: user.into(),
                pass: pass.into(),
            }),
        )?;
        let response = rpc::recv::<Response<LoginResult>>(&mut reader)?;
        if let Some(result) = response.result {
            tracing::info!("success");
            let LoginResult { id, job, .. } = result;
            job_tx.send(job).unwrap();
            thread::spawn(move || {
                let span = tracing::info_span!("listener");
                let _enter = span.enter();
                loop {
                    let msg = rpc::recv::<PoolMessage>(&mut reader).unwrap();
                    match msg {
                        PoolMessage::Response(response) => {
                            if let Some(err) = response.error {
                                tracing::warn!("{}", err.message);
                            } else {
                                match response.result.unwrap().status.as_str() {
                                    "OK" => tracing::info!("accepted"),
                                    "KEEPALIVED" => tracing::debug!("keepalived"),
                                    _ => todo!(),
                                }
                            }
                        }
                        PoolMessage::NewJob(request) => {
                            tracing::info!("new job");
                            job_tx.send(request.params).unwrap()
                        }
                    }
                }
            });
            Ok(Self {
                login_id: id,
                writer,
                job_rx,
            })
        } else {
            let msg = response.error.unwrap().message;
            tracing::warn!("{}", msg);
            Err(io::Error::other(msg))
        }
    }
    pub fn submit(&mut self, share: Share) -> io::Result<()> {
        rpc::send(
            &mut self.writer,
            &Request::<SubmitParams>::new(SubmitParams {
                id: self.login_id.clone(),
                job_id: share.job_id,
                nonce: share.nonce,
                result: share.hash,
            }),
        )
    }
    pub fn keep_alive(&mut self) -> io::Result<()> {
        rpc::send(
            &mut self.writer,
            &Request::<KeepAlivedParams>::new(KeepAlivedParams {
                id: self.login_id.clone(),
            }),
        )
    }
    pub fn try_recv_job(&self) -> Result<Job, TryRecvError> {
        self.job_rx.try_recv()
    }
}
