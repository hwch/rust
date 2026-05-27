use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();
        let tx_fut1 = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        let tx_fut2 = async move {
            let vals = vec![
                String::from("Hi"),
                String::from("From"),
                String::from("The"),
                String::from("Future"),
                String::from("Future1"),
                String::from("Future2"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
        trpl::join!(rx_fut, tx_fut2, tx_fut1);
    });
}
