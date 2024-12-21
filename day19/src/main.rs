// 1. We have 3 states:
// - Empty
// - Ready
// - Flying

use std::marker::PhantomData;

pub struct Empty {}
pub struct Ready {}
pub struct Flying {}

// 2. Finish the Seligh struct definition
pub struct Sleigh<State> {
    state: PhantomData<State>,
}


impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }

    pub fn load(&self) -> Sleigh<Ready>{
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(&self) -> Sleigh<Flying> {
        Sleigh { state: PhantomData }
    }

    pub fn unload(&self) -> Sleigh<Empty> {
        Sleigh { state: PhantomData}
    }
}

impl Sleigh<Flying> {
    pub fn land(&self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData}
    }
}


// 3. Write the Sleigh Implementations for all states

fn main() {
    let sleigh = Sleigh::new();
    let loaded_sleigh = sleigh.load();
    let flying_sleigh = loaded_sleigh.take_off();
    let ready_sleigh = flying_sleigh.land();
    let unloaded_sleigh = loaded_sleigh.unload();

    assert!(sleigh.state == PhantomData::<Empty>);
    assert!(loaded_sleigh.state == PhantomData::<Ready>);
    assert!(flying_sleigh.state == PhantomData::<Flying>);
    assert!(ready_sleigh.state == PhantomData::<Ready>);
    assert!(unloaded_sleigh.state == PhantomData::<Empty>);
}
