extern crate kagura;

pub mod frame;

pub use frame::frame;

type Messenger<From, To> = Box<dyn FnOnce(From) -> To + 'static>;
type MessengerGen<From, To> = Box<dyn Fn() -> Messenger<From, To>>;
