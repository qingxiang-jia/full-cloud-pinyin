use rand::Rng;
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

fn main() {
    let mut rng = rand::thread_rng();
    let handler = TaskHandler::new();
    handler.handle_task(Task {
        to_print: "a".to_owned(),
        delay: rng.gen_range(0..500),
    });
    handler.handle_task(Task {
        to_print: "b".to_owned(),
        delay: rng.gen_range(0..500),
    });
    handler.handle_task(Task {
        to_print: "c".to_owned(),
        delay: rng.gen_range(0..500),
    });
    handler.handle_task(Task {
        to_print: "d".to_owned(),
        delay: rng.gen_range(0..500),
    });

    std::thread::sleep(Duration::from_millis(4000)); // Wait for the tasks to finish.
}

pub struct Task {
    to_print: String,
    delay: i32,
}

async fn long_running(to_print: String, delay: i32) {
    sleep(Duration::from_millis(delay as u64)).await;
    println!("{}", to_print);
}

#[derive(Clone)]
pub struct TaskHandler {
    send_channel: mpsc::Sender<Task>,
}

impl TaskHandler {
    pub fn new() -> TaskHandler {
        // Set up a channel for communicating.
        let (send, mut recv) = mpsc::channel::<Task>(16);

        // Build the runtime for the new thread.
        //
        // The runtime is created before spawning the thread
        // to more cleanly forward errors if the `unwrap()`
        // panics.
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        std::thread::spawn(move || {
            rt.block_on(async move {
                while let Some(task) = recv.recv().await {
                    tokio::spawn(long_running(task.to_print, task.delay));
                }

                // Once all senders have gone out of scope,
                // the `.recv()` call returns None and it will
                // exit from the while loop and shut down the
                // thread.
            });
        });

        TaskHandler { send_channel: send }
    }

    pub fn handle_task(&self, task: Task) {
        match self.send_channel.blocking_send(task) {
            Ok(()) => {}
            Err(_) => panic!("The shared runtime has shut down."),
        }
    }
}
