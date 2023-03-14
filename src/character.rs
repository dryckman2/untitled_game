use crate::file_utils::load_png_file;
use crate::get_unwrap;
use crate::sprite::Sprite;
use crate::sprite_sheet::SpriteSheet;
use crate::tile::Tile;
use crate::traits::Move;

pub struct Character {
    x_pos: usize,
    y_pos: usize,
    img: SpriteSheet,
    curr_sprite: usize,
}

impl Character {
    pub fn create() -> Character {
        let img = SpriteSheet::split_sprite(load_png_file("txr/character.png"));
        Character {
            x_pos: 3,
            y_pos: 2,
            img,
            curr_sprite: 0,
        }
    }

    ///Place Character on gb
    pub fn place(&self, gb: &mut Vec<Vec<Tile>>) {
        get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr());
    }

    ///Clone current sprite for Character
    pub fn clone_curr(&self) -> Sprite {
        self.img[self.curr_sprite][0].clone()
    }

    ///Change the current sprite for character
    pub fn change_curr(&mut self, gb: &mut Vec<Vec<Tile>>) {
        get_unwrap(gb, self.x_pos, self.y_pos).remove();
        self.curr_sprite = (self.curr_sprite + 1) % 4;
        get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr());
    }
}

impl Move for Character {
    fn try_down(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.y_pos != 14 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.y_pos += 1;
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr());
        }
    }

    fn try_up(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.y_pos != 0 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.y_pos -= 1;
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr());
        }
    }

    fn try_left(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.x_pos != 0 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.x_pos -= 1;
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr());
        }
    }

    fn try_right(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.x_pos != 19 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.x_pos += 1;
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr());
        }
    }
}
