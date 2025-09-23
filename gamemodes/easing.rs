use std::time::{Duration, Instant};
use std::thread::sleep;

/// =================== EASING FUNCTIONS ===================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    Linear,
    // Quad
    InQuad, OutQuad, InOutQuad,
    // Cubic
    InCubic, OutCubic, InOutCubic,
    // Quart
    InQuart, OutQuart, InOutQuart,
    // Quint
    InQuint, OutQuint, InOutQuint,
    // Sine
    InSine, OutSine, InOutSine,
    // Expo
    InExpo, OutExpo, InOutExpo,
    // Circ
    InCirc, OutCirc, InOutCirc,
    // Back
    InBack, OutBack, InOutBack,
    // Elastic
    InElastic, OutElastic, InOutElastic,
    // Bounce
    InBounce, OutBounce, InOutBounce,
}

pub fn ease(kind: Easing, t: f64) -> f64 {
    use Easing::*;
    match kind {
        Linear => linear(t),

        InQuad => in_quad(t), OutQuad => out_quad(t), InOutQuad => in_out_quad(t),

        InCubic => in_cubic(t), OutCubic => out_cubic(t), InOutCubic => in_out_cubic(t),

        InQuart => in_quart(t), OutQuart => out_quart(t), InOutQuart => in_out_quart(t),

        InQuint => in_quint(t), OutQuint => out_quint(t), InOutQuint => in_out_quint(t),

        InSine => in_sine(t), OutSine => out_sine(t), InOutSine => in_out_sine(t),

        InExpo => in_expo(t), OutExpo => out_expo(t), InOutExpo => in_out_expo(t),

        InCirc => in_circ(t), OutCirc => out_circ(t), InOutCirc => in_out_circ(t),

        InBack => in_back(t), OutBack => out_back(t), InOutBack => in_out_back(t),

        InElastic => in_elastic(t), OutElastic => out_elastic(t), InOutElastic => in_out_elastic(t),

        InBounce => in_bounce(t), OutBounce => out_bounce(t), InOutBounce => in_out_bounce(t),
    }
}

#[inline]
fn clamp01(t: f64) -> f64 {
    if t <= 0.0 { 0.0 } else if t >= 1.0 { 1.0 } else { t }
}

#[inline] pub fn linear(t: f64) -> f64 { clamp01(t) }

// Quad
#[inline] pub fn in_quad(t: f64) -> f64 { let t = clamp01(t); t * t }
#[inline] pub fn out_quad(t: f64) -> f64 { let t = clamp01(t); t * (2.0 - t) }
#[inline]
pub fn in_out_quad(t: f64) -> f64 {
    let mut t = clamp01(t) * 2.0;
    if t < 1.0 { 0.5 * t * t } else { t -= 1.0; -0.5 * (t * (t - 2.0) - 1.0) }
}

// Cubic
#[inline] pub fn in_cubic(t: f64) -> f64 { let t = clamp01(t); t * t * t }
#[inline] pub fn out_cubic(t: f64) -> f64 { let t = clamp01(t) - 1.0; t * t * t + 1.0 }
#[inline]
pub fn in_out_cubic(t: f64) -> f64 {
    let mut t = clamp01(t) * 2.0;
    if t < 1.0 { 0.5 * t * t * t } else { t -= 2.0; 0.5 * (t * t * t + 2.0) }
}

// Quart
#[inline] pub fn in_quart(t: f64) -> f64 { let t = clamp01(t); t.powi(4) }
#[inline] pub fn out_quart(t: f64) -> f64 { let t = clamp01(t) - 1.0; 1.0 - t.powi(4) }
#[inline]
pub fn in_out_quart(t: f64) -> f64 {
    let mut t = clamp01(t) * 2.0;
    if t < 1.0 { 0.5 * t.powi(4) } else { t -= 2.0; -0.5 * (t.powi(4) - 2.0) }
}

