use tokio::sync::Mutex;
use std::{time::{SystemTime, UNIX_EPOCH}, collections::VecDeque, sync::{atomic::{Ordering, AtomicIsize}, Arc}};

use app::json::JsonMessage;
use actix_web::{get, web, Responder, HttpResponse, HttpServer, App, post};

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

struct QueueMessage {
    time: u128,

    message: JsonMessage,
}

struct MyQueue {
    queue: Arc<Mutex<VecDeque<QueueMessage>>>,
}

impl Default for MyQueue {
    fn default() -> Self {
        return MyQueue {
            queue: Arc::new(Mutex::new(VecDeque::new())),
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
        let mut queue = self.queue.lock().await;
        while let Some(item) = queue.get(0) {
            if item.time < now {
                queue.pop_front();
            } else {
                break;
            }
        }
        msg.map(|x| {
            queue.push_back(x);
        });
        queue.len()
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





