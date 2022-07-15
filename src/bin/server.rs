use std::{
    collections::VecDeque,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver},
    oneshot,
};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use app::json::JsonMessage;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

/// Tuple used to send message to the task that contains the queue and allows to send back the size of the queue
/// with the oneshot channel
type QueueChannelMessage = (u128, Option<QueueMessage>, oneshot::Sender<usize>);

struct QueueMessage {
    time: u128,

    message: JsonMessage,
}

struct MyQueue {
    // queue: Arc<Mutex<VecDeque<QueueMessage>>>,
    queue_sender: mpsc::UnboundedSender<QueueChannelMessage>,
}

impl Default for MyQueue {
    fn default() -> Self {
        let (queue_sender, queue_receiver) = mpsc::unbounded_channel();

        // Queue receiver goes into this tokio task and will remain there forever
        // Operation on the queue will happen on this task, we might wanna implement some kind of shutdown logic
        tokio::spawn(Self::queue_receiver_job(queue_receiver));

        return MyQueue { queue_sender };
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
    async fn queue_receiver_job(mut queue_receiver: UnboundedReceiver<QueueChannelMessage>) {
        let mut queue: VecDeque<QueueMessage> = VecDeque::new();

        while let Some((now, msg, len_sender)) = queue_receiver.recv().await {
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

            // We might not wanna ignore error here hehe
            let _ = len_sender.send(queue.len());
        }
    }

    async fn empty_queue(&self, now: u128, msg: Option<QueueMessage>) -> usize {
        let (len_sender, len_receiver) = oneshot::channel();

        // We might not wanna ignore error there as well
        let _ = self.queue_sender.send((now, msg, len_sender));

        // Here is a bit of a bold move to unwrap, but why not ? :)
        return len_receiver.await.unwrap();
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
        Some(QueueMessage {
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
