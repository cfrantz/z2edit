use crate::apu::ApuDmc;
use crate::apu::ApuNoise;
use crate::apu::ApuPulse;
use crate::apu::ApuTriangle;
use crate::mapper::mmc5::MMC5;
use crate::mapper::vrc7::{OplDebug, Vrc7};
use crate::system::Nes;

#[derive(Clone, Debug)]
pub struct ApuDebug {
    pub visible: bool,
}

impl ApuDebug {
    pub fn new() -> Self {
        ApuDebug { visible: false }
    }

    fn draw_pulse(ui: &imgui::Ui, pulse: &mut ApuPulse, name: &str) {
        ui.plot_lines(format!("##{name}"), &pulse.debug_buf)
            .overlay_text(&name)
            .values_offset(pulse.debug_idx)
            .scale_min(0.0)
            .scale_max(1.0)
            .graph_size([0.0, 128.0])
            .build();
        ui.same_line();
        ui.group(|| {
            ui.text(format!("Control: {:02x}", pulse.reg.control));
            ui.text(format!("Sweep:   {:02x}", pulse.reg.sweep));
            ui.text(format!(
                "Timer:   {:02x}{:02x}",
                pulse.reg.timer_hi, pulse.reg.timer_lo
            ));
        });
        ui.slider_config(&name, 0.0f32, 1.0f32)
            .display_format("%.02f")
            .build(&mut pulse.channel_volume);
    }

    fn draw_triangle(ui: &imgui::Ui, triangle: &mut ApuTriangle) {
        ui.plot_lines("##triangle", &triangle.debug_buf)
            .overlay_text("Triangle")
            .values_offset(triangle.debug_idx)
            .scale_min(0.0)
            .scale_max(1.0)
            .graph_size([0.0, 128.0])
            .build();
        ui.same_line();
        ui.group(|| {
            ui.text(format!("Control: {:02x}", triangle.reg.control));
            ui.text(format!(
                "Timer:   {:02x}{:02x}",
                triangle.reg.timer_hi, triangle.reg.timer_lo
            ));
        });
        ui.slider_config("Triangle", 0.0f32, 1.0f32)
            .display_format("%.02f")
            .build(&mut triangle.channel_volume);
    }

    fn draw_noise(ui: &imgui::Ui, noise: &mut ApuNoise) {
        ui.plot_lines("##noise", &noise.debug_buf)
            .overlay_text("Noise")
            .values_offset(noise.debug_idx)
            .scale_min(0.0)
            .scale_max(1.0)
            .graph_size([0.0, 128.0])
            .build();
        ui.same_line();
        ui.group(|| {
            ui.text(format!("Control: {:02x}", noise.reg.control));
            ui.text(format!("Period:  {:02x}", noise.reg.period));
            ui.text(format!("Length:  {:02x}", noise.reg.length));
        });
        ui.slider_config("Noise", 0.0f32, 1.0f32)
            .display_format("%.02f")
            .build(&mut noise.channel_volume);
    }

    fn draw_dmc(ui: &imgui::Ui, dmc: &mut ApuDmc) {
        ui.plot_lines("##dmc", &dmc.debug_buf)
            .overlay_text("DMC")
            .values_offset(dmc.debug_idx)
            .scale_min(0.0)
            .scale_max(1.0)
            .graph_size([0.0, 128.0])
            .build();
        ui.same_line();
        ui.group(|| {
            ui.text(format!("Control: {:02x}", dmc.reg.control));
            ui.text(format!("Value:   {:02x}", dmc.reg.value));
            ui.text(format!("Address: {:02x}", dmc.reg.address));
            ui.text(format!("Length:  {:02x}", dmc.reg.length));
        });
        ui.slider_config("DMC", 0.0f32, 1.0f32)
            .display_format("%.02f")
            .build(&mut dmc.channel_volume);
    }

    fn draw_opl(ui: &imgui::Ui, opl: &mut OplDebug, name: &str) {
        ui.plot_lines(format!("##{name}"), &opl.debug_buf)
            .overlay_text(&name)
            .values_offset(opl.debug_idx)
            .scale_min(-1.0)
            .scale_max(1.0)
            .graph_size([0.0, 128.0])
            .build();
        ui.same_line();
        ui.group(|| {
            ui.text(format!("Volume:    {:02x}", opl.volume));
            ui.text(format!("Frequency: {:02x}{:02x}", opl.flo, opl.fhi,));
        });
        ui.slider_config(&name, 0.0f32, 1.0f32)
            .display_format("%.02f")
            .build(&mut opl.channel_volume);
    }

    pub fn draw(&mut self, nes: &Nes, ui: &imgui::Ui) {
        if !self.visible {
            return;
        }
        let mut visible = self.visible;
        ui.window("Audio").opened(&mut visible).build(|| {
            let mut apu = nes.apu.lock().expect("apu debug");
            ApuDebug::draw_pulse(ui, &mut apu.pulse0, "Pulse 0");
            ApuDebug::draw_pulse(ui, &mut apu.pulse1, "Pulse 1");
            ApuDebug::draw_triangle(ui, &mut apu.triangle);
            ApuDebug::draw_noise(ui, &mut apu.noise);
            ApuDebug::draw_dmc(ui, &mut apu.dmc);

            let mut mapper = nes.mapper.lock().expect("mapper audio");
            if let Some(mmc5) = mapper.as_any_mut().downcast_mut::<MMC5>() {
                ApuDebug::draw_pulse(ui, &mut mmc5.pulse[0], "MMC5 Pulse 0");
                ApuDebug::draw_pulse(ui, &mut mmc5.pulse[1], "MMC5 Pulse 1");
            } else if let Some(vrc7) = mapper.as_any_mut().downcast_mut::<Vrc7>() {
                for (i, opl) in vrc7.opl_debug.iter_mut().enumerate() {
                    ApuDebug::draw_opl(ui, opl, &format!("VRC7 FM Synth {i}"));
                }
            }
        });
        self.visible = visible;
    }
}
