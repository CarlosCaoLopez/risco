// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Felt tip on paper, synthesised: band-passed noise whose loudness follows
//! the pointer speed, plus fibre grain, and a cap click when the stroke ends.

use rodio::Source;
use std::num::NonZero;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::time::Duration;

const RATE: u32 = 44_100;
/// Pointer speed (px/s) that gives full loudness.
const FULL_SPEED: f32 = 1400.0;

enum Cmd {
    Start { volume: f32 },
    Speed(f32),
    Stop { ok: bool },
}

/// Handle to the audio thread; cheap to clone.
#[derive(Clone)]
pub struct Sound {
    tx: Sender<Cmd>,
}

impl Sound {
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::channel::<Cmd>();
        std::thread::Builder::new()
            .name("sound".into())
            .spawn(move || {
                // Opened lazily so a machine without audio still runs the app.
                let mut sink: Option<rodio::MixerDeviceSink> = None;
                let mut stroke: Option<Arc<Params>> = None;
                let mut volume = 1.0f32;
                for cmd in rx {
                    if sink.is_none() {
                        sink = rodio::DeviceSinkBuilder::open_default_sink()
                            .map(|mut s| {
                                s.log_on_drop(false);
                                s
                            })
                            .ok();
                    }
                    let Some(out) = &sink else { continue };
                    match cmd {
                        Cmd::Start { volume: v } => {
                            volume = v.clamp(0.0, 1.0);
                            if let Some(old) = stroke.take() {
                                old.running.store(false, Ordering::Relaxed);
                            }
                            let params = Arc::new(Params::default());
                            out.mixer().add(Scratch::new(params.clone()));
                            stroke = Some(params);
                        }
                        Cmd::Speed(v) => {
                            if let Some(p) = &stroke {
                                let level = (v / FULL_SPEED).clamp(0.0, 1.0).powf(0.7) * volume;
                                p.level.store(level.to_bits(), Ordering::Relaxed);
                            }
                        }
                        Cmd::Stop { ok } => {
                            if let Some(p) = stroke.take() {
                                p.running.store(false, Ordering::Relaxed);
                            }
                            if ok {
                                out.mixer().add(CapClick::new(volume));
                            }
                        }
                    }
                }
            })
            .expect("spawn sound thread");
        Self { tx }
    }

    pub fn start(&self, volume: f32) {
        let _ = self.tx.send(Cmd::Start { volume });
    }

    pub fn speed(&self, px_per_sec: f32) {
        let _ = self.tx.send(Cmd::Speed(px_per_sec));
    }

    pub fn stop(&self, ok: bool) {
        let _ = self.tx.send(Cmd::Stop { ok });
    }

    /// A short stroke for the settings window.
    pub fn demo(&self, volume: f32) {
        let s = self.clone();
        std::thread::spawn(move || {
            s.start(volume);
            for i in 0..40 {
                let t = i as f32 / 40.0;
                s.speed(FULL_SPEED * (0.4 + 0.6 * (t * std::f32::consts::PI).sin()));
                std::thread::sleep(Duration::from_millis(14));
            }
            s.stop(true);
        });
    }
}

#[derive(Default)]
struct Params {
    /// Target loudness 0..1, as f32 bits.
    level: AtomicU32,
    running: AtomicBool,
}

struct Rng(u32);

impl Rng {
    fn next(&mut self) -> f32 {
        // xorshift32
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}

/// RBJ band-pass biquad.
struct BandPass {
    b0: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BandPass {
    fn new(freq: f32, q: f32) -> Self {
        let w0 = 2.0 * std::f32::consts::PI * freq / RATE as f32;
        let alpha = w0.sin() / (2.0 * q);
        let a0 = 1.0 + alpha;
        Self {
            b0: alpha / a0,
            b2: -alpha / a0,
            a1: -2.0 * w0.cos() / a0,
            a2: (1.0 - alpha) / a0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    fn run(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b2 * self.x2 - self.a1 * self.y1 - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }
}

struct Scratch {
    params: Arc<Params>,
    rng: Rng,
    hiss: BandPass,
    body: BandPass,
    amp: f32,
    grain: f32,
    wobble: f32,
}

impl Scratch {
    fn new(params: Arc<Params>) -> Self {
        params.running.store(true, Ordering::Relaxed);
        Self {
            params,
            rng: Rng(0x9e37_79b9),
            hiss: BandPass::new(4200.0, 0.8),
            body: BandPass::new(1300.0, 1.1),
            amp: 0.0,
            grain: 0.0,
            wobble: 0.0,
        }
    }
}

impl Iterator for Scratch {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let running = self.params.running.load(Ordering::Relaxed);
        let target =
            if running { f32::from_bits(self.params.level.load(Ordering::Relaxed)) } else { 0.0 };
        // ~40 ms glide so speed changes never click.
        self.amp += (target - self.amp) * 0.0006;
        if !running && self.amp < 1e-4 {
            return None;
        }
        let n = self.rng.next();
        // Paper fibres: sparse little bumps under the tip.
        if self.rng.next() > 0.9975 {
            self.grain = 1.0;
        }
        self.grain *= 0.993;
        self.wobble += (self.rng.next() * 0.5 - self.wobble) * 0.002;
        let tone = self.hiss.run(n) * 0.9 + self.body.run(n) * 0.55;
        let s = tone * (0.75 + 0.25 * self.wobble + 0.6 * self.grain);
        Some(s * self.amp * 0.5)
    }
}

impl Source for Scratch {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> rodio::ChannelCount {
        NonZero::new(1).unwrap()
    }
    fn sample_rate(&self) -> rodio::SampleRate {
        NonZero::new(RATE).unwrap()
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

/// Two quick plastic ticks: the cap going on and seating.
struct CapClick {
    t: u32,
    volume: f32,
    rng: Rng,
    click: BandPass,
}

impl CapClick {
    fn new(volume: f32) -> Self {
        Self { t: 0, volume, rng: Rng(0x1234_5678), click: BandPass::new(2600.0, 2.5) }
    }
}

impl Iterator for CapClick {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let len = RATE / 8;
        if self.t >= len {
            return None;
        }
        let secs = self.t as f32 / RATE as f32;
        let tick = |start: f32, decay: f32| {
            let d = secs - start;
            if d < 0.0 {
                0.0
            } else {
                (-d * decay).exp()
            }
        };
        let env = tick(0.0, 260.0) + 0.55 * tick(0.045, 340.0);
        let thump = (2.0 * std::f32::consts::PI * 190.0 * secs).sin() * tick(0.0, 70.0) * 0.35;
        let n = self.click.run(self.rng.next());
        self.t += 1;
        Some((n * 2.2 * env + thump) * 0.45 * self.volume)
    }
}

impl Source for CapClick {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> rodio::ChannelCount {
        NonZero::new(1).unwrap()
    }
    fn sample_rate(&self) -> rodio::SampleRate {
        NonZero::new(RATE).unwrap()
    }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_millis(125))
    }
}
