pub const IGNORE_COLOR: u32 = 0xAABBCCDD;

#[derive(Clone)]
pub struct Color {
    pub(crate) c: u32,
    pub(crate) ignore: bool,
}

impl Color {
    pub fn from_f32_channel(channels: &[f32]) -> Color {
        let c = ((channels[0] * 255.0) as u32) << 16
            | ((channels[1] * 255.0) as u32) << 8
            | ((channels[2] * 255.0) as u32);
        let a = channels[3] == 0.0;
        Color { c, ignore: a }
    }

    pub fn ignore() -> Color {
        Color {
            c: IGNORE_COLOR,
            ignore: true,
        }
    }
}
