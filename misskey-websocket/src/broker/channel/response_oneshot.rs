use crate::broker::model::SharedBrokerState;
use crate::error::Result;

use futures::channel::oneshot::{self, Receiver, Sender};

#[derive(Debug)]
pub struct ResponseSender<T>(Sender<T>);

impl<T> ResponseSender<T> {
    pub fn send(self, t: T) {
        if self.0.send(t).is_err() {
            panic!("oneshot broker response channel unexpectedly closed");
        }
    }
}

#[derive(Debug)]
pub struct ResponseReceiver<T> {
    inner: Receiver<T>,
    state: SharedBrokerState,
}

impl<T> ResponseReceiver<T> {
    pub async fn recv(self) -> Result<T> {
        match self.inner.await {
            Ok(x) => Ok(x),
            Err(_) => {
                let state = self.state.read().await;
                let err = state
                    .dead()
                    .expect("broker control channel unexpectedly closed");
                Err(err.clone())
            }
        }
    }
}

pub fn response_channel<T>(state: SharedBrokerState) -> (ResponseSender<T>, ResponseReceiver<T>) {
    let (sender, receiver) = oneshot::channel();
    (
        ResponseSender(sender),
        ResponseReceiver {
            inner: receiver,
            state,
        },
    )
}