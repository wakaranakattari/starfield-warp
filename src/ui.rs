use ratatui::{Frame, buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::app::App;
use crate::star::Y_SCALE;
use crate::theme::Depth;

const HEAD_GLYPHS: [char; 4] = ['·', '•', '*', '●'];
const MAX_STREAK: i32 = 16;
const DOT_TAIL: char = '·';

#[inline]
fn tail_factor(speed: f32) -> f32 {
    (speed / 1.2).clamp(0.22, 1.0)
}

pub fn draw(frame: &mut Frame, app: &App) {
    frame.render_widget(StarWidget { app }, frame.area());
}

struct StarWidget<'a> {
    app: &'a App,
}

impl Widget for StarWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }
        clear_area(area, buf);

        let app = self.app;
        let styles = app.theme().styles();
        let base_x = area.x as f32;
        let base_y = area.y as f32;
        let right = area.right() as i32;
        let bottom = area.bottom() as i32;
        let frame_no = app.frame();

        for (idx, star) in app.stars().iter().enumerate() {
            let z = star.z;
            let pz = star.pz;
            if z > 0.8 && (frame_no.wrapping_add((idx as u64).wrapping_mul(13)) % 41) < 2 {
                continue;
            }

            let inv = 1.0 / z;
            let pinv = 1.0 / pz;
            let sx = base_x + app.cx() + star.x * inv;
            let sy = base_y + app.cy() + star.y * inv * Y_SCALE;
            let px = base_x + app.cx() + star.x * pinv;
            let py = base_y + app.cy() + star.y * pinv * Y_SCALE;

            let head_x = sx.floor() as i32;
            let head_y = sy.floor() as i32;
            let prev_x = px.floor() as i32;
            let prev_y = py.floor() as i32;

            let dx = head_x - prev_x;
            let dy = head_y - prev_y;
            if dx == 0 && dy == 0 {
                if head_x >= area.x as i32
                    && head_x < right
                    && head_y >= area.y as i32
                    && head_y < bottom
                {
                    let depth = Depth::from_z(z) as usize;
                    let cell = &mut buf[(head_x as u16, head_y as u16)];
                    cell.set_char(HEAD_GLYPHS[depth]);
                    cell.set_style(styles[depth]);
                }
                continue;
            }

            let depth = Depth::from_z(z) as usize;
            let tail_style = styles[depth.saturating_sub(1)];

            let steps = dx.abs().max(dy.abs());
            if steps <= 0 {
                continue;
            }
            let factor = tail_factor(app.speed());
            let eff = ((steps as f32 * factor).round() as i32).clamp(0, MAX_STREAK);
            if eff == 0 {
                if head_x >= area.x as i32
                    && head_x < right
                    && head_y >= area.y as i32
                    && head_y < bottom
                {
                    let cell = &mut buf[(head_x as u16, head_y as u16)];
                    cell.set_char(HEAD_GLYPHS[depth]);
                    cell.set_style(styles[depth]);
                }
                continue;
            }
            let tx0 = sx - (sx - px) * factor;
            let ty0 = sy - (sy - py) * factor;
            let tail_x = tx0.floor() as i32;
            let tail_y = ty0.floor() as i32;
            let min_x = head_x.min(tail_x);
            let max_x = head_x.max(tail_x);
            let min_y = head_y.min(tail_y);
            let max_y = head_y.max(tail_y);
            if max_x < area.x as i32 || min_x >= right || max_y < area.y as i32 || min_y >= bottom {
                continue;
            }

            let streak_glyph = if eff <= 2 {
                DOT_TAIL
            } else {
                streak_char(dx, dy)
            };

            let fx0 = tx0;
            let fy0 = ty0;
            let fx1 = sx;
            let fy1 = sy;
            for i in 0..eff {
                let tt = (i as f32 + 1.0) / (eff as f32 + 1.0);
                let fx = fx0 + (fx1 - fx0) * tt;
                let fy = fy0 + (fy1 - fy0) * tt;
                let cx = fx.floor() as i32;
                let cy = fy.floor() as i32;
                if cx >= area.x as i32 && cx < right && cy >= area.y as i32 && cy < bottom {
                    let cell = &mut buf[(cx as u16, cy as u16)];
                    cell.set_char(streak_glyph);
                    cell.set_style(tail_style);
                }
            }

            if head_x >= area.x as i32
                && head_x < right
                && head_y >= area.y as i32
                && head_y < bottom
            {
                let cell = &mut buf[(head_x as u16, head_y as u16)];
                cell.set_char(HEAD_GLYPHS[depth]);
                cell.set_style(styles[depth]);
            }
        }

        draw_reticle(area, buf, app);
        if app.show_hud() {
            draw_hud(area, buf, app);
        }
    }
}

#[inline]
fn streak_char(dx: i32, dy: i32) -> char {
    let ax = dx.abs();
    let ay = dy.abs();
    if ax > ay * 2 {
        '─'
    } else if ay > ax * 2 {
        '│'
    } else if (dx < 0) == (dy < 0) {
        '╲'
    } else {
        '╱'
    }
}

fn draw_reticle(area: Rect, buf: &mut Buffer, app: &App) {
    let cx = area.x as i32 + app.cx() as i32;
    let cy = area.y as i32 + app.cy() as i32;
    if cx < area.x as i32
        || cx >= area.right() as i32
        || cy < area.y as i32
        || cy >= area.bottom() as i32
    {
        return;
    }
    let styles = app.theme().styles();
    let cell = &mut buf[(cx as u16, cy as u16)];
    cell.set_char(if app.boosting() { '✦' } else { '·' });
    cell.set_style(styles[0]);
}

fn draw_hud(area: Rect, buf: &mut Buffer, app: &App) {
    if area.height < 2 || area.width < 24 {
        return;
    }
    let label = if app.paused() {
        format!(
            " paused · {} stars · {} · {:.2}x ",
            app.stars().len(),
            app.theme().name(),
            app.speed()
        )
    } else if app.boosting() {
        format!(
            " WARP {:.2}x · {} stars · {} · {:.0}fps ",
            app.speed(),
            app.stars().len(),
            app.theme().name(),
            app.fps()
        )
    } else {
        format!(
            " {} stars · {:.2}x · {} · {:.0}fps · Tab theme · Space warp ",
            app.stars().len(),
            app.speed(),
            app.theme().name(),
            app.fps()
        )
    };
    let w = label.chars().count().min(area.width as usize) as u16;
    let x = area.right().saturating_sub(w + 1);
    let y = area.bottom().saturating_sub(1);
    buf.set_string(x, y, &label, Style::default());
}

fn clear_area(area: Rect, buf: &mut Buffer) {
    let blank = Style::default();
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            let cell = &mut buf[(x, y)];
            cell.set_char(' ');
            cell.set_style(blank);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cruise_tail_is_short_warp_is_full() {
        assert!(tail_factor(0.35) < 0.4, "cruise {}", tail_factor(0.35));
        assert_eq!(tail_factor(3.0), 1.0);
        assert!(tail_factor(0.08) >= 0.22);
    }
    #[test]
    fn short_streak_uses_dot() {
        assert_eq!(streak_char(5, 0), '─');
        assert_eq!(streak_char(0, 5), '│');
    }
}
