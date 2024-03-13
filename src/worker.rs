use crate::{job::Job, share::Share};

use bus::Bus;
use randomx_rs::{RandomXCache, RandomXDataset, RandomXFlag, RandomXVM};
use std::{
    num::NonZeroUsize,
    sync::mpsc::{channel, Receiver, TryRecvError},
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

        let mut flags = RandomXFlag::get_recommended_flags();
        let cache = match mode {
            Mode::Fast => None,
            Mode::Light => Some(RandomXCache::new(flags, &job.seed).unwrap()),
        };
        let dataset = match mode {
            Mode::Fast => {
                flags |= RandomXFlag::FLAG_FULL_MEM;
                let cache = RandomXCache::new(flags, &job.seed).unwrap();
                Some(RandomXDataset::new(flags, cache, 0).unwrap())
            }
            Mode::Light => None,
        };
        for i in 0..num_threads.get() {
            let mut job_rx = job_bus.add_rx();
            let share_tx = share_tx.clone();
            let cache = cache.clone();
            let dataset = dataset.clone();
            let mut nonce = i as u16;
            let mut job = job.clone();
            let mut diff = job.difficulty();
            thread::spawn(move || {
                let vm = RandomXVM::new(flags, cache, dataset).unwrap();
                loop {
                    if let Ok(new_job) = job_rx.try_recv() {
                        nonce = i as u16;
                        job = new_job;
                        diff = job.difficulty();
                    }
                    if nonce < u16::MAX {
                        let nonce_bytes = &(nonce as u32).to_be_bytes();
                        job.blob[39..=42].copy_from_slice(nonce_bytes);
                        let hash = vm.calculate_hash(&job.blob).unwrap();
                        if u64::from_le_bytes(hash[24..].try_into().unwrap()) <= diff {
                            let _ = share_tx.send(Share {
                                job_id: job.id.clone(),
                                nonce: nonce_bytes.to_vec(),
                                hash,
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