// Quint
#[inline] pub fn in_quint(t: f64) -> f64 { let t = clamp01(t); t.powi(5) }
#[inline] pub fn out_quint(t: f64) -> f64 { let t = clamp01(t) - 1.0; t.powi(5) + 1.0 }
#[inline]
pub fn in_out_quint(t: f64) -> f64 {
    let mut t = clamp01(t) * 2.0;
    if t < 1.0 { 0.5 * t.powi(5) } else { t -= 2.0; 0.5 * (t.powi(5) + 2.0) }
}

// Sine
#[inline] pub fn in_sine(t: f64) -> f64 { 1.0 - (clamp01(t) * std::f64::consts::FRAC_PI_2).cos() }
#[inline] pub fn out_sine(t: f64) -> f64 { (clamp01(t) * std::f64::consts::FRAC_PI_2).sin() }
#[inline] pub fn in_out_sine(t: f64) -> f64 { -0.5 * ((std::f64::consts::PI * clamp01(t)).cos() - 1.0) }

// Expo
#[inline]
pub fn in_expo(t: f64) -> f64 {
    let t = clamp01(t);
    if t == 0.0 { 0.0 } else { (2.0f64).powf(10.0 * (t - 1.0)) }
}
#[inline]
pub fn out_expo(t: f64) -> f64 {
    let t = clamp01(t);
    if t == 1.0 { 1.0 } else { 1.0 - (2.0f64).powf(-10.0 * t) }
}
#[inline]
pub fn in_out_expo(t: f64) -> f64 {
    let mut t = clamp01(t);
    if t == 0.0 { return 0.0; }
    if t == 1.0 { return 1.0; }
    t *= 2.0;
    if t < 1.0 { 0.5 * (2.0f64).powf(10.0 * (t - 1.0)) } else { 0.5 * (2.0 - (2.0f64).powf(-10.0 * (t - 1.0))) }
}

// Circ
#[inline] pub fn in_circ(t: f64) -> f64 { 1.0 - (1.0 - clamp01(t).powi(2)).sqrt() }
#[inline] pub fn out_circ(t: f64) -> f64 { let t = clamp01(t) - 1.0; (1.0 - t * t).sqrt() }
#[inline]
pub fn in_out_circ(t: f64) -> f64 {
    let mut t = clamp01(t) * 2.0;
    if t < 1.0 { -0.5 * ((1.0 - t * t).sqrt() - 1.0) } else { t -= 2.0; 0.5 * ((1.0 - t * t).sqrt() + 1.0) }
}

// Back
#[inline] pub fn in_back(t: f64) -> f64 { let s = 1.70158; let t = clamp01(t); t * t * ((s + 1.0) * t - s) }
#[inline] pub fn out_back(t: f64) -> f64 { let s = 1.70158; let t = clamp01(t) - 1.0; t * t * ((s + 1.0) * t + s) + 1.0 }
#[inline]
pub fn in_out_back(t: f64) -> f64 {
    let s = 1.70158 * 1.525;
    let mut t = clamp01(t) * 2.0;
    if t < 1.0 { 0.5 * (t * t * ((s + 1.0) * t - s)) } else { t -= 2.0; 0.5 * (t * t * ((s + 1.0) * t + s) + 2.0) }
}

// Elastic
#[inline]
pub fn in_elastic(t: f64) -> f64 {
    let t = clamp01(t);
    if t == 0.0 { return 0.0; }
    if t == 1.0 { return 1.0; }
    let p = 0.3;
    let s = p / 4.0;
    let t = t - 1.0;
    -((2.0f64).powf(10.0 * t)) * ((t - s) * (2.0 * std::f64::consts::PI) / p).sin()
}
#[inline]
pub fn out_elastic(t: f64) -> f64 {
    let t = clamp01(t);
    if t == 0.0 { return 0.0; }
    if t == 1.0 { return 1.0; }
    let p = 0.3;
    let s = p / 4.0;
    (2.0f64).powf(-10.0 * t) * ((t - s) * (2.0 * std::f64::consts::PI) / p).sin() + 1.0
}
#[inline]
pub fn in_out_elastic(t: f64) -> f64 {
    let mut t = clamp01(t) * 2.0;
    if t == 0.0 { return 0.0; }
    if t == 2.0 { return 1.0; }
    let p = 0.45;
    let s = p / 4.0;
    if t < 1.0 {
        let t1 = t - 1.0;
        -0.5 * (2.0f64).powf(10.0 * t1) * ((t1 - s) * (2.0 * std::f64::consts::PI) / p).sin()
    } else {
        let t1 = t - 1.0;
        0.5 * (2.0f64).powf(-10.0 * t1) * ((t1 - s) * (2.0 * std::f64::consts::PI) / p).sin() + 1.0
    }
}

