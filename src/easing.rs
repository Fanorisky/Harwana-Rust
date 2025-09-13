use std::time::{Duration, Instant};

// =================== EASING FUNCTIONS ===================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    Linear,
    InQuad,
    OutQuad,
    InOutQuad,
    InCubic,
    OutCubic,
    InOutCubic,
    InQuart,
    OutQuart,
    InOutQuart,
    InQuint,
    OutQuint,
    InOutQuint,
    InSine,
    OutSine,
    InOutSine,
    InExpo,
    OutExpo,
    InOutExpo,
    InCirc,
    OutCirc,
    InOutCirc,
    InBack,
    OutBack,
    InOutBack,
    InElastic,
    OutElastic,
    InOutElastic,
    InBounce,
    OutBounce,
    InOutBounce,
}

pub fn ease(kind: Easing, t: f64) -> f64 {
    use Easing::*;
    match kind {
        Linear => linear(t),
        InQuad => in_quad(t),
        OutQuad => out_quad(t),
        InOutQuad => in_out_quad(t),
        InCubic => in_cubic(t),
        OutCubic => out_cubic(t),
        InOutCubic => in_out_cubic(t),
        InQuart => in_quart(t),
        OutQuart => out_quart(t),
        InOutQuart => in_out_quart(t),
        InQuint => in_quint(t),
        OutQuint => out_quint(t),
        InOutQuint => in_out_quint(t),
        InSine => in_sine(t),
        OutSine => out_sine(t),
        InOutSine => in_out_sine(t),
        InExpo => in_expo(t),
        OutExpo => out_expo(t),
        InOutExpo => in_out_expo(t),
        InCirc => in_circ(t),
        OutCirc => out_circ(t),
        InOutCirc => in_out_circ(t),
        InBack => in_back(t),
        OutBack => out_back(t),
        InOutBack => in_out_back(t),
        InElastic => in_elastic(t),
        OutElastic => out_elastic(t),
        InOutElastic => in_out_elastic(t),
        InBounce => in_bounce(t),
        OutBounce => out_bounce(t),
        InOutBounce => in_out_bounce(t),
    }
}

#[inline] fn clamp01(t: f64) -> f64 { t.max(0.0).min(1.0) }
#[inline] pub fn linear(t: f64) -> f64 { clamp01(t) }

// Quadratic
#[inline] pub fn in_quad(t: f64) -> f64 { let t=clamp01(t); t*t }
#[inline] pub fn out_quad(t: f64) -> f64 { let t=clamp01(t); t*(2.0-t) }
#[inline] pub fn in_out_quad(t: f64) -> f64 { let mut t=clamp01(t)*2.0; if t<1.0 {0.5*t*t} else {t-=1.0; -0.5*(t*(t-2.0)-1.0)} }

// Cubic
#[inline] pub fn in_cubic(t: f64) -> f64 { let t=clamp01(t); t*t*t }
#[inline] pub fn out_cubic(t: f64) -> f64 { let t=clamp01(t)-1.0; t*t*t+1.0 }
#[inline] pub fn in_out_cubic(t: f64) -> f64 { let mut t=clamp01(t)*2.0; if t<1.0 {0.5*t*t*t} else {t-=2.0;0.5*(t*t*t+2.0)} }

// Quartic
#[inline] pub fn in_quart(t: f64) -> f64 { let t=clamp01(t); t*t*t*t }
#[inline] pub fn out_quart(t: f64) -> f64 { let t=clamp01(t)-1.0; 1.0-t*t*t*t }
#[inline] pub fn in_out_quart(t: f64) -> f64 { let mut t=clamp01(t)*2.0; if t<1.0 {0.5*t*t*t*t} else {t-=2.0;-0.5*(t*t*t*t-2.0)} }

// Quintic
#[inline] pub fn in_quint(t: f64) -> f64 { let t=clamp01(t); t*t*t*t*t }
#[inline] pub fn out_quint(t: f64) -> f64 { let t=clamp01(t)-1.0; t*t*t*t*t+1.0 }
#[inline] pub fn in_out_quint(t: f64) -> f64 { let mut t=clamp01(t)*2.0; if t<1.0 {0.5*t*t*t*t*t} else {t-=2.0;0.5*(t*t*t*t*t+2.0)} }

