mod dsa_emu2413;

use dsa_emu2413::OPLL;

pub struct Emu2413 {
    p: *mut OPLL,
}

unsafe impl Send for Emu2413 {}
unsafe impl Sync for Emu2413 {}

impl Default for Emu2413 {
    fn default() -> Self {
        Emu2413::new(3579545, 48000)
    }
}

impl Emu2413 {
    pub const CHANNELS: usize = 10;

    pub fn new(clock: u32, rate: u32) -> Self {
        unsafe {
            let p = dsa_emu2413::OPLL_new(clock, rate);
            dsa_emu2413::OPLL_setChipType(p, 1);
            dsa_emu2413::OPLL_resetPatch(p, 1);
            Emu2413 { p }
        }
    }

    pub fn write_reg(&mut self, reg: u32, val: u8) {
        unsafe {
            dsa_emu2413::OPLL_writeReg(self.p, reg, val);
        }
    }

    pub fn output(&mut self) -> [f32; Self::CHANNELS] {
        unsafe {
            dsa_emu2413::OPLL_calc(self.p);
            let p = &*self.p;
            let mut output = [0.0f32; Self::CHANNELS];
            for i in 0..9 {
                output[i] = p.ch_out[i] as f32 / 2048.0;
            }
            //for i in 9..14 {
            //    output[i] += p.ch_out[i] as f32 / 2048.0;
            //}
            output
        }
    }
}
