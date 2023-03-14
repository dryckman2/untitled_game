use crate::color::{Color, IGNORE_COLOR};
use std::slice::Iter;

#[derive(Clone)]
pub struct Sprite {
    pub(crate) img: Vec<Vec<Color>>,
}

impl Sprite {
    ///Blank Sprite with all ignore values
    pub fn blank() -> Sprite {
        Sprite { img: vec![] }
    }

    pub fn push(&mut self, tp: Vec<Color>) {
        self.img.push(tp);
    }

    pub fn iter(&self) -> Iter<'_, Vec<Color>> {
        self.img.iter()
    }

    pub fn get_mut(&mut self, i: usize) -> &mut Vec<Color> {
        self.img.get_mut(i).unwrap()
    }
    pub fn height(&self) -> usize {
        self.img.len()
    }
    pub fn width(&self) -> usize {
        self.img[0].len()
    }
}