// Sine
#[inline] pub fn in_sine(t: f64) -> f64 { 1.0-(clamp01(t)*std::f64::consts::FRAC_PI_2).cos() }
#[inline] pub fn out_sine(t: f64) -> f64 { (clamp01(t)*std::f64::consts::FRAC_PI_2).sin() }
#[inline] pub fn in_out_sine(t: f64) -> f64 { -0.5*((std::f64::consts::PI*clamp01(t)).cos()-1.0) }

// Expo
#[inline] pub fn in_expo(t: f64) -> f64 { if t<=0.0 {0.0} else {(2.0f64).powf(10.0*(clamp01(t)-1.0))} }
#[inline] pub fn out_expo(t: f64) -> f64 { if t>=1.0 {1.0} else {1.0-(2.0f64).powf(-10.0*clamp01(t))} }
#[inline] pub fn in_out_expo(t: f64) -> f64 { let mut t=clamp01(t); if t==0.0{return 0.0;} if t==1.0{return 1.0;} t*=2.0; if t<1.0 {0.5*(2.0f64).powf(10.0*(t-1.0))} else {0.5*(2.0-(2.0f64).powf(-10.0*(t-1.0)))} }

// Circ
#[inline] pub fn in_circ(t: f64) -> f64 { 1.0-(1.0-clamp01(t)*clamp01(t)).sqrt() }
#[inline] pub fn out_circ(t: f64) -> f64 { let t=clamp01(t)-1.0; (1.0-t*t).sqrt() }
#[inline] pub fn in_out_circ(t: f64) -> f64 { let mut t=clamp01(t)*2.0; if t<1.0 {-0.5*((1.0-t*t).sqrt()-1.0)} else {t-=2.0;0.5*((1.0-t*t).sqrt()+1.0)} }

// Back
#[inline] pub fn in_back(t: f64) -> f64 { let s=1.70158; let t=clamp01(t); t*t*((s+1.0)*t-s) }
#[inline] pub fn out_back(t: f64) -> f64 { let s=1.70158; let t=clamp01(t)-1.0; t*t*((s+1.0)*t+s)+1.0 }
#[inline] pub fn in_out_back(t: f64) -> f64 { let s=1.70158*1.525; let mut t=clamp01(t)*2.0; if t<1.0 {0.5*(t*t*((s+1.0)*t-s))} else {t-=2.0;0.5*(t*t*((s+1.0)*t+s)+2.0)} }

// Elastic
#[inline] pub fn in_elastic(t: f64) -> f64 { let t=clamp01(t); if t==0.0{return 0.0;} if t==1.0{return 1.0;} let p=0.3; let s=p/4.0; let t=t-1.0; -((2.0f64).powf(10.0*t))*((t-s)*(2.0*std::f64::consts::PI)/p).sin() }
#[inline] pub fn out_elastic(t: f64) -> f64 { let t=clamp01(t); if t==0.0{return 0.0;} if t==1.0{return 1.0;} let p=0.3; let s=p/4.0; (2.0f64).powf(-10.0*t)*((t-s)*(2.0*std::f64::consts::PI)/p).sin()+1.0 }
#[inline] pub fn in_out_elastic(t: f64) -> f64 { let mut t=clamp01(t)*2.0; if t==0.0{return 0.0;} if t==2.0{return 1.0;} let p=0.45; let s=p/4.0; if t<1.0 { let t1=t-1.0; -0.5*(2.0f64).powf(10.0*t1)*((t1-s)*(2.0*std::f64::consts::PI)/p).sin() } else { let t1=t-1.0; 0.5*(2.0f64).powf(-10.0*t1)*((t1-s)*(2.0*std::f64::consts::PI)/p).sin()+1.0 } }

// Bounce
#[inline] fn out_bounce(t: f64) -> f64 { let t=clamp01(t); if t<1.0/2.75 {7.5625*t*t} else if t<2.0/2.75 {let t=t-1.5/2.75; 7.5625*t*t+0.75} else if t<2.5/2.75 {let t=t-2.25/2.75; 7.5625*t*t+0.9375} else {let t=t-2.625/2.75; 7.5625*t*t+0.984375} }
#[inline] pub fn in_bounce(t: f64) -> f64 { 1.0-out_bounce(1.0-clamp01(t)) }
#[inline] pub fn in_out_bounce(t: f64) -> f64 { let t=clamp01(t); if t<0.5 {0.5*in_bounce(t*2.0)} else {0.5*out_bounce(t*2.0-1.0)+0.5} }

