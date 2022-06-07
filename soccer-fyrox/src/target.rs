use crate::prelude::*;

pub trait Target: MyActor {
    fn active(&self, ball: &Ball) -> bool;
    fn team(&self) -> u8;
}
