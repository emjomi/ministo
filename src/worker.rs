use crate::{job::Job, share::Share};
use rust_randomx::{Context, Hasher};
use std::{
    num::NonZeroUsize,
    sync::{
        mpsc::{self, Receiver, TryRecvError},
        Arc, Mutex,
    },
    thread,
};
use watch::WatchSender;

pub struct Worker {
    share_rx: Receiver<Share>,
    job_tx: WatchSender<Job>,
}

impl Worker {
    #[tracing::instrument(skip(job))]
    pub fn init(job: Job, num_threads: NonZeroUsize, fast: bool) -> Self {
        let (share_tx, share_rx) = mpsc::channel();
        let (job_tx, job_rx) = watch::channel(job.clone());
        tracing::info!("init context");
        let context = Arc::new(Mutex::new(Arc::new(Context::new(&job.seed, fast))));
        tracing::info!("context is ready");
        for i in 0..num_threads.get() {
            let context = context.clone();
            let share_tx = share_tx.clone();
            let mut job_rx = job_rx.clone();
            let mut nonce = i as u16;
            let mut job = job.clone();
            let mut difficulty = job.difficulty();
            thread::spawn(move || {
                let span = tracing::info_span!("thread", id = i);
                let _enter = span.enter();
                let mut hasher = Hasher::new(Arc::clone(&context.lock().unwrap()));
                loop {
                    if let Some(new_job) = job_rx.get_if_new() {
                        if new_job.seed != job.seed {
                            let mut context_lock = context.lock().unwrap();
                            if context_lock.key() != new_job.seed {
                                tracing::info!("reinit context");
                                *context_lock = Arc::new(Context::new(&new_job.seed, fast));
                                tracing::info!("context is ready");
                            }
                            hasher.update(Arc::clone(&context_lock));
                        }
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
                            tracing::debug!("found share");
                            let _ = share_tx.send(Share {
                                job_id: job.id.clone(),
                                nonce: nonce_bytes.to_vec(),
                                hash: hash.as_ref().into(),
                            });
                        }
                        nonce = nonce.saturating_add(num_threads.get() as u16);
                    }
                }
            });
        }
        Self { share_rx, job_tx }
    }
    pub fn work(&self, job: Job) {
        self.job_tx.send(job);
    }
    pub fn try_recv_share(&self) -> Result<Share, TryRecvError> {
        self.share_rx.try_recv()
    }
}
