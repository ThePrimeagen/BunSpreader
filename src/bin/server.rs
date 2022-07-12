use lockfree::prelude::Set;
use std::{
    hash::Hash,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use app::json::JsonMessage;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn gen_id() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug)]
struct Item {
    id: u64,
    time: u128,
    message: JsonMessage,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

struct MyQueue {
    queue: Arc<Set<Item>>,
}

impl Default for MyQueue {
    fn default() -> Self {
        return MyQueue {
            queue: Arc::new(Set::new()),
        };
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
    async fn empty_queue(&self, now: u128, msg: Option<Item>) -> usize {
        let mut count = 0;

        for item in self.queue.iter() {
            if item.time < now {
                self.queue.remove(&item);
            } else {
                count += 1;
            }
        }

        if let Some(msg) = msg {
            self.queue.insert(msg).ok();
            count += 1;
        }

        count
    }
}

#[post("/json/{time_in_queue}")]
async fn json(
    req: web::Json<JsonMessage>,
    data: web::Data<MyQueue>,
    time_in_queue: web::Path<usize>,
) -> impl Responder {
    let now = get_now();
    data.empty_queue(
        now,
        Some(Item {
            id: gen_id(),
            time: now + (*time_in_queue as u128),
            message: req.0,
        }),
    )
    .await;

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
