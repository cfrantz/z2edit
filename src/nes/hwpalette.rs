// RGB constants for NES colors.
use once_cell::sync::Lazy;

pub const PALETTE: [u32; 64] = [
    // ABGR
    0xff666666, 0xff882a00, 0xffa71214, 0xffa4003b, 0xff7e005c, 0xff40006e, 0xff00066c, 0xff001d56,
    0xff003533, 0xff00480b, 0xff005200, 0xff084f00, 0xff4d4000, 0xff000000, 0xff000000, 0xff000000,
    0xffadadad, 0xffd95f15, 0xffff4042, 0xfffe2775, 0xffcc1aa0, 0xff7b1eb7, 0xff2031b5, 0xff004e99,
    0xff006d6b, 0xff008738, 0xff00930c, 0xff328f00, 0xff8d7c00, 0xff000000, 0xff000000, 0xff000000,
    0xfffffeff, 0xffffb064, 0xffff9092, 0xffff76c6, 0xffff6af3, 0xffcc6efe, 0xff7081fe, 0xff229eea,
    0xff00bebc, 0xff00d888, 0xff30e45c, 0xff82e045, 0xffdecd48, 0xff4f4f4f, 0xff000000, 0xff000000,
    0xfffffeff, 0xffffdfc0, 0xffffd2d3, 0xffffc8e8, 0xffffc2fb, 0xffeac4fe, 0xffc5ccfe, 0xffa5d8f7,
    0xff94e5e4, 0xff96efcf, 0xffabf4bd, 0xffccf3b3, 0xfff2ebb5, 0xffb8b8b8, 0xff000000, 0xff000000,
];

pub static FPALETTE: Lazy<Vec<[f32; 4]>> = Lazy::new(|| {
    PALETTE.iter().map(|x| color_as_f32(*x)).collect()
});

const R_SHIFT: usize = 0;
const G_SHIFT: usize = 8;
const B_SHIFT: usize = 16;
const A_SHIFT: usize = 24;

fn color_as_f32(color: u32) -> [f32; 4] {
    [
        ((color >> R_SHIFT) & 0xFF) as f32 / 255.0,
        ((color >> G_SHIFT) & 0xFF) as f32 / 255.0,
        ((color >> B_SHIFT) & 0xFF) as f32 / 255.0,
        ((color >> A_SHIFT) & 0xFF) as f32 / 255.0,
    ]
}

pub fn get(index: usize) -> u32 {
    // Out-of-bounds index returns black.
    let index = if index < PALETTE.len() { index } else { 0x0f };
    PALETTE[index]
}

pub fn fget(index: usize) -> [f32; 4] {
    // Out-of-bounds index returns black.
    let index = if index < PALETTE.len() { index } else { 0x0f };
    FPALETTE[index]
}
