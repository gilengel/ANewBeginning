use bevy::prelude::*;

use crate::math::operations::{ Center, Intersects };
use crate::math::line::Line;

pub struct Polygon {
    points: Vec<Vec2>
}

impl Center for Polygon {
    fn center(&self) -> Vec2 {
        

        // using the formula as described here https://en.wikipedia.org/wiki/Centroid#Of_a_polygon
        let mut a = 0.0;

        let mut c_x = 0.0;
        let mut c_y  = 0.0;

        let mut iter = self.points.iter().peekable();
        while let Some(point) = iter.next() {
            if let Some(next) = iter.peek() {
                let term = point.x() * next.y() - next.x() * point.y();
                                
                c_x = c_x + (point.x() + next.x()) * term;
                c_y = c_y + (point.y() + next.y()) * term;
                
                a = a + point.x() * next.y() - next.x() * point.y(); 
            }
        }        
        a = 0.5 * a;

        Vec2::new(1.0 / (6.0 * a) * c_x, 1.0 / (6.0 * a) * c_y)
    }
}

fn convert_points_to_lines(polygon: &Polygon) -> Vec<Line> {
    let mut lines = Vec::new();
    if polygon.points.is_empty() {
        return lines;
    }

    let mut iter = polygon.points.iter().peekable();
    while let Some(point) = iter.next() {
        if let Some(next) = iter.peek() {
            lines.push(Line {
                point1: point.clone(),
                point2: *next.clone()
            });
        } else {
            lines.push(Line {
                point1: point.clone(),
                point2: *polygon.points.first().unwrap()
            });
        }        
    }

    lines
}

impl Intersects<Polygon> for Polygon {
    fn intersects(&self, other: &Polygon) -> bool {
        println!("AOOOOOOOOOOOOOOOOOOOOOOOOOOOO");
        for line in convert_points_to_lines(self) {
            println!("{}", line);
            for other_line in convert_points_to_lines(other) {
                if line.intersects(&other_line) {
                    println!("{} {}", line, other_line);
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use super::*;

    #[test]
    fn center_triangle() {
        let polygon = Polygon {
            points: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(100.0, 0.0),
                Vec2::new(50.0, 50.0)
                ]
        };       

        assert_eq!(polygon.center(), Vec2::new(50.0, 16.666666));
    }

    #[test]
    fn center_quad() {
        let polygon = Polygon {
            points: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(100.0, 0.0),
                Vec2::new(100.0, 100.0),
                Vec2::new(0.0, 100.0)
                ]
        };       

        assert_eq!(polygon.center(), Vec2::new(50.0, 50.0));
    }

    #[test]
    fn intersection_polygons() {
        let polygon = Polygon {
            points: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(100.0, 0.0),
                Vec2::new(100.0, 100.0),
                Vec2::new(0.0, 100.0)
                ]
        };  
        
        let polygon2 = Polygon {
            points: vec![
                Vec2::new(50.0, 50.0),
                Vec2::new(150.0, 50.0),
                Vec2::new(150.0, 150.0),
                Vec2::new(50.0, 150.0)
                ]
        };     

        assert_eq!(polygon.intersects(&polygon2), true);
    }

    #[test]
    fn not_intersection_polygons() {
        let polygon = Polygon {
            points: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(100.0, 0.0),
                Vec2::new(100.0, 100.0),
                Vec2::new(0.0, 100.0)
                ]
        };  
        
        let polygon2 = Polygon {
            points: vec![
                Vec2::new(200.0, 0.0),
                Vec2::new(300.0, 0.0),
                Vec2::new(300.0, 100.0),
                Vec2::new(200.0, 100.0)
                ]
        };     

        assert_eq!(polygon.intersects(&polygon2), false);
    }
}