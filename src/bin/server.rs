use flume::{Receiver, Sender};
use lockfree::prelude::Queue as LFQueue;
use tokio::sync::Mutex;
use std::{time::{SystemTime, UNIX_EPOCH}, collections::VecDeque, sync::{atomic::{Ordering, AtomicIsize, AtomicUsize}, Arc}};

use app::json::JsonMessage;
use actix_web::{get, web, Responder, HttpResponse, HttpServer, App, post};

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

struct QueueMessage {
    time: u128,
    message: JsonMessage,
}

impl Default for QueueMessage {
    fn default() -> Self {
        return Self {
            time: 0,
            message: JsonMessage::default(),
        };
    }
}

struct MyQueue {
    tx_queue: Sender<QueueMessage>,
    tx_peek: Sender<QueueMessage>,
    queue: Receiver<QueueMessage>,
    peek: Receiver<QueueMessage>,
    len: AtomicUsize,
}

impl Default for MyQueue {
    fn default() -> Self {
        let (tx_queue, queue) = flume::bounded(1_000_000);
        let (tx_peek, peek) = flume::bounded(100);

        return MyQueue {
            queue,
            peek,
            tx_queue,
            tx_peek,
            len: AtomicUsize::new(0),
        }
    }
}

fn get_now() -> u128 {
    let start = SystemTime::now();
    return start
        .duration_since(UNIX_EPOCH)
        .expect("I hate my life and this shouldn't of failed so fu unix")
        .as_millis();
}

impl MyQueue {
    async fn empty_queue(&self, now: u128, msg: Option<QueueMessage>) -> usize {
        let mut count: isize = 0;
        let mut check = true;
        while let Ok(item) = self.peek.try_recv() {
            if item.time > now {
                self.tx_peek.send(item).expect("always blue");
                check = false;
                break;
            } else {
                count -= 1;
            }
        }

        if check {
            while let Ok(item) = self.queue.try_recv() {
                if item.time < now {
                    count -= 1;
                } else {
                    self.tx_peek.send(item).expect("always blue");
                    break;
                }
            }
        }

        msg.map(|m| {
            self.tx_queue.send(m).expect("always blue");
            count += 1;
        });

        return if count > 0 {
            self.len.fetch_add(count as usize, Ordering::Relaxed)
        } else if count < 0 {
            // TODO: I am terrible
            self.len.fetch_sub((-1 * count) as usize, Ordering::Relaxed)
        } else {
            self.len.load(Ordering::Relaxed)
        };
    }
}

#[post("/json/{time_in_queue}")]
async fn json(req: web::Json<JsonMessage>, data: web::Data<MyQueue>, time_in_queue: web::Path<usize>) -> impl Responder {
    let now = get_now();
    data.empty_queue(now, Some(QueueMessage {
        time: now + (*time_in_queue as u128),
        message: req.0,
    })).await;

    let resp = HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("time in queue will be {}", time_in_queue));

    return resp;
}

#[get("/status")]
async fn status(data: web::Data<MyQueue>) -> impl Responder {
    let now = get_now();
    let len = data.empty_queue(now, None).await;

    let resp = HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("{}", len));

    return resp;
}

#[tokio::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let workers: usize = str::parse(&std::env::var("WORKERS").unwrap_or("1".to_string())).unwrap_or(1);

    let topics: web::Data<MyQueue> = web::Data::new(MyQueue::default());

    println!("workers {}", workers);
    HttpServer::new(move || {
        App::new()
            .app_data(topics.clone())
            .service(json)
            .service(status)
    })
    .workers(num_cpus::get() * workers)
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}



