use futures::stream;
use futures::task::Poll;
use futures::task::Context;
use std::pin::Pin;
use futures::channel::mpsc;
use futures::prelude::*;

trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

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
