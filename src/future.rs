use futures::{Future, Poll};
use errors::Error;

/// A future that resolves into a Fixerio response.
pub struct FixerioFuture<T> {
    inner: Box<Future<Item = T, Error = Error>>,
}

impl<T> FixerioFuture<T> {
    pub(crate) fn new(inner: Box<Future<Item = T, Error = Error>>) -> Self {
        Self { inner: inner }
    }
}

impl<T> Future for FixerioFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}