use crate::color::Color;
use crate::sprite::Sprite;
use image::Pixel;

///Loads png file to sprite; Possibly creating an extra large sprite
pub fn load_png_file(filename: &str) -> Sprite {
    let img = image::open(filename).expect("File not found!");
    let mut txr = Sprite::blank();
    let b_img = img.into_rgba32f();
    for i in 0..b_img.height() {
        let mut temp = vec![];
        for j in 0..b_img.width() {
            let p = b_img[(j, i)];
            let color = Color::from_f32_channel(p.channels());
            temp.push(color);
        }
        txr.push(temp.clone());
    }
    txr
}
