use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus};
use sdl2::AudioSubsystem;
use std::sync::mpsc;

pub struct AudioPlayback {
    receiver: mpsc::Receiver<Vec<f32>>,
}

impl AudioCallback for AudioPlayback {
    type Channel = f32;
    fn callback(&mut self, output: &mut [f32]) {
        match self.receiver.try_recv() {
            Ok(values) => {
                output.copy_from_slice(values.as_slice());
            }
            Err(e) => {
                log::debug!("audio underrun: {e:?}");
                for sample in output.iter_mut() {
                    *sample = 0.0;
                }
            }
        }
    }
}

#[pyclass(unsendable)]
pub struct AudioOut {
    playback: AudioDevice<AudioPlayback>,
    sender: mpsc::SyncSender<Vec<f32>>,
}

impl AudioOut {
    pub fn new(subsys: &AudioSubsystem, freq: i32, channels: u8, samples: u16) -> Result<Self> {
        let want = AudioSpecDesired {
            freq: Some(freq),
            channels: Some(channels),
            samples: Some(samples),
        };
        let (sender, receiver) = mpsc::sync_channel(2);
        let playback = subsys
            .open_playback(None, &want, |_spec| AudioPlayback { receiver })
            .map_err(|e| anyhow!("audio initialization: {e}"))?;
        playback.resume();

        Ok(AudioOut { playback, sender })
    }

    pub fn status(&self) -> AudioStatus {
        self.playback.status()
    }
}

#[pymethods]
impl AudioOut {
    pub fn pause(&self) {
        self.playback.pause();
    }
    pub fn resume(&self) {
        self.playback.resume();
    }
    pub fn play(&self, data: Vec<f32>) -> Result<()> {
        self.sender.send(data)?;
        Ok(())
    }
}
