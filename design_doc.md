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

```rust
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

```rust
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
```rust
struct ActorRef<T: Actor> {
    /// Whether the actor is local or not.
    bool: is_local,
    /// Logical path to the actor.
    path: ActorPath,
    /// 'real' actor.
    actor_cell: ActorCell<T>,
}
```

## Actor cell

This is what is used to contain the logic of an actor, i.e its actor system, props, mailbox, sender
information and children and parent ActorRef.

```rust
struct ActorCell<T: Actor> {
    mailbox: Mailbox,
    props: Props<T>,
    actor_system: ActorSystem,
    // TODO (be carefull about genericity here, ActorRef must not! be generic).
    parent: ActorRef<Stuff>,
    actor: T
}
```

## Actor System

The actor system is in charge of the whole set of actors, there should only be one per application.

It is the one who creates and starts the three original actors:
  - The root actor.
  - The user actor.
  - The system actor.

The original actor creation have to go by him using actor_system.actor_of(props, name).

## Actor hierarchy

Just like in akka each actor supervises its children.

There are three actors provided by the actor system which are very important.

### Root actor

He is the one and original actor of the system and thus all actors are its children.

He is the one responsible for monitoring the whole system, terminating him terminates the whole
system.

### User actor

All actors created by the user are its children.

Calling actor_system.actor_of(props, name) will actually call user_actor.actor_context().actor_of(props, name).

This actor is the one responsible for creating the thread pool that will handle the user actors.
This is done this way so that each hierarchy can spawn its own actors, and not having a shared pool
for all actors, this way the system actors (in the system actor's hierarchy) can have there own
thread pool to handle sockets (for remoting), dead letters (for logging), path queries (for getting
an ActorRef from a path), etc...

### System actor

Not to mistake with actor system :p

Having an actor to manage the system related actions allows us to use the power of actors while
implementing actors, thus the event stream is an actor, the DeadMessages mailbox is an actor and
the part responsible for sending messages accross machines is also an actor (with its dedicated
thread pool, as network operations tend to be blocking).

## Futures

Let the fun begin !

## Sending messages accross actors

The main (and recommanded) way to have actors commmunicate with each other is the use of the tell
function on actor references.

If an anwser is desired and that it cannot be done by having it send with a send, the ask pattern
can be used.

### tell

Tell is a way to send a message to an actor with its actor ref, the message is received at most
once.

The pattern is the following:

```rust
actor_1 = actors_system.actor_of(Props::Myactor::new(), "1");
actor_2 = actors_system.actor_of(Props::Myactor::new(), "2");

actor_1.tell(actor_2, "Hello");
```

Or the following:

```rust
impl Actor for MyActor {
    fn receive(message: Message, context: &Context) {
        match message {
            _ => context.tell(context.sender(), "Hello friend")
        }
    }
}
```

Note that we have to go through the actor context  compared to Scala, because there are no implicits
in rust, and thus an Actor cannot be implitly casted into an ActorRef.

Having done that, the message will be enqueud in the receiving actor's mailbox, and the actor will
be enqueud in the pool actors with message to handle.

### ask

The ask patter allows to get an answer from an actor in the form of a future (which can act as
actor refs)

## Actors addressing

path and non local stuff.

## Remoting

Actors have TCP socket, everything is actors.
