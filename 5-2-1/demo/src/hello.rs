use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

pub struct Hello {
    state: StateHello,
}

enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    pub fn new() -> Self {
        Hello {
            state: StateHello::HELLO,
        }
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        match(*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;
                Poll::Pending
            }
            StateHello::WORLD => {
                println!("World");
                (*self).state = StateHello::END;
                Poll::Pending
            }
            StateHello::END => {
                Poll::Ready(())
            }
        }
    }
}