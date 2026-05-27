use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let fut1 = async move {
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

        let fut2 = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        trpl::join(fut1, fut2).await;
    });
}
