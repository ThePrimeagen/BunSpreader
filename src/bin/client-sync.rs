use anyhow::{Result, anyhow, Context};
use app::json::{JsonMessage, InnerJsonMessage};
use clap::Parser;
use reqwest::Client;
use std::{sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
}, time::Duration};
use tokio::{sync::Semaphore, time::Instant};

#[derive(Parser)]
struct Args {
    #[clap(env = "TIQ", short = 't', long = "time", default_value= "10000")]
    time_in_queue: usize,

    #[clap(env = "COUNT", short = 'c', long = "count", default_value= "1000")]
    count: usize,

    #[clap(env = "ADDR", short = 'a', long = "address", default_value= "0.0.0.0")]
    address: String,

    #[clap(env = "PORT", short = 'p', long = "port", default_value = "42069")]
    port: u16,

    #[clap(env = "MAX_CONN", short = 'm', long = "max_conn", default_value = "100")]
    max_conn: usize,
}


#[derive(Default)]
struct Stats {
    error: AtomicUsize,
    success: AtomicUsize,
}

async fn get_status(client: Client, url: &str) -> Result<String> {
    let resp = match client
            .get(url)
            .send().await {
        Ok(r) => r,
        Err(_) => {
            return Err(anyhow!("unable to make the reqwest"));
        }
    };

    let text = resp.text().await;

    return text.context("sorry, this sucked");
}

async fn send_request(client: Client, url: &str, stats: &Stats, body: String) -> Result<String> {
    let resp = match client
            .post(url)
            .header("content-type", "application/json")
            .body(body)
            .send().await {
        Ok(r) => r,
        Err(_) => {
            stats.error.fetch_add(1, Ordering::Relaxed);
            return Err(anyhow!("unable to make the reqwest"));
        }
    };

    let text = resp.text().await;

    stats.success.fetch_add(1, Ordering::Relaxed);
    return text.context("sorry, this sucked");
}

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let args = Args::parse();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", args.port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1000];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..])
}
