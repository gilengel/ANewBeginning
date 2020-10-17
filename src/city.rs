use bevy::{
    prelude::Vec2
};

/// Calculates the center position between two vectors in world space
fn calculate_center_ws(start: Vec2, end: Vec2) -> Vec2 {
    let connection_vec2 = end - start;
    let connection_length = connection_vec2.length();
    
    start + connection_vec2 / connection_length * connection_length / 2.0
}   

#[derive(Default)]
pub struct StraightStreet {    
    pub position: Vec2,
    pub rotation: f32,
    
    pub start: Vec2,
    pub end: Vec2
}

impl StraightStreet {
    pub fn length(&self) -> f32 {
        (self.start - self.end).length()
    }

    pub fn new(start: Vec2, end: Vec2) -> StraightStreet {
        StraightStreet { 
            start: start, 
            end: end, 
            position: calculate_center_ws(start, end),
            rotation: -(end - start).angle_between(Vec2::new(1.0, 0.0))
        }
    }

    pub fn set_start(&mut self, start: Vec2) {
        self.start = start;

        self.position = calculate_center_ws(start, self.end);
        self.rotation =  -(self.end - start).angle_between(Vec2::new(1.0, 0.0));
    }

    pub fn set_end(&mut self, end: Vec2) {
        self.end = end;

        self.position = calculate_center_ws(self.start, end);
        self.rotation =  -(end - self.start).angle_between(Vec2::new(1.0, 0.0));
    }

    pub fn intersection(&self, other: &StraightStreet) -> Option<Vec2> {
        let s1 = self.end - self.start;
        let s2 = other.end - other.start;

        let s = (-s1.y() * (self.start.x() - other.start.x()) + s1.x() * (self.start.y() - other.start.y())) / (-s2.x() * s1.y() + s1.x() * s2.y());
        let t = ( s2.x() * (self.start.y() - other.start.y()) - s2.y() * (self.start.x() - other.start.x())) / (-s2.x() * s1.y() + s1.x() * s2.y());
    
        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
            return Some(Vec2::new(
                    self.start.x() + (t * s1.x()), 
                    self.start.y() + (t * s1.y())
            ));
        }  
    
        None
    }   
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use super::*;

    #[test]
    fn it_works() {
        let street1 = StraightStreet {
            start: Vec2::new(-50.0, 0.0),
            end: Vec2::new(50.0, 0.0),
            position: Vec2::new(0.0 , 0.0),
            rotation: 0.0
        };

        let street2 = StraightStreet {
            start: Vec2::new(0.0, 0.0),
            end: Vec2::new(0.0, 100.0),
            position: Vec2::new(0.0 , 0.0),
            rotation: 0.0
        };

        let intersection = street1.intersection(&street2);

        assert!(intersection.is_some());

        if let Some(i) = intersection {
            assert_eq!(i, Vec2::new(0.0, 0.0));
        }
        


        assert_eq!(2 + 2, 4);
    }
}


/*
pub trait Connection {
    fn new(start: Vec2, end: Vec2) -> Self;
    
    // Sets the start to the new position
    fn set_start(&mut self, start: Vec2);

    // Sets the end to the new position
    fn set_end(&mut self, end: Vec2); 

    fn intersection(&self, other: &StraightStreet) -> Option<Vec2>;
}


impl Connection for StraightStreet {

}



fn get_vector_intersection(line1_start: Vec2, line1_end: Vec2, line2_start: Vec2, line2_end: Vec2) -> Option<Vec2> {
    let s1 = line1_end - line1_start;
    let s2 = line2_end - line2_start;
    
    let s = (-s1.y() * (line1_start.x() - line2_start.x()) + s1.x() * (line1_start.y() - line2_start.y())) / (-s2.x() * s1.y() + s1.x() * s2.y());
    let t = ( s2.x() * (line1_start.y() - line2_start.y()) - s2.y() * (line1_start.x() - line2_start.x())) / (-s2.x() * s1.y() + s1.x() * s2.y());

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        Some(Vec2::new(
                line1_start.x() + (t + s1.x()), 
                line1_start.y() + (t * s1.y())
        ));
    }  

    None
}


struct Crossroad;

struct Building;

pub struct City {
    streets: Vec<Street>,
    crossroads: Vec<Crossroad>,
    buildings: Vec<Buildings>,


}
*/