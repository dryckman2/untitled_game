use crate::sprite::Sprite;

#[derive(Clone)]
pub struct Tile {
    pub(crate) background: Sprite,
    pub(crate) vis_queue: Vec<Sprite>,
}

impl Tile {
    pub fn place(&mut self, to_place: Sprite) {
        self.vis_queue.push(to_place);
    }
    pub fn remove(&mut self) {
        self.vis_queue.pop();
    }

    pub fn vis(&self) -> Sprite {
        let mut lay = self.background.clone();
        for vis in self.vis_queue.iter() {
            for (i, col) in vis.iter().enumerate() {
                for (j, px) in col.iter().enumerate() {
                    if *px != 0xaabbcc {
                        *lay.get_mut(i).get_mut(j).unwrap() = *px;
                    }
                }
            }
        }
        lay
    }
}
