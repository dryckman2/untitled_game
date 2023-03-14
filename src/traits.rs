use crate::tile::Tile;

pub trait Move {
    fn try_down(&mut self, gb: &mut Vec<Vec<Tile>>);
    fn try_up(&mut self, gb: &mut Vec<Vec<Tile>>);
    fn try_left(&mut self, gb: &mut Vec<Vec<Tile>>);
    fn try_right(&mut self, gb: &mut Vec<Vec<Tile>>);
}
