use rand::{SeedableRng, rngs::SmallRng};

use crate::star::{Star, Z_NEAR};
use crate::theme::Theme;

const CELLS_PER_STAR: u32 = 9;
const MIN_STARS: usize = 250;
const MAX_STARS: usize = 1800;

const MIN_SPEED: f32 = 0.08;
const DEFAULT_SPEED: f32 = 0.35;
const MAX_SPEED: f32 = 3.0;
const BOOST_MULT: f32 = 6.0;
const SPEED_LERP: f32 = 3.5;
const KILL_MARGIN: f32 = 4.0;

pub struct App {
    stars: Vec<Star>,
    width: u16,
    height: u16,
    cx: f32,
    cy: f32,
    half_w: f32,
    half_h: f32,
    theme: Theme,
    rng: SmallRng,
    base_speed: f32,
    speed: f32,
    target_speed: f32,
    boosting: bool,
    paused: bool,
    show_hud: bool,
    frame: u64,
    fps_ema: f32,
}

impl App {
    pub fn new(width: u16, height: u16) -> Self {
        let mut rng = SmallRng::from_os_rng();
        let (cx, cy, half_w, half_h) = geometry(width, height);
        let target = target_count(width, height);
        let mut stars = Vec::with_capacity(target);
        for _ in 0..target {
            stars.push(Star::spawn(&mut rng, half_w, half_h, true));
        }
        Self {
            stars,
            width,
            height,
            cx,
            cy,
            half_w,
            half_h,
            theme: Theme::default(),
            rng,
            base_speed: DEFAULT_SPEED,
            speed: DEFAULT_SPEED,
            target_speed: DEFAULT_SPEED,
            boosting: false,
            paused: false,
            show_hud: true,
            frame: 0,
            fps_ema: 60.0,
        }
    }

    #[inline]
    pub fn stars(&self) -> &[Star] {
        &self.stars
    }
    #[inline]
    pub fn theme(&self) -> Theme {
        self.theme
    }
    #[inline]
    pub fn speed(&self) -> f32 {
        self.speed
    }
    #[inline]
    pub fn boosting(&self) -> bool {
        self.boosting
    }
    #[inline]
    pub fn paused(&self) -> bool {
        self.paused
    }
    #[inline]
    pub fn show_hud(&self) -> bool {
        self.show_hud
    }
    #[inline]
    pub fn frame(&self) -> u64 {
        self.frame
    }
    #[inline]
    pub fn fps(&self) -> f32 {
        self.fps_ema
    }
    #[inline]
    pub fn cx(&self) -> f32 {
        self.cx
    }
    #[inline]
    pub fn cy(&self) -> f32 {
        self.cy
    }

    pub fn toggle_boost(&mut self) {
        self.boosting = !self.boosting;
        self.retarget();
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn toggle_hud(&mut self) {
        self.show_hud = !self.show_hud;
    }

    pub fn faster(&mut self) {
        self.base_speed = (self.base_speed * 1.25 + 0.02).min(MAX_SPEED);
        self.retarget();
    }

    pub fn slower(&mut self) {
        self.base_speed = ((self.base_speed - 0.02) / 1.25).max(MIN_SPEED);
        self.retarget();
    }

    pub fn next_theme(&mut self) {
        self.theme = self.theme.next();
    }

    pub fn prev_theme(&mut self) {
        self.theme = self.theme.prev();
    }

    fn retarget(&mut self) {
        self.target_speed = if self.boosting {
            (self.base_speed * BOOST_MULT).min(MAX_SPEED)
        } else {
            self.base_speed
        };
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        if self.width == width && self.height == height {
            return;
        }
        self.width = width;
        self.height = height;
        let (cx, cy, half_w, half_h) = geometry(width, height);
        self.cx = cx;
        self.cy = cy;
        self.half_w = half_w;
        self.half_h = half_h;

        let target = target_count(width, height);
        if target > self.stars.len() {
            self.stars.reserve(target - self.stars.len());
            for _ in 0..target - self.stars.len() {
                self.stars
                    .push(Star::spawn(&mut self.rng, half_w, half_h, true));
            }
        } else if target + 64 < self.stars.len() {
            self.stars.truncate(target);
            self.stars.shrink_to_fit();
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.frame += 1;
        if dt > 0.0 {
            let fps = 1.0 / dt;
            self.fps_ema += (fps - self.fps_ema) * 0.05;
        }
        if self.paused {
            return;
        }

        let k = 1.0 - (-SPEED_LERP * dt).exp();
        self.speed += (self.target_speed - self.speed) * k;

        let dz = self.speed * dt;
        if dz <= 0.0 {
            return;
        }
        let (half_w, half_h) = (self.half_w, self.half_h);
        let right = half_w + KILL_MARGIN;
        let bottom = half_h * 2.0 + KILL_MARGIN;
        for star in &mut self.stars {
            star.pz = star.z;
            star.z -= dz;
            if star.z <= Z_NEAR {
                star.respawn(&mut self.rng, half_w, half_h);
                continue;
            }
            let inv = 1.0 / star.z;
            if star.x * inv > right
                || star.x * inv < -right
                || star.y * inv > bottom
                || star.y * inv < -bottom
            {
                star.respawn(&mut self.rng, half_w, half_h);
            }
        }
    }
}

#[inline]
fn geometry(width: u16, height: u16) -> (f32, f32, f32, f32) {
    let w = width.max(1) as f32;
    let h = height.max(1) as f32;
    (w * 0.5, h * 0.5, w * 0.5, h * 0.5)
}

#[inline]
fn target_count(width: u16, height: u16) -> usize {
    let area = width as u32 * height as u32;
    (area / CELLS_PER_STAR).clamp(MIN_STARS as u32, MAX_STARS as u32) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::star::Z_FAR;
    #[test]
    fn tick_recycles_and_keeps_count() {
        let mut app = App::new(200, 60);
        let n = app.stars().len();
        assert!((250..=1800).contains(&n));
        for _ in 0..600 {
            app.tick(1.0 / 60.0);
        }
        assert_eq!(app.stars().len(), n);
        for s in app.stars() {
            assert!(s.z > Z_NEAR && s.z <= Z_FAR);
        }
    }
    #[test]
    fn resize_grows_and_shrinks() {
        let mut app = App::new(80, 24);
        let small = app.stars().len();
        app.resize(250, 80);
        assert!(app.stars().len() >= small);
        app.resize(80, 24);
        assert!(app.stars().len() <= 250 + 64);
    }
    #[test]
    fn boost_changes_target() {
        let mut app = App::new(100, 30);
        let base = app.speed();
        app.toggle_boost();
        for _ in 0..120 {
            app.tick(1.0 / 60.0);
        }
        assert!(app.speed() > base * 2.0);
    }
    #[test]
    fn fullscreen_coverage() {
        use crate::star::Y_SCALE;
        let app = App::new(200, 60);
        let half_w = 100.0;
        let half_h = 30.0;
        let mut visible = 0;
        let mut near_edge = 0;
        for s in app.stars() {
            let sx = s.x / s.z;
            let sy = s.y / s.z * Y_SCALE;
            if sx.abs() <= half_w && sy.abs() <= half_h {
                visible += 1;
                if sx.abs() > half_w * 0.7 || sy.abs() > half_h * 0.7 {
                    near_edge += 1;
                }
            }
        }
        let ratio = visible as f32 / app.stars().len() as f32;
        assert!(ratio > 0.95, "visible ratio {ratio}");
        assert!(
            near_edge as f32 / app.stars().len() as f32 > 0.25,
            "edge stars {} of {}",
            near_edge,
            app.stars().len()
        );
    }
}
