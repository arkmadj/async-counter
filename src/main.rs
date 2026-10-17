use std::{thread, time::Duration, vec};

fn main() {
    trpl::block_on(async {
        // let handle = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });
        //
        // for i in 1..5 {
        //     println!("hi number {i} from the second task");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }
        //
        // handle.await.unwrap();

        // let fut1 = async {
        //     for i in 1..10 {
        //         println!("hi number {i} in the first task!");
        //     }
        // };
        //
        // let fut2 = async {
        //     for i in 1..5 {
        //         println!("hi number {i} in the second task!");
        //     }
        // };
        //
        // trpl::join(fut1, fut2).await;
        //
        // let (tx, mut rx) = trpl::channel();
        //
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        //
        // let received = rx.recv().await.unwrap();
        // println!("received '{received}'");

        //     let (tx, mut rx) = trpl::channel();
        //
        //     let tx1 = tx.clone();
        //     let tx1_fut = async move {
        //         let vals = vec![
        //             String::from("hi"),
        //             String::from("from"),
        //             String::from("the"),
        //             String::from("future"),
        //         ];
        //
        //         for val in vals {
        //             tx1.send(val).unwrap();
        //             trpl::sleep(Duration::from_millis(500)).await;
        //         }
        //     };
        //
        //     let rx_fut = async {
        //         while let Some(value) = rx.recv().await {
        //             println!("received '{value}'");
        //         }
        //     };
        //
        //     let tx_fut = async move {
        //         let vals = vec![
        //             String::from("more"),
        //             String::from("messages"),
        //             String::from("for"),
        //             String::from("you"),
        //         ];
        //
        //         for val in vals {
        //             tx.send(val).unwrap();
        //             trpl::sleep(Duration::from_millis(1500)).await;
        //         }
        //     };
        //
        //     trpl::join!(tx1_fut, tx_fut, rx_fut);

        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 30);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 30);
            trpl::sleep(one_ms).await;
            slow("b", 30);
            trpl::sleep(one_ms).await;
            slow("b", 30);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
