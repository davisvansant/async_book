use futures::stream;
use futures::task::Poll;
use futures::task::Context;
use std::pin::Pin;
use futures::channel::mpsc;
use futures::prelude::*;
use futures::executor::block_on;
use std::io;

// trait Stream {
//     type Item;
//
//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
// }

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;

    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt;

    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }

    sum
}

async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt;

    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    
    Ok(sum)
}

async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt;

    const MAX_CONCURRENT_JUMPERS: usize = 100;

    stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
        jump_n_times(num).await?;
        report_n_jumps(num).await?;
        Ok(())
    }).await?;

    Ok(())
}

async fn jump_n_times(_: u8) -> Result<(), io::Error> { Ok(()) }
async fn report_n_jumps(_: u8) -> Result<(), io::Error> { Ok(()) }
