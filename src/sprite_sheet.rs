use std::ops::{Index};
use crate::sprite::Sprite;

pub struct SpriteSheet {
    sheet: Vec<Vec<Sprite>>,
}

impl SpriteSheet {
    pub fn split_sprite(orig: Sprite) -> SpriteSheet {
        let height = orig.height();
        let width = orig.width();
        let mut set = SpriteSheet { sheet: vec![] };
        for i in (0..height).step_by(32) {
            let mut temp = vec![];
            for j in (0..width).step_by(32) {
                let mut temp_2 = Sprite{ img: vec![vec![0xaabbcc;32];32] };
                square_cpy(&orig, &mut temp_2, i, j);
                temp.push(temp_2);
            }
            set.sheet.push(temp);
        }
        set
    }
}

impl Index<usize> for SpriteSheet {
    type Output = Vec<Sprite>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sheet[index]
    }
}

pub fn square_cpy(orig: &Sprite, new: &mut Sprite, i: usize, j: usize) {
    for x in 0..32 {
        for y in 0..32 {
            let co = orig.img[i + x][j + y];
            if co != 0 {
                new.img[x][y] = co;
            }
        }
    }
}