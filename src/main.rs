mod character;
mod file_utils;
mod sprite;
mod tile;
mod traits;
mod sprite_sheet;

use crate::character::Character;
use crate::file_utils::load_png_file;
use crate::tile::Tile;
use crate::traits::Move;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::thread::sleep;
use std::time::Duration;
use std::vec;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let background = Tile {
        background: load_png_file("txr/grass_txr.png"),
        vis_queue: vec![],
    };
    let mut game_board: Vec<Vec<Tile>> = vec![vec![background; HEIGHT / 32]; WIDTH / 32];
    let mut ch = Character::create();

    ch.place(&mut game_board);

    let mut window =
        Window::new("Untitled 2d Game", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::S, KeyRepeat::Yes) {
            ch.try_down(&mut game_board);
        }
        if window.is_key_pressed(Key::W, KeyRepeat::Yes) {
            ch.try_up(&mut game_board);
        }
        if window.is_key_pressed(Key::A, KeyRepeat::Yes) {
            ch.try_left(&mut game_board);
        }
        if window.is_key_pressed(Key::D, KeyRepeat::Yes) {
            ch.try_right(&mut game_board);
        }
        if window.is_key_pressed(Key::Space, KeyRepeat::Yes) {
            ch.change_curr(&mut game_board);
        }

        let buffer: Vec<u32> = produce_buffer(&game_board);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        sleep(Duration::from_millis(16));
    }
}

fn produce_buffer(board: &Vec<Vec<Tile>>) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * WIDTH];

    for i in 0..(HEIGHT / 32) {
        for j in 0..(WIDTH / 32) {
            let str = &board[j][i].vis();
            let offset = 32 * (i * WIDTH + j);
            for (y, col) in str.iter().enumerate() {
                for (x, pixel) in col.iter().enumerate() {
                    buffer[offset + y * WIDTH + x] = *pixel;
                }
            }
            buffer[offset] = 0xffffff00 //TODO: Remove, Creates Grid Markers
        }
    }

    buffer
}

fn get_unwrap(n: &mut Vec<Vec<Tile>>, i: usize, j: usize) -> &mut Tile {
    n.get_mut(i).unwrap().get_mut(j).unwrap()
}