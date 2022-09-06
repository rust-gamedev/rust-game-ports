use crate::{actor::Actor, car::Car, hedge::Hedge, log::Log, splat::Splat, train::Train};

#[derive(Clone)]
pub enum Child {
    Hedge(Hedge),
    Train(Train),
    Splat(Splat),
    Car(Car),
    Log(Log),
}

impl Child {
    pub fn update(&mut self) {
        match self {
            Child::Hedge(hedge) => hedge.update(),
            Child::Train(train) => train.update(),
            Child::Splat(splat) => splat.update(),
            Child::Car(car) => car.update(),
            Child::Log(log) => log.update(),
        }
    }

    pub fn draw(&self, x: i32, y: i32) {
        match self {
            Child::Hedge(hedge) => hedge.draw(x, y),
            Child::Train(train) => train.draw(x, y),
            Child::Splat(splat) => splat.draw(x, y),
            Child::Car(car) => car.draw(x, y),
            Child::Log(log) => log.draw(x, y),
        }
    }

    pub fn x(&self) -> i32 {
        match self {
            Child::Hedge(hedge) => hedge.x(),
            Child::Train(train) => train.x(),
            Child::Splat(splat) => splat.x(),
            Child::Car(car) => car.x(),
            Child::Log(log) => log.x(),
        }
    }

    pub fn width(&self) -> i32 {
        match self {
            Child::Hedge(hedge) => hedge.width(),
            Child::Train(train) => train.width(),
            Child::Splat(splat) => splat.width(),
            Child::Car(car) => car.width(),
            Child::Log(log) => log.width(),
        }
    }
}
