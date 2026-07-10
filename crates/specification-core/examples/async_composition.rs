//! Runs a runtime-neutral async specification with a tiny local executor.

use std::{
    convert::Infallible,
    future::{Future, ready},
    task::{Context, Poll, Waker},
};

use specification_core::AsyncSpecification;

struct AtLeast(u8);

impl AsyncSpecification<u8> for AtLeast {
    type Error = Infallible;

    fn is_satisfied_by<'candidate>(
        &'candidate self,
        age: &'candidate u8,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send + 'candidate {
        ready(Ok(*age >= self.0))
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn main() {
    let working_age = AtLeast(18).and(AtLeast(65).not());
    assert!(block_on(working_age.is_satisfied_by(&42)).expect("rule is infallible"));
}
