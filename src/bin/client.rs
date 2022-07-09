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
    #[clap(short = 't', long = "time", default_value= "10000")]
    time_in_queue: usize,

    #[clap(env = "COUNT", short = 'c', long = "count", default_value= "1000")]
    count: usize,

    #[clap(short = 'a', long = "address", default_value= "0.0.0.0")]
    address: String,

    #[clap(short = 'p', long = "port", default_value = "42069")]
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::new();
    let stats = Arc::new(Stats::default());

    let semaphore = Arc::new(Semaphore::new(args.max_conn));

    let url = format!("http://{}:{}/json/{}", args.address, args.port, args.time_in_queue);
    println!("using this url {}", url);
    let url = Arc::new(url);
    let mut handles = vec![];

    let now = Instant::now();
    let body = JsonMessage {
        message: "me daddy".to_string(),
        another_property: InnerJsonMessage {
            width: 4,
            height: 6,
            girth: 8,
            depth: 10,
            length: 12,
            circumference: 14,
        }
    };
    let body = serde_json::to_string(&body)?;

    for i in 0..args.count {
        if i % 10 == 0 {
            println!("look at my i {}", i);
        }

        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let client = client.clone();
        let stats = stats.clone();
        let url = url.clone();
        let body = body.clone();

        handles.push(tokio::spawn(async move {
            send_request(client, &url, &stats, body).await;
            drop(permit);
        }));
    }

    tokio::spawn(async move {
        while semaphore.available_permits() != args.max_conn {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }).await.unwrap_or(());

    let total_time = now.elapsed().as_millis();
    let success = stats.success.load(Ordering::Relaxed);
    let error = stats.error.load(Ordering::Relaxed);
    let rps = args.count as u64 / (total_time as u64);

    println!("total_time: {} success {} errors {} rps {}", total_time, success, error, rps);

    let url = format!("http://{}:{}/status", args.address, args.port);
    println!("url {}", url);
    let url = Arc::new(url);

    loop {
        println!("waiting 1 second and seeing if server is done");
        tokio::time::sleep(Duration::from_millis(1000)).await;
        match get_status(client.clone(), &url).await {
            Ok(out) => {
                println!("just got this back from the server {}", out);
                if let Ok(x) = str::parse::<usize>(&out) {
                    if x == 0 {
                        println!("all dn");
                        break;
                    }
                };
            },
            Err(e) => {
                println!("just got a sweet error {}", e);
            }
        }
    }
    return Ok(());
}
