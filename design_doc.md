# RobotS an actor library for the rust language

## Introduction

This document will present the design of the RobotS library.

This library is highly inspired from the work done in [akka](akka.io) which is itself inspired by
Erlang's processes in the [Eralng OTP](http://www.erlang.org/doc/).

Just like in akka:
  - Actors will have a hierarchy, each actor supervising its children.
  - Actors will be defined by implementing the `Actor` trait and by implementing the `receive`
 method.
  - An ActorSystem will be present.

The value of this project is to combine the advantages of an Actor model and of a type system,
thus catching as much errors at compile time as possible and having guarantees. It would also be
the first implementation of an actor model in rust and is thus also useful in that way.

## Actors creation

We will use the system of `Props` used in akka to have immutable and thread safe factories of
`Actors`, having these is valuable as it allows to recreate / restart actors if they fail.

```
struct Props<T: Actor> {
    /// Here will go the fields when options are given for actors creations.
    // This is needed to have genericity over any T if we do not hold any T.
    _phantom: PhantomData<T>
    // TODO(raimundo) a sequence of Any.
}

impl Props<T> {}

actor_ref = actor_system.actor_of(Props::MyActor::new(), "my_actor");
```

An instance of the actor is then created using either actor\_system.actor\_of(props, name) or
with self.actor\_context().actor\_of(props, name), the first one create a top level user actor
and the second one creates a child of the actor's whose actor\_context is called.
This gives us a reference to the newly created actor.

## Actors Implementation

In order to implement a new `Actor`, it is necessary to create a `struct` with the attributes of
the actor (is any) and to implement the `Actor` trait for that struct. Note that only the `receive`
method has to be implemented.

```
struct MyActor {
    // Fields go here.
}

impl Actor for MyActor {
    fn receive(message: Message, context: &Context) -> () {
        match message.content() {
            case "hello".to_owned() => println!("Oh, hello there"),
            case _ => println!("Haven't your parents told you to greet people ?")
        }
    }
}
```

TODO(gamazeps): explain how to do and where to do the new.

## Actor References

Just like the actor model imposes, we never manipulate actors, but actor references instead :
  - Sending messages is done by sending addresing the actor with its reference.
  - Creating an actor gives out a reference to this actor.

The actor reference structure looks like the following:

// TODO(gamazeps): add the appropriate concurrency protections.
```
struct ActorRef<T: Actor> {
    /// Whether the actor is local or not.
    bool: is_local,
    /// Logical path to the actor.
    path: ActorPath,
    /// 'real' actor.
    actor_cell: ActorCell,
}
```

## Actor cell

This is what is used to contain the logic of an actor, i.e its actor system, props, mailbox, sender
information and children and parent ActorRef.

```
struct ActorCell<T: Actor> {
    mailbox: Mailbox,
    props: Props<T>,
    actor_system: ActorSystem,
    // TODO (be carefull about genericity here, ActorRef must not! be generic).
    parent: ActorRef<Stuff>,
    actor: T
}
```
