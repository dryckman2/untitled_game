use crate::character::CharStatus::{ATTACKING, IDLING};
use crate::file_utils::load_png_file;
use crate::get_unwrap;
use crate::sprite::Sprite;
use crate::sprite_sheet::SpriteSheet;
use crate::tile::Direction::{East, North, South, West};
use crate::tile::{Direction, Tile};
use crate::traits::Move;

pub struct Character {
    x_pos: usize,
    y_pos: usize,
    img: SpriteSheet,
    ticks: usize,
    current_sprite: Sprite,
    idle_cycle: Vec<(usize, usize)>,
    direction: Direction,
    left_flip: bool,
    status: CharStatus,
}
#[derive(PartialEq)]
pub enum CharStatus {
    IDLING,
    ATTACKING,
}

impl Character {
    pub fn main() -> Character {
        let img = SpriteSheet::split_sprite(load_png_file("txr/character.png"));
        let start = img.get((1, 1)).clone();
        Character {
            x_pos: 10,
            y_pos: 10,
            img,
            ticks: 0,
            current_sprite: start,
            idle_cycle: vec![(0, 0), (1, 0), (1, 1)],
            direction: East,
            left_flip: false,
            status: IDLING,
        }
    }

    ///TODO: Change this to spawn a wave of enemies
    pub fn enemy(i: usize, j: usize) -> Character {
        let img = SpriteSheet::split_sprite(load_png_file("txr/enemy.png"));
        let start = img.get((0, 0)).clone();
        Character {
            x_pos: i,
            y_pos: j,
            img,
            ticks: 0,
            current_sprite: start,
            idle_cycle: vec![(0, 0), (0, 1)],
            direction: East,
            left_flip: false,
            status: IDLING,
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
    pub fn tick_character(&mut self, gb: &mut Vec<Vec<Tile>>) {
        get_unwrap(gb, self.x_pos, self.y_pos).remove();
        match self.status {
            IDLING => {
                self.current_sprite = self
                    .img
                    .get(self.idle_cycle[(self.ticks / 7) % self.idle_cycle.len()])
                    .clone();
            }
            ATTACKING => {
                if self.ticks == 8 {
                    self.status = IDLING;
                    self.ticks = 0;
                } else {
                    self.current_sprite = self.img.get((8, self.ticks)).clone();
                }
            }
        }
        self.ticks = self.ticks + 1;
        if self.direction == West {
            self.current_sprite.reverse();
        }
        get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
    }

    pub fn attack(&mut self) {
        if self.status == IDLING {
            self.status = ATTACKING;
            self.ticks = 0;
        }
    }
}

impl Move for Character {
    fn try_down(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.y_pos != 14 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.y_pos += 1;
            self.direction = South;
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
        }
    }

    fn try_up(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.y_pos != 0 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.y_pos -= 1;
            self.direction = North;
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
        }
    }

    fn try_left(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.x_pos != 0 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.x_pos -= 1;
            if !self.left_flip {
                self.current_sprite.reverse();
                self.left_flip = true;
            }
            self.direction = West;
            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
        }
    }

    fn try_right(&mut self, gb: &mut Vec<Vec<Tile>>) {
        if self.x_pos != 19 {
            get_unwrap(gb, self.x_pos, self.y_pos).remove();
            self.x_pos += 1;
            if self.left_flip {
                self.current_sprite.reverse();
                self.left_flip = false;
            }
            self.direction = East;

            get_unwrap(gb, self.x_pos, self.y_pos).place(self.clone_curr().clone());
        }
    }
}
