use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Depth {
    Deep = 0,
    Dim = 1,
    Mid = 2,
    Head = 3,
}

impl Depth {
    #[inline]
    pub fn from_z(z: f32) -> Self {
        if z > 0.75 {
            Self::Deep
        } else if z > 0.45 {
            Self::Dim
        } else if z > 0.18 {
            Self::Mid
        } else {
            Self::Head
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
    Starlight,
    Matrix,
    Nebula,
    Solar,
    Ice,
    Crimson,
    Sunset,
    Ghost,
}

impl Theme {
    pub fn next(self) -> Self {
        match self {
            Self::Starlight => Self::Matrix,
            Self::Matrix => Self::Nebula,
            Self::Nebula => Self::Solar,
            Self::Solar => Self::Ice,
            Self::Ice => Self::Crimson,
            Self::Crimson => Self::Sunset,
            Self::Sunset => Self::Ghost,
            Self::Ghost => Self::Starlight,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Starlight => Self::Ghost,
            Self::Matrix => Self::Starlight,
            Self::Nebula => Self::Matrix,
            Self::Solar => Self::Nebula,
            Self::Ice => Self::Solar,
            Self::Crimson => Self::Ice,
            Self::Sunset => Self::Crimson,
            Self::Ghost => Self::Sunset,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Starlight => "Starlight",
            Self::Matrix => "Matrix",
            Self::Nebula => "Nebula",
            Self::Solar => "Solar",
            Self::Ice => "Ice",
            Self::Crimson => "Crimson",
            Self::Sunset => "Sunset",
            Self::Ghost => "Ghost",
        }
    }

    pub fn styles(self) -> [Style; 4] {
        match self {
            Self::Starlight => [
                Style::default().fg(Color::Rgb(70, 80, 110)),
                Style::default().fg(Color::Rgb(120, 150, 200)),
                Style::default().fg(Color::Rgb(180, 210, 255)),
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Matrix => [
                Style::default().fg(Color::Rgb(0, 70, 20)),
                Style::default().fg(Color::Rgb(0, 170, 60)),
                Style::default().fg(Color::Rgb(80, 255, 120)),
                Style::default()
                    .fg(Color::Rgb(230, 255, 230))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Nebula => [
                Style::default().fg(Color::Rgb(60, 30, 90)),
                Style::default().fg(Color::Rgb(130, 80, 200)),
                Style::default().fg(Color::Rgb(220, 130, 255)),
                Style::default()
                    .fg(Color::Rgb(250, 240, 255))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Solar => [
                Style::default().fg(Color::Rgb(90, 55, 10)),
                Style::default().fg(Color::Rgb(200, 130, 30)),
                Style::default().fg(Color::Rgb(255, 200, 90)),
                Style::default()
                    .fg(Color::Rgb(255, 250, 230))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Ice => [
                Style::default().fg(Color::Rgb(30, 80, 90)),
                Style::default().fg(Color::Rgb(60, 170, 190)),
                Style::default().fg(Color::Rgb(130, 230, 245)),
                Style::default()
                    .fg(Color::Rgb(235, 255, 255))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Crimson => [
                Style::default().fg(Color::Rgb(90, 20, 25)),
                Style::default().fg(Color::Rgb(190, 50, 60)),
                Style::default().fg(Color::Rgb(255, 110, 110)),
                Style::default()
                    .fg(Color::Rgb(255, 235, 235))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Sunset => [
                Style::default().fg(Color::Rgb(80, 35, 60)),
                Style::default().fg(Color::Rgb(200, 90, 110)),
                Style::default().fg(Color::Rgb(255, 170, 100)),
                Style::default()
                    .fg(Color::Rgb(255, 245, 230))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Ghost => [
                Style::default().fg(Color::Rgb(45, 48, 60)),
                Style::default().fg(Color::Rgb(90, 95, 115)),
                Style::default().fg(Color::Rgb(150, 155, 180)),
                Style::default()
                    .fg(Color::Rgb(220, 225, 235))
                    .add_modifier(Modifier::BOLD),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn depth_buckets() {
        assert_eq!(Depth::from_z(0.9), Depth::Deep);
        assert_eq!(Depth::from_z(0.6), Depth::Dim);
        assert_eq!(Depth::from_z(0.3), Depth::Mid);
        assert_eq!(Depth::from_z(0.05), Depth::Head);
    }
    #[test]
    fn theme_cycle() {
        let mut t = Theme::Starlight;
        for _ in 0..8 {
            t = t.next();
        }
        assert_eq!(t, Theme::Starlight);
    }
}
