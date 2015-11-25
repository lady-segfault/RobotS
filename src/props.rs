use std::marker::{PhantomData, Reflect};
use std::sync::Arc;

use Actor;

/// Factory for `A`.
///
/// It will always create an `A` with the same function and arguments.
///
/// It is also thread safe, and thus we can respawn an Actor across different threads.
pub struct Props<Args: Copy , M: Copy  + Send + 'static + Reflect, A: Actor<M>> {
    _phantom_actor: PhantomData<A>,
    _phantom_message: PhantomData<M>,
    creator: Arc<Fn(Args) -> A  + Send>,
    args: Args,
}

impl<Args: Copy , M: Copy  + Send + 'static + Reflect, A: Actor<M>> Props<Args, M, A> {
    /// Creates a `Props` which is a factory for `A` with the `creator` function and `args` args.
    pub fn new(creator: Arc<Fn(Args) -> A  + Send>, args: Args) -> Props<Args, M, A> {
        Props::<Args, M, A> {
            _phantom_actor: PhantomData,
            _phantom_message: PhantomData,
            creator: creator,
            args: args,
        }
    }

    /// Creates an Actor instance with the `creator` function and the `args` args.
    ///
    /// This is meant to allow to respawn an Actor when it fails.
    pub fn create(&self) -> A {
        // TODO(gamazeps): reopen https://github.com/rust-lang/rust/issues/18343 with an example.
        let args = self.args;
        (self.creator)(args)
    }
}

impl<Args: Copy , M: Copy  + Send + 'static + Reflect, A: Actor<M>> Clone for Props<Args, M, A> {
    fn clone(&self) -> Props<Args, M, A> {
        Props::<Args, M, A> {
            _phantom_actor: PhantomData,
            _phantom_message: PhantomData,
            creator: self.creator.clone(),
            args: self.args,
        }
    }
}
