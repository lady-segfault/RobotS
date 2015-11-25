use std::any::Any;
use std::marker::Reflect;
use std::sync::Arc;

use eventual::{Future, Complete};

use {Actor, ActorCell, ActorContext, CanReceive};

impl<M: Copy  + Send + 'static + Reflect,
    E: Sync + Send + 'static>
    CanReceive for Complete<M, E> {
    fn receive(&self, message: Box<Any>, sender: Arc<CanReceive >) {
        let cast = message.downcast::<M>();
        match cast {
            Ok(message) => self.complete(*message),
            Err(_) => panic!("Send a message of the wrong type to a future"),
        }
    }

    fn handle(&self) {
        panic!("Tried to call handle on a Complete");
    }
}

pub trait AskPattern<Args: Copy  + Send + 'static,
    M: Copy  + Send + 'static + Reflect,
    A: Actor<M> + 'static,
    E: Send + 'static>: ActorContext<Args, M, A> {
    fn ask<Message: Copy  + Send + 'static + Reflect, T: CanReceive>(&self, to: T, message: Message)
        -> Future<M, E>;
}

impl<Args: Copy  + Send + 'static,
    M: Copy  + Send + 'static + Reflect,
    A: Actor<M> + 'static,
    E: Send + 'static>
    AskPattern<Args, M, A, E> for ActorCell<Args, M, A> {
    fn ask<Message: Copy  + Send + 'static + Reflect, T: CanReceive>(&self, to: T, message: Message)
        -> Future<M, E> {
            let (complete, future) = Future::<M, E>::pair();
        to.receive(Box::new(message), Arc::new(complete));
        future
        }
}

