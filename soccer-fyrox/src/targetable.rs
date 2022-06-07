use crate::prelude::*;

pub trait Targetable: MyActor {
    fn active(&self, ball: &Ball) -> bool;
}
