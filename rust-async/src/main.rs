use rand::Rng;
use tokio::runtime::Builder;
use tokio::time::{sleep, Duration};

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();

    let h1 = runtime.spawn(long_running("a".to_owned(), rng.gen_range(1..500)));
    let h2 = runtime.spawn(long_running("b".to_owned(), rng.gen_range(1..500)));
    let h3 = runtime.spawn(long_running("c".to_owned(), rng.gen_range(1..500)));
    let h4 = runtime.spawn(long_running("d".to_owned(), rng.gen_range(1..500)));
    _ = runtime.block_on(h1); // Otherwise main ends and the async runtime also ends so long_running don't get a chance to run.
    _ = runtime.block_on(h2); // Otherwise main ends and the async runtime also ends so long_running don't get a chance to run.
    _ = runtime.block_on(h3); // Otherwise main ends and the async runtime also ends so long_running don't get a chance to run.
    _ = runtime.block_on(h4); // Otherwise main ends and the async runtime also ends so long_running don't get a chance to run.
}

async fn long_running(to_print: String, how_long: i32) {
    sleep(Duration::from_millis(how_long as u64)).await;
    println!("{}", to_print);
}
