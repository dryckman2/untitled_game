use crate::sprite::Sprite;

#[derive(Clone)]
pub struct Tile {
    pub(crate) background: Sprite,
    pub(crate) vis_queue: Vec<Sprite>,
}

#[derive(PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Tile {
    ///Place sprite in visibility queue
    pub fn place(&mut self, to_place: Sprite) {
        self.vis_queue.push(to_place);
    }
    ///Remove sprite form visibility queue, might remove the wrong one
    pub fn remove(&mut self) {
        self.vis_queue.pop();
    }

    ///Collapse tile to single sprite with vis_queue on top of background
    pub fn vis(&self) -> Sprite {
        let mut lay = self.background.clone();
        for vis in self.vis_queue.iter() {
            for (i, col) in vis.iter().enumerate() {
                for (j, px) in col.iter().enumerate() {
                    if !px.ignore {
                        *lay.get_mut(i).get_mut(j).unwrap() = (*px).clone();
                    }
                }
            }
        }
        lay
    }
}