// Bounce
#[inline]
fn out_bounce(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t = t - 1.5 / 2.75;
        7.5625 * t * t + 0.75
    } else if t < 2.5 / 2.75 {
        let t = t - 2.25 / 2.75;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 2.625 / 2.75;
        7.5625 * t * t + 0.984375
    }
}
#[inline] pub fn in_bounce(t: f64) -> f64 { 1.0 - out_bounce(1.0 - clamp01(t)) }
#[inline] pub fn in_out_bounce(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 0.5 { 0.5 * in_bounce(t * 2.0) } else { 0.5 * out_bounce(t * 2.0 - 1.0) + 0.5 }
}

/// =================== ANIMATOR POOL ===================

const MAX_ANIMATORS: usize = 128;

#[derive(Debug, Clone, Copy)]
pub enum AnimatorType {
    Position,
    LetterSize,
    TextSize,
    FullSize,
    Color,
    BoxColor,
    BackgroundColor,
}

#[derive(Debug, Clone)]
pub struct Animator {
    pub player_id: u32,
    pub textdraw_id: u32,
    pub anim_type: AnimatorType,
    pub start_x: f64,
    pub start_y: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub start_color: Option<u32>,   // ARGB-like: r<<24 | g<<16 | b<<8 | a
    pub target_color: Option<u32>,
    pub duration_ms: u64,
    pub start_time: Instant,
    pub easing: Easing,
}

impl Animator {
    pub fn progress(&self) -> f64 {
        let elapsed = self.start_time.elapsed();
        if self.duration_ms == 0 { return 1.0; }
        (elapsed.as_secs_f64() / (self.duration_ms as f64)).min(1.0)
    }

    pub fn is_finished(&self) -> bool {
        self.progress() >= 1.0
    }
}

/// Pool with fixed capacity, mirrors Pawn style storage
pub struct AnimatorPool {
    slots: Vec<Option<Animator>>,
    largest_index: isize,
}

impl AnimatorPool {
    pub fn new() -> Self {
        Self {
            slots: vec![None; MAX_ANIMATORS],
            largest_index: -1,
        }
    }

    /// find free slot or -1
    fn get_free_slot(&mut self) -> isize {
        for (i, slot) in self.slots.iter().enumerate() {
            if slot.is_none() {
                if i as isize > self.largest_index {
                    self.largest_index = i as isize;
                }
                return i as isize;
            }
        }
        -1
    }

    /// Insert animator, returns slot index or -1
    pub fn insert(&mut self, anim: Animator) -> isize {
        let idx = self.get_free_slot();
        if idx == -1 { return -1; }
        self.slots[idx as usize] = Some(anim);
        idx
    }

    /// Destroy animator by index (like Animator_Destroy)
    pub fn destroy(&mut self, id: usize) -> bool {
        if id >= self.slots.len() { return false; }
        self.slots[id] = None;

        // update largest_index if needed
        if (id as isize) >= self.largest_index {
            // find next largest
            let mut found = false;
            for i in (0..=id).rev() {
                if self.slots[i].is_some() {
                    self.largest_index = i as isize;
                    found = true;
                    break;
                }
            }
            if !found {
                self.largest_index = -1;
            }
        }
        true
    }

