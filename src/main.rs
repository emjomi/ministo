mod job;
mod share;
mod stratum;
mod worker;

use crate::{stratum::Stratum, worker::Worker};
use clap::Parser;
use std::{
    io,
    num::{NonZeroU64, NonZeroUsize},
    time::{Duration, Instant},
};

const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Parser)]
struct Args {
    /// Pool address (URL:PORT)
    #[arg(short = 'o', long, default_value = "gulf.moneroocean.stream:10001")]
    url: String,
    /// Wallet address
    #[arg(
        short,
        long,
        default_value = "8571HAJKFudM4Y12Q9WLaMGgsiMyJCkvZaw38iy1ufMAXRuZbdfVekH7Ab6UdDFVfJbrASauW4iU69nM6dZ2A4hv1dBnRsp"
    )]
    user: String,
    /// Worker name
    #[arg(short, long, default_value = "x")]
    pass: String,
    /// Number of CPU threads
    #[arg(short, long, default_value_t = all_threads())]
    threads: NonZeroUsize,
    /// Switch to light mode
    #[arg(long)]
    light: bool,
    /// Hashrate log interval in seconds
    #[arg(long, default_value_t = NonZeroU64::new(60).expect("60 is non-zero"))]
    hashrate_log_interval: NonZeroU64,
}

fn all_threads() -> NonZeroUsize {
    std::thread::available_parallelism().expect("failed to determine available parallelism")
}

fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .with_file(false)
        .with_line_number(false)
        .init();

    let Args {
        url,
        user,
        pass,
        light,
        threads,
        hashrate_log_interval,
    } = Args::parse();

    let mut stratum = Stratum::login(&url, &user, &pass)?;
    let worker = Worker::init(stratum.try_recv_job().unwrap(), threads, !light);

    let mut keep_alive_timer = Instant::now();
    let mut hashrate_timer = Instant::now();
    let hashrate_interval = Duration::from_secs(hashrate_log_interval.get());

    let mut last_hash_count = 0u64;

    loop {
        if let Ok(job) = stratum.try_recv_job() {
            worker.work(job);
        }
        if let Ok(share) = worker.try_recv_share() {
            stratum.submit(share)?;
        }
        if keep_alive_timer.elapsed() >= KEEP_ALIVE_INTERVAL {
            keep_alive_timer = Instant::now();
            stratum.keep_alive()?;
        }
        if hashrate_timer.elapsed() >= hashrate_interval {
            hashrate_timer = Instant::now();
            let hash_count = worker.hash_count();
            tracing::info!(
                "hashrate {} H/s",
                hash_count.wrapping_sub(last_hash_count) / hashrate_log_interval.get()
            );
            last_hash_count = hash_count;
        }
    }
}
