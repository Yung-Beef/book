use std::{pin::pin, thread, time::Duration};

use trpl::{interval, IntervalStream, ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::block_on(async {
        let messages = pin!(get_messages());

        let deciseconds =
            pin!(IntervalStream::new(interval(Duration::from_millis(1)))
                .throttle(Duration::from_millis(100))
                .map(|interval| {
                    let duration = interval.elapsed();
                    format!("milliseconds elapsed: {}", duration.as_millis())
                }));

        let mut merged = messages.merge(deciseconds).take(10);
        while let Some(alternative) = merged.next().await {
            println!("Got: {alternative:?}");
        }
    })
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let thread_tx = tx.clone();
    thread::spawn(move || {
        for message in [String::from("Hello"), String::from("Goodbye")] {
            thread_tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    trpl::spawn_task(async move {
        for message in [String::from("Hello"), String::from("Goodbye")] {
            tx.send(message).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    ReceiverStream::new(rx)
}