    /// Process all animators: update, call stubs, destroy when finished
    pub fn process(&mut self) {
        if self.largest_index < 0 { return; }
        let max_current = (self.largest_index as usize) + 1;

        for i in 0..max_current {
            if let Some(anim) = &mut self.slots[i] {
                let t = anim.progress();
                // compute eased
                let eased = ease(anim.easing, t);

                // lerp position/size
                let x = lerp(anim.start_x, anim.target_x, eased);
                let y = lerp(anim.start_y, anim.target_y, eased);

                match anim.anim_type {
                    AnimatorType::Position => {
                        PlayerTextDrawSetPos(anim.player_id, anim.textdraw_id, x, y);
                    }
                    AnimatorType::LetterSize => {
                        PlayerTextDrawLetterSize(anim.player_id, anim.textdraw_id, x, y);
                    }
                    AnimatorType::TextSize => {
                        PlayerTextDrawTextSize(anim.player_id, anim.textdraw_id, x, y);
                    }
                    AnimatorType::FullSize => {
                        PlayerTextDrawLetterSize(anim.player_id, anim.textdraw_id, x, y);
                        PlayerTextDrawTextSize(anim.player_id, anim.textdraw_id, x, y);
                    }
                    AnimatorType::Color => {
                        if let (Some(c1), Some(c2)) = (anim.start_color, anim.target_color) {
                            let c = lerp_rgba(c1, c2, eased);
                            PlayerTextDrawColour(anim.player_id, anim.textdraw_id, c);
                        }
                    }
                    AnimatorType::BoxColor => {
                        if let (Some(c1), Some(c2)) = (anim.start_color, anim.target_color) {
                            let c = lerp_rgba(c1, c2, eased);
                            PlayerTextDrawBoxColour(anim.player_id, anim.textdraw_id, c);
                        }
                    }
                    AnimatorType::BackgroundColor => {
                        if let (Some(c1), Some(c2)) = (anim.start_color, anim.target_color) {
                            let c = lerp_rgba(c1, c2, eased);
                            PlayerTextDrawBackgroundColour(anim.player_id, anim.textdraw_id, c);
                        }
                    }
                }

                // show (in SA-MP: PlayerTextDrawShow)
                PlayerTextDrawShow(anim.player_id, anim.textdraw_id);

                if t >= 1.0 {
                    // call finish and destroy
                    Animator_OnFinish(anim.player_id, i, anim.anim_type);
                    self.destroy(i);
                }
            }
        }
    }

    /// helper to create a new animator with start time now
    pub fn insert_animator(
        &mut self,
        player_id: u32,
        textdraw_id: u32,
        start_x: f64, start_y: f64,
        target_x: f64, target_y: f64,
        duration_ms: u64,
        easing: Easing,
        anim_type: AnimatorType,
        start_color: Option<u32>,
        target_color: Option<u32>,
    ) -> isize {
        let anim = Animator {
            player_id,
            textdraw_id,
            anim_type,
            start_x,
            start_y,
            target_x,
            target_y,
            start_color,
            target_color,
            duration_ms,
            start_time: Instant::now(),
            easing,
        };
        self.insert(anim)
    }

    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(|s| s.is_none())
    }
}

/// =================== HELPERS (lerp & color) ===================

#[inline]
fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

/// Pawn-style RGBA pack: r<<24 | g<<16 | b<<8 | a
fn lerp_rgba(color1: u32, color2: u32, t: f64) -> u32 {
    let r1 = ((color1 >> 24) & 0xFF) as i32;
    let g1 = ((color1 >> 16) & 0xFF) as i32;
    let b1 = ((color1 >> 8) & 0xFF) as i32;
    let a1 = (color1 & 0xFF) as i32;

    let r2 = ((color2 >> 24) & 0xFF) as i32;
    let g2 = ((color2 >> 16) & 0xFF) as i32;
    let b2 = ((color2 >> 8) & 0xFF) as i32;
    let a2 = (color2 & 0xFF) as i32;

    let r = r1 + (( (r2 - r1) as f64 * t ).round() as i32);
    let g = g1 + (( (g2 - g1) as f64 * t ).round() as i32);
    let b = b1 + (( (b2 - b1) as f64 * t ).round() as i32);
    let a = a1 + (( (a2 - a1) as f64 * t ).round() as i32);

    (((r as u32) & 0xFF) << 24) |
    (((g as u32) & 0xFF) << 16) |
    (((b as u32) & 0xFF) << 8)  |
    ((a as u32) & 0xFF)
}

