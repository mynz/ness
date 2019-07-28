extern crate rustness;
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::Vector,
    lifecycle::{State, run},
};

struct Screen;

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen)
    }
}

#[test]
#[ignore]
fn test_app_simple() {
    // main thread でしか動作しない
    run::<Screen>("Hello", Vector::new(320, 240), Default::default());
}
