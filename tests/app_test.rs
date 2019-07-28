extern crate quicksilver;
extern crate rustness;

use quicksilver::{
    geom::Vector,
    lifecycle::{run, State},
    Result,
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