/// =================== STUBS for SA-MP native calls ===================
/// Replace these with your native bindings later.

 #[allow(non_snake_case)]
fn PlayerTextDrawSetPos(player_id: u32, textdraw_id: u32, x: f64, y: f64) {
    println!("Stub: SetPos(player={}, td={}, x={:.2}, y={:.2})", player_id, textdraw_id, x, y);
}

 #[allow(non_snake_case)]
fn PlayerTextDrawLetterSize(player_id: u32, textdraw_id: u32, x: f64, y: f64) {
    println!("Stub: LetterSize(player={}, td={}, x={:.2}, y={:.2})", player_id, textdraw_id, x, y);
}

 #[allow(non_snake_case)]
fn PlayerTextDrawTextSize(player_id: u32, textdraw_id: u32, x: f64, y: f64) {
    println!("Stub: TextSize(player={}, td={}, x={:.2}, y={:.2})", player_id, textdraw_id, x, y);
}

 #[allow(non_snake_case)]
fn PlayerTextDrawColour(player_id: u32, textdraw_id: u32, color: u32) {
    println!("Stub: Colour(player={}, td={}, color=0x{:08X})", player_id, textdraw_id, color);
}

 #[allow(non_snake_case)]
fn PlayerTextDrawBoxColour(player_id: u32, textdraw_id: u32, color: u32) {
    println!("Stub: BoxColour(player={}, td={}, color=0x{:08X})", player_id, textdraw_id, color);
}

 #[allow(non_snake_case)]
fn PlayerTextDrawBackgroundColour(player_id: u32, textdraw_id: u32, color: u32) {
    println!("Stub: BackgroundColour(player={}, td={}, color=0x{:08X})", player_id, textdraw_id, color);
}

 #[allow(non_snake_case)]
fn PlayerTextDrawShow(player_id: u32, textdraw_id: u32) {
    println!("Stub: Show(player={}, td={})", player_id, textdraw_id);
}

/// Callback when an animator finishes (stub)
 #[allow(non_snake_case)]
fn Animator_OnFinish(player_id: u32, animator_id: usize, anim_type: AnimatorType) {
    println!("Animator finished: id={} player={} type={:?}", animator_id, player_id, anim_type);
}

/// =================== HIGH-LEVEL API (helpers like Pawn) ===================

/// Insert helpers that mirror Pawn API
pub fn player_text_move_to(
    pool: &mut AnimatorPool,
    player_id: u32,
    textdraw_id: u32,
    x: f64,
    y: f64,
    duration_ms: u64,
    easing: Easing,
) -> isize {
    // In real environment, you'd query current pos; here we just assume current pos is (0,0) or you can pass it
    // For demonstration, let's assume textdraw currently at (0,0) — you can extend to read actual values
    let start_x = 0.0;
    let start_y = 0.0;
    pool.insert_animator(
        player_id, textdraw_id,
        start_x, start_y,
        x, y,
        duration_ms, easing,
        AnimatorType::Position,
        None, None,
    )
}

pub fn player_text_move_to_xy_from(pool: &mut AnimatorPool, player_id: u32, textdraw_id: u32, start_x: f64, start_y: f64, x: f64, y: f64, duration_ms: u64, easing: Easing) -> isize {
    pool.insert_animator(
        player_id, textdraw_id,
        start_x, start_y,
        x, y,
        duration_ms, easing,
        AnimatorType::Position,
        None, None,
    )
}

pub fn player_text_interpolate_color(
    pool: &mut AnimatorPool,
    player_id: u32,
    textdraw_id: u32,
    target_color: u32,
    duration_ms: u64,
    easing: Easing,
) -> isize {
    // in real usage you'd query current color; here we fake start color
    let start_color = 0xFF0000FFu32; // red as default example
    pool.insert_animator(
        player_id, textdraw_id,
        0.0, 0.0, 0.0, 0.0,
        duration_ms, easing,
        AnimatorType::Color,
        Some(start_color),
        Some(target_color),
    )
}