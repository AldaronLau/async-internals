use std::thread::Thread;
use std::task::{Context, Wake, Poll};
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on(fut: impl Future<Output = ()> + Unpin) {
    let mut fut = fut;
    let t = std::thread::current();
    let waker = Arc::new(ThreadWaker(t)).into();
    let mut cx = Context::from_waker(&waker);

    while Pin::new(&mut fut).poll(&mut cx).is_pending() {
        std::thread::park();
    }
}

fn main() {
    let future = async {
        MyFuture(async_std::task::sleep(Duration::from_millis(500))).await;
    };

    pin_utils::pin_mut!(future);

    block_on(future);
}

struct MyFuture<F: Future<Output = ()>>(F);

impl<F: Future<Output = ()>> Future for MyFuture<F> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let poll = unsafe { self.map_unchecked_mut(|s| &mut s.0).poll(context) };

        if poll.is_ready() {
            println!("Hello, world!");
        }

        poll
    }
}

/*
#[async_std::main]
async fn main() {
    async_std::println!("Hello, world!").await
}*/
