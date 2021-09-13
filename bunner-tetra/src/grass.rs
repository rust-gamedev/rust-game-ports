use crate::{row::Row, row_type::RowType};

pub struct Grass {
    _base_image: &'static str,
    _index: i32,
    _y: u32,
    _hedge_row_index: Option<bool>,
    _hedge_mask: Vec<bool>,
}

impl Grass {
    pub fn new(_predecessor: Option<RowType>, _index: i32, _y: u32) -> Self {
        Self {
            _base_image: "grass",
            _index,
            _y,
            _hedge_row_index: None,
            _hedge_mask: vec![],
        }
    }
}

impl Row for Grass {}
