use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn abs(&self) -> f64 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line(pub Vec2, pub Vec2);

impl Line {
    pub fn len(&self) -> f64 {
        (self.0 - self.1).abs()
    }
}

pub fn intersect(arrow: &Line, line: &Line) -> Option<f64> {
    let (a, b) = (line.0 - arrow.0, line.1 - arrow.0);
    let v = arrow.1 - arrow.0;

    let inv = Vec2 {
        x: (v.x * b.y - b.x * v.y) / (a.x * b.y - b.x * a.y),
        y: (v.y * a.x - a.y * v.x) / (a.x * b.y - b.x * a.y),
    };

    if inv.x >= 0.0 && inv.y >= 0.0 && (inv.x + inv.y) >= 1.0 {
        let v = Vec2 {
            x: inv.x / (inv.x + inv.y),
            y: inv.y / (inv.x + inv.y),
        };
        Some(((v.x * a.x + v.y * b.x).powi(2) + (v.x * a.y + v.y * b.y).powi(2)).sqrt())
    } else {
        None
    }
}

pub fn barr_check(arrow: &Line, barriers: &Vec<Line>) -> Option<f64> {
    barriers
        .iter()
        .filter_map(|barr| intersect(arrow, barr))
        .fold(None, |min, b| match min {
            None => Some(b),
            Some(a) => {
                if a > b {
                    Some(b)
                } else {
                    Some(a)
                }
            }
        })
}

pub fn rtx(mut arrow: Line, barriers: &Vec<Line>, max_dist: f64) -> f64 {
    let mut dist = 0.0;
    let mut check = barr_check(&arrow, barriers);
    while check.is_none() && dist < max_dist {
        dist += arrow.len();
        arrow = Line(
            arrow.1,
            2.0 * arrow.1 - arrow.0,
        );
        check = barr_check(&arrow, barriers);
    }
    check.map_or(max_dist, |add| dist + add)
}
