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
    ticks: usize,
    current_sprite: Sprite,
    idle_cycle: Vec<(usize, usize)>,
    left_flip: bool,
}

impl Character {
    pub fn create() -> Character {
        let img = SpriteSheet::split_sprite(load_png_file("txr/character.png"));
        let start = img.get((1, 1)).clone();
        Character {
            x_pos: 3,
            y_pos: 2,
            img,
            ticks: 0,
            current_sprite: start,
            idle_cycle: vec![(0, 0), (1, 0), (1, 1)],
            left_flip: false,
        }
    }

    ///Place Character on gb
    pub fn place(&self, gb: &mut Vec<Vec<Tile>>) {
        get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
    }

    ///Clone current sprite for Character
    pub fn clone_curr(&self) -> &Sprite {
        &self.current_sprite
    }

    ///Change the current sprite for character
    pub fn change_curr(&mut self, gb: &mut Vec<Vec<Tile>>) {
        get_unwrap(gb, self.x_pos, self.y_pos).remove();
        self.ticks = self.ticks + 1;
        self.current_sprite = self
            .img
            .get(self.idle_cycle[self.ticks % self.idle_cycle.len()])
            .clone();
        if self.left_flip {
            self.current_sprite.reverse();
        }
        get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
    }
}

impl Move for Character {
    fn try_down(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.y_pos != 14 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.y_pos += 1;
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
        }
    }

    fn try_up(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.y_pos != 0 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.y_pos -= 1;
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
        }
    }

    fn try_left(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.x_pos != 0 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.x_pos -= 1;
            if self.left_flip == false {
                self.current_sprite.reverse();
                self.left_flip = true;
            }
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
        }
    }

    fn try_right(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.x_pos != 19 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.x_pos += 1;
            if self.left_flip == true {
                self.current_sprite.reverse();
                self.left_flip = false;
            }
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
        }
    }
}
