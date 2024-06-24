use std::{pin::pin, thread, time::Duration};

use trpl::{interval, IntervalStream, ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::block_on(async {
        let messages = get_messages()
            .timeout_repeating(interval(Duration::from_millis(200)));

        let deciseconds =
            IntervalStream::new(interval(Duration::from_millis(1)))
                .throttle(Duration::from_millis(100))
                .map(|interval| {
                    let duration = interval.elapsed();
                    format!("Interval: {}ms elapsed", duration.as_millis())
                })
                .timeout(Duration::from_secs(10));

        let stream = messages.merge(deciseconds).take(10);

        let mut stream = pin!(stream);
        while let Some(result) = stream.next().await {
            match result {
                Ok(item) => println!("{item}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (message_number, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if message_number % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: '{message}' after {time_to_sleep}ms"))
                .unwrap();
        }
    });

    ReceiverStream::new(rx)
}
