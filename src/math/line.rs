use bevy::prelude::*;

use crate::math::operations::{ Center, Intersects };

pub struct Line {
    pub point1: Vec2,
    pub point2: Vec2
}

impl Line {
    pub fn intersects_position(&self, other: &Line) -> Option<Vec2> {
        // using the formula as described here https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
        let s1 = self.point2 - self.point1;
        let s2 = other.point2 - other.point1;

        let s = (-s1.y() * (self.point1.x() - other.point1.x()) + s1.x() * (self.point1.y() - other.point1.y())) / (-s2.x() * s1.y() + s1.x() * s2.y());
        let t = ( s2.x() * (self.point1.y() - other.point1.y()) - s2.y() * (self.point1.x() - other.point1.x())) / (-s2.x() * s1.y() + s1.x() * s2.y());
    
        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
            return Some(Vec2::new(
                self.point1.x() + (t * s1.x()), 
                self.point1.y() + (t * s1.y())
            ));
        }
        
        None
    }
}

impl Intersects<Line> for Line {
    fn intersects(&self, other: &Line) -> bool {
        match self.intersects_position(other) {
            Some(_) => return true,
            None => return false
        }
    }
}

pub trait Parallel<T> {
    fn parralel(&self, offset: f32) -> T; 
}

impl Parallel<Line> for Line {
    fn parralel(&self, offset: f32) -> Line {
        let vec = self.point1 - self.point2;
        
        Line {
            point1: Vec2::new(-vec.y(), vec.x()).normalize() * offset + self.point1,
            point2: Vec2::new(-vec.y(), vec.x()).normalize() * offset + self.point2
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use super::*;

    #[test]
    fn lines_intersects() {
        let line1 = Line {
            point1: Vec2::new(0.0, 0.0),
            point2: Vec2::new(100.0, 0.0)
        };       

        let line2 = Line {
            point1: Vec2::new(50.0, 0.0),
            point2: Vec2::new(0.0, 100.0)
        };  

        assert_eq!(line1.intersects(&line2), true);
    }

    #[test]
    fn lines_not_intersects() {
        let line1 = Line {
            point1: Vec2::new(0.0, 50.0),
            point2: Vec2::new(100.0, 50.0)
        };       

        let line2 = Line {
            point1: Vec2::new(150.0, 0.0),
            point2: Vec2::new(150.0, 100.0)
        };  

        assert_eq!(line1.intersects_position(&line2), None);
    }

}