// =================== ANIMATOR ===================
pub struct Animator {
    pub from: f64,
    pub to: f64,
    pub duration: Duration,
    pub easing: Easing,
    start_time: Instant,
    finished: bool,
}

impl Animator {
    pub fn new(from: f64, to: f64, duration_ms: u64, easing: Easing) -> Self {
        Self {
            from,
            to,
            duration: Duration::from_millis(duration_ms),
            easing,
            start_time: Instant::now(),
            finished: false,
        }
    }

    pub fn value(&mut self) -> f64 {
        if self.finished { return self.to; }
        let elapsed = self.start_time.elapsed();
        let t = (elapsed.as_secs_f64() / self.duration.as_secs_f64()).min(1.0);
        if t>=1.0 { self.finished=true; return self.to; }
        let eased = ease(self.easing, t);
        self.from+(self.to-self.from)*eased
    }

    pub fn is_finished(&self) -> bool { self.finished }
}

#[derive(Debug, Clone, Copy)]
pub struct Color { pub r:f64, pub g:f64, pub b:f64, pub a:f64 }

pub struct ColorAnimator { pub r:Animator, pub g:Animator, pub b:Animator, pub a:Animator }
impl ColorAnimator {
    pub fn new(from:Color,to:Color,dur:u64,easing:Easing)->Self{
        Self{r:Animator::new(from.r,to.r,dur,easing),g:Animator::new(from.g,to.g,dur,easing),b:Animator::new(from.b,to.b,dur,easing),a:Animator::new(from.a,to.a,dur,easing)}
    }
    pub fn value(&mut self)->Color{Color{r:self.r.value(),g:self.g.value(),b:self.b.value(),a:self.a.value()}}
    pub fn is_finished(&self)->bool{self.r.is_finished()&&self.g.is_finished()&&self.b.is_finished()&&self.a.is_finished()}
}

pub struct TextDrawAnimator {
    pub pos_x: Option<Animator>,
    pub pos_y: Option<Animator>,
    pub scale_x: Option<Animator>,
    pub scale_y: Option<Animator>,
    pub alpha: Option<Animator>,
    pub color: Option<ColorAnimator>,
}

impl TextDrawAnimator {
    pub fn new()->Self{Self{pos_x:None,pos_y:None,scale_x:None,scale_y:None,alpha:None,color:None}}
    pub fn with_pos(mut self,fx:f64,tx:f64,fy:f64,ty:f64,dur:u64,easing:Easing)->Self{self.pos_x=Some(Animator::new(fx,tx,dur,easing));self.pos_y=Some(Animator::new(fy,ty,dur,easing));self}
    pub fn with_scale(mut self,fx:f64,tx:f64,fy:f64,ty:f64,dur:u64,easing:Easing)->Self{self.scale_x=Some(Animator::new(fx,tx,dur,easing));self.scale_y=Some(Animator::new(fy,ty,dur,easing));self}
    pub fn with_alpha(mut self,from:f64,to:f64,dur:u64,easing:Easing)->Self{self.alpha=Some(Animator::new(from,to,dur,easing));self}
    pub fn with_color(mut self,from:Color,to:Color,dur:u64,easing:Easing)->Self{self.color=Some(ColorAnimator::new(from,to,dur,easing));self}

    pub fn update(&mut self)->TextDrawState{
        TextDrawState{
            pos_x:self.pos_x.as_mut().map(|a|a.value()),
            pos_y:self.pos_y.as_mut().map(|a|a.value()),
            scale_x:self.scale_x.as_mut().map(|a|a.value()),
            scale_y:self.scale_y.as_mut().map(|a|a.value()),
            alpha:self.alpha.as_mut().map(|a|a.value()),
            color:self.color.as_mut().map(|c|c.value()),
        }
    }
    pub fn is_finished(&self)->bool{
        let mut done=true;
        if let Some(a)=&self.pos_x{done&=a.is_finished();}
        if let Some(a)=&self.pos_y{done&=a.is_finished();}
        if let Some(a)=&self.scale_x{done&=a.is_finished();}
        if let Some(a)=&self.scale_y{done&=a.is_finished();}
        if let Some(a)=&self.alpha{done&=a.is_finished();}
        if let Some(c)=&self.color{done&=c.is_finished();}
        done
    }
}

#[derive(Debug)]
pub struct TextDrawState {
    pub pos_x: Option<f64>,
    pub pos_y: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub alpha: Option<f64>,
    pub color: Option<Color>,
}
