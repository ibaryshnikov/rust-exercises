use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

struct Data {
    value: i32,
}

impl Future for Data {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.value)
    }
}

#[tokio::main]
async fn main() {
    let data = Data { value: 5 };
    let answer = data.await;
    println!("The answer is {}", answer);
}
