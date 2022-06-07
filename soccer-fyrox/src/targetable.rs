use crate::prelude::*;

pub trait Targetable: MyActor {
    fn active(&self, ball: &Ball) -> bool;
    fn team(&self) -> u8;
}
