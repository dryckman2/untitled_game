use std::slice::Iter;

#[derive(Clone)]
pub struct Sprite {
    pub(crate) img: Vec<Vec<u32>>,
}

impl Sprite {
    pub fn blank() -> Sprite {
        Sprite { img: vec![vec![0xaabbcc;0];0] }
    }

    pub fn push(&mut self, tp: Vec<u32>) {
        self.img.push(tp);
    }

    pub fn iter(&self) -> Iter<'_, Vec<u32>> {
        self.img.iter()
    }

    pub fn get_mut(&mut self, i: usize) -> &mut Vec<u32> {
        self.img.get_mut(i).unwrap()
    }
    pub fn height(&self) -> usize {
        self.img.len()
    }
    pub fn width(&self)->usize{
        self.img[0].len()
    }
}

