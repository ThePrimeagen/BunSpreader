use lockfree::prelude::Map;
use std::{
    hash::Hash,
    sync::{
        atomic::{AtomicU32, AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::time::{sleep_until, Duration};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use app::json::JsonMessage;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn gen_id() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug)]
struct QueueMessage {
    id: u64,
    time: u64,
    message: JsonMessage,
}

#[derive(Clone)]
struct MyQueue {
    queue: Arc<Map<u64, QueueMessage>>,
    count: Arc<AtomicU32>,
}

impl Default for MyQueue {
    fn default() -> Self {
        return MyQueue {
            queue: Arc::new(Map::new()),
            count: Default::default(),
        };
    }
}

impl MyQueue {
    fn add_item(&self, item: QueueMessage) {
        let id = item.id;
        let exp = item.time;

        self.queue.insert(id, item);
        self.count.fetch_add(1, Ordering::SeqCst);

        let this = self.clone();
        tokio::task::spawn(async move {
            tokio::time::sleep(Duration::from_millis(exp)).await;

            this.queue.remove(&id);
            this.count.fetch_sub(1, Ordering::SeqCst);
        });
    }
}

#[post("/json/{time_in_queue}")]
async fn json(
    req: web::Json<JsonMessage>,
    data: web::Data<MyQueue>,
    time_in_queue: web::Path<usize>,
) -> impl Responder {
    data.add_item(QueueMessage {
        id: gen_id(),
        time: *time_in_queue as _,
        message: req.0,
    });

    let resp = HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("time in queue will be {}", time_in_queue));

    return resp;
}

#[get("/status")]
async fn status(data: web::Data<MyQueue>) -> impl Responder {
    let count = data.count.load(Ordering::SeqCst);
    let resp = HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("{}", count));

    return resp;
}

#[tokio::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let workers: usize =
        str::parse(&std::env::var("WORKERS").unwrap_or("1".to_string())).unwrap_or(1);

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
