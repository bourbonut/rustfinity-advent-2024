use std::marker::PhantomData;

pub struct Empty;
pub struct Ready;
pub struct Flying;

// TODO: Define the `status` method for all states

pub struct Sleigh<T: State> {
    // This is only public for testing purposes
    // In real-world scenarios, this should be private
    pub state: PhantomData<T>,
}

impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }

    pub fn load(&self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(&self) -> Sleigh<Flying> {
        Sleigh { state: PhantomData }
    }

    pub fn unload(&self) -> Sleigh<Empty> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Flying> {
    pub fn land(&self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

pub trait State {
    fn status() -> &'static str;
}

impl State for Empty {
    fn status() -> &'static str {
        "Empty"
    }
}

impl State for Ready {
    fn status() -> &'static str {
        "Ready"
    }
}

impl State for Flying {
    fn status() -> &'static str {
        "Flying"
    }
}

impl<T: State> Sleigh<T> {
  pub fn status(&self) -> &'static str {
      T::status()
  }
}

fn main() {
    let empty_sleigh = Sleigh::new();
    let ready_sleigh = empty_sleigh.load();
    let flying_sleigh = ready_sleigh.take_off();

    assert_eq!(empty_sleigh.status(), "Empty");
    assert_eq!(ready_sleigh.status(), "Ready");
    assert_eq!(flying_sleigh.status(), "Flying");
}
