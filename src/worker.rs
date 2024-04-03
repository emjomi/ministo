use crate::{job::Job, share::Share};

use bus::Bus;
use rust_randomx::{Context, Hasher};
use std::{
    num::NonZeroUsize,
    sync::{
        mpsc::{channel, Receiver, TryRecvError},
        Arc,
    },
    thread,
};

#[derive(Clone, Copy)]
pub enum Mode {
    Light,
    Fast,
}

pub struct Worker {
    share_rx: Receiver<Share>,
    job_bus: Bus<Job>,
}

impl Worker {
    pub fn init(job: Job, mode: Mode, num_threads: NonZeroUsize) -> Self {
        let (share_tx, share_rx) = channel();
        let mut job_bus = Bus::new(8);
        let context = Arc::new(Context::new(&job.seed, matches!(mode, Mode::Fast)));
        for i in 0..num_threads.get() {
            let mut job_rx = job_bus.add_rx();
            let share_tx = share_tx.clone();
            let context = context.clone();
            let mut nonce = i as u16;
            let mut job = job.clone();
            let mut difficulty = job.difficulty();
            thread::spawn(move || {
                let hasher = Hasher::new(context);
                loop {
                    if let Ok(new_job) = job_rx.try_recv() {
                        nonce = i as u16;
                        job = new_job;
                        difficulty = job.difficulty();
                    }
                    if nonce < u16::MAX {
                        let nonce_bytes = &(nonce as u32).to_be_bytes();
                        job.blob[39..=42].copy_from_slice(nonce_bytes);
                        let hash = hasher.hash(&job.blob);
                        if u64::from_le_bytes(hash.as_ref()[24..].try_into().unwrap()) <= difficulty
                        {
                            let _ = share_tx.send(Share {
                                job_id: job.id.clone(),
                                nonce: nonce_bytes.to_vec(),
                                hash: hash.as_ref().try_into().unwrap(),
                            });
                        }
                        nonce = nonce.saturating_add(num_threads.get() as u16);
                    }
                }
            });
        }
        Worker { share_rx, job_bus }
    }
    pub fn work(&mut self, job: Job) {
        self.job_bus.broadcast(job);
    }
    pub fn try_recv_share(&self) -> Result<Share, TryRecvError> {
        self.share_rx.try_recv()
    }
}
