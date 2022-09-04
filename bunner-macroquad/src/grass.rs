use crate::{
    child::Child, hedge::Hedge, hedge_mask::HedgeMask, hedge_row::HedgeRow, hedge_tile::HedgeTile,
    position::Position, resources::Resources, road::Road, row::Row, water::Water, ROW_HEIGHT,
    WIDTH,
};
use macroquad::{
    audio::play_sound_once,
    prelude::collections::storage,
    rand::{self},
    texture::Texture2D,
};

#[derive(Clone)]
pub struct Grass {
    index: i32,
    y: i32,
    hedge_row: HedgeRow,
    hedge_mask: Vec<HedgeMask>,
    children: Vec<Child>,
}

impl Row for Grass {
    fn y(&self) -> i32 {
        self.y
    }

    fn children(&self) -> &[Child] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Child> {
        self.children.as_mut()
    }

    fn image(&self) -> Texture2D {
        *storage::get::<Resources>()
            .grass_textures
            .get(self.index as usize)
            .unwrap()
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().grass_sound);
    }

    fn next(&self) -> Box<dyn Row> {
        let y = self.y - ROW_HEIGHT;
        if self.index <= 5 {
            self.grass_row(self.index + 8, y)
        } else if self.index == 6 {
            self.grass_row(7, y)
        } else if self.index == 7 {
            self.grass_row(15, y)
        } else if self.index >= 8 && self.index <= 14 {
            self.grass_row(self.index + 1, y)
        } else if rand::gen_range::<u8>(0, 2) == 0 {
            Box::new(Road::empty(y))
        } else {
            Box::new(Water::empty(y))
        }
    }

    fn allow_movement(&self, x: i32) -> bool {
        (16..=WIDTH - 16).contains(&x) && !self.collide(x, 8)
    }

    fn push(&self) -> i32 {
        0
    }
}

impl Grass {
    pub fn new(
        previous_hedge_mask: Option<Vec<HedgeMask>>,
        previous_hedge_row: HedgeRow,
        index: i32,
        y: i32,
    ) -> Self {
        let (hedge_mask, hedge_row) = match previous_hedge_mask {
            Some(_) if previous_hedge_row == HedgeRow::None => Self::first_hedge_row(index),
            Some(mask) if previous_hedge_row == HedgeRow::First => (mask, HedgeRow::Second),
            Some(_) => (Vec::new(), HedgeRow::None),
            None => Self::first_hedge_row(index),
        };

        let mut children: Vec<Child> = Vec::new();
        if hedge_row != HedgeRow::None {
            // See comments in classify_hedge_segment for explanation of previous_mid_segment
            #[allow(unused_assignments)]
            let mut hedge_tile = HedgeTile::Grass;
            let mut previous_mid_segment = None;
            for i in 1..13 {
                (hedge_tile, previous_mid_segment) =
                    Self::classify_hedge_segment(&hedge_mask[i - 1..i + 2], previous_mid_segment);
                if hedge_tile != HedgeTile::Grass {
                    children.push(Child::Hedge(Hedge::new(
                        hedge_tile,
                        hedge_row,
                        Position::new(i as i32 * 40 - 20, 0),
                    )));
                }
            }
        }

        Self {
            y,
            index,
            hedge_row,
            hedge_mask,
            children,
        }
    }

    pub fn without_hedge(index: i32, y: i32) -> Self {
        Self::new(None, HedgeRow::None, index, y)
    }

    pub fn classify_hedge_segment(
        mask_window: &[HedgeMask],
        previous_mid_segment: Option<HedgeTile>,
    ) -> (HedgeTile, Option<HedgeTile>) {
        if mask_window[1] == HedgeMask::Empty {
            (HedgeTile::Grass, None)
        } else if mask_window[0] == HedgeMask::Empty && mask_window[2] == HedgeMask::Empty {
            (HedgeTile::SingleWidth, None)
        } else if mask_window[0] == HedgeMask::Empty {
            (HedgeTile::LeftMost, None)
        } else if mask_window[2] == HedgeMask::Empty {
            (HedgeTile::RightMost, None)
        } else {
            match previous_mid_segment {
                Some(HedgeTile::Middle4) if mask_window[2] == HedgeMask::Empty => {
                    (HedgeTile::Middle5, None)
                }
                Some(HedgeTile::Middle4) if mask_window[2] == HedgeMask::Hedge => {
                    (HedgeTile::Middle3, Some(HedgeTile::Middle3))
                }
                Some(HedgeTile::Middle3) => (HedgeTile::Middle3, Some(HedgeTile::Middle3)),
                _ => (HedgeTile::Middle3, Some(HedgeTile::Middle3)),
            }
        }
    }

    pub fn first_hedge_row(index: i32) -> (Vec<HedgeMask>, HedgeRow) {
        if rand::gen_range::<u8>(0, 1) == 0 && index > 7 && index < 14 {
            (Self::generate_hedge_mask(), HedgeRow::First)
        } else {
            (Vec::new(), HedgeRow::None)
        }
    }

    pub fn generate_hedge_mask() -> Vec<HedgeMask> {
        let mut mask = Vec::new();
        mask.resize_with(12, || {
            if rand::gen_range::<u8>(1, 100) > 1 {
                HedgeMask::Hedge
            } else {
                HedgeMask::Empty
            }
        });
        // Ensure there is at least one gap
        let _ = std::mem::replace(&mut mask[rand::gen_range(0, 11)], HedgeMask::Empty);

        let mut new_mask = Vec::with_capacity(12);
        for i in 0..12 {
            let low_index = 0.max(i as i32 - 1) as usize;
            let high_index = 11.min(i + 1);
            new_mask.push(
                if mask[low_index..=high_index]
                    .iter()
                    .filter(|&&item| item == HedgeMask::Empty)
                    .count()
                    > 0
                {
                    HedgeMask::Empty
                } else {
                    HedgeMask::Hedge
                },
            );
        }

        // Duplicate first and last elements
        let mut mask = Vec::new();
        mask.push(*new_mask.get(0).unwrap());
        mask.extend(new_mask.clone());
        mask.push(new_mask.pop().unwrap());

        mask
    }

    fn grass_row(&self, index: i32, y: i32) -> Box<dyn Row> {
        Box::new(Grass::new(
            Some(self.hedge_mask.clone()),
            self.hedge_row,
            index,
            y,
        ))
    }
}
