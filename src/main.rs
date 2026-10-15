use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();

        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} in the first task!");
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} in the second task!");
            }
        };

        trpl::join(fut1, fut2).await;
    });
}
