use std::future::Future;

pub trait AsyncFn<Arg>: Fn(Arg) -> Self::Future {
    type Future: Future<Output = ()>;
}

impl<A, F, Fut> AsyncFn<A> for F
    where
        F: Fn(A) -> Fut,
        Fut: Future<Output = ()>,
{
    type Future = Fut;
}