use crate::color::{Color, IGNORE_COLOR};
use crate::sprite::Sprite;
use std::ops::Index;

pub struct SpriteSheet {
    sheet: Vec<Vec<Sprite>>,
}

impl SpriteSheet {
    ///Splits a large sprite into a sprite sheet of 32x32
    pub fn split_sprite(orig: Sprite) -> SpriteSheet {
        let height = orig.height();
        let width = orig.width();
        let mut set = SpriteSheet { sheet: vec![] };
        for i in (0..height).step_by(32) {
            let mut temp = vec![];
            for j in (0..width).step_by(32) {
                let mut temp_2 = Sprite {
                    img: vec![vec![Color::ignore(); 32]; 32],
                };
                if square_cpy(&orig, &mut temp_2, i, j) {
                    temp.push(temp_2);
                }
            }
            set.sheet.push(temp);
        }
        println!("{}  {}", set.sheet.len(), set.sheet[0].len());
        set
    }
}

impl Index<usize> for SpriteSheet {
    type Output = Vec<Sprite>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sheet[index]
    }
}

///Copy from orig at i,j to origin of new; Returns if copy did anything
pub fn square_cpy(orig: &Sprite, new: &mut Sprite, i: usize, j: usize) -> bool {
    let mut significant = false;
    for x in 0..32 {
        for y in 0..32 {
            let co = &orig.img[i + x][j + y];
            if !co.ignore {
                significant = true;
                new.img[x][y] = (*co).clone();
            }
        }
    }
    significant
}
