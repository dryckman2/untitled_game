use crate::sprite::Sprite;
use image::Pixel;

pub fn load_png_file(filename: &str) -> Sprite {
    let img = image::open(filename).expect("File not found!");
    let mut txr = Sprite::blank();
    let b_img = img.into_rgb32f();
    for i in 0..b_img.height() {
        let mut temp = vec![];
        for j in 0..b_img.width() {
            let c = b_img[(j, i)].channels();
            let color = ((c[0] * 255.0) as u32) << 16
                | ((c[1] * 255.0) as u32) << 8
                | ((c[2] * 255.0) as u32);
            temp.push(color);
        }
        txr.push(temp.clone());
    }
    txr
}
