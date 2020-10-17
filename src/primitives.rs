
/*
pub mod primitives {
    pub struct BezierCurve {
        pub curve_points: Vec<bevy::math::Vec2>,
        pub control_points: Vec<bevy::math::Vec2>
    }

    impl BezierCurve {
        fn change_last_control_point(&mut self, new_control_point: &mut bevy::math::Vec2) {
            match self.control_points.last_mut() {
                Some(mut _v) => _v = new_control_point,
                None => {},
            };
        }

        
        pub fn add_control_point(&mut self, new_control_point: &bevy::math::Vec2) {
            self.control_points.push(*new_control_point);
        }
        

        
        fn calculate_curve_point(&self, points: Vec<bevy::math::Vec2>, mu: f32) -> bevy::math::Vec2 {
            let mut kn: f32 = 0.0;
            let mut nn: f32 = 0.0;
            let mut nkn: f32 = 0.0;
            let mut blend: f32 = 0.0;
            let mut muk: f32 = 1.0;
            let mut munk: f32 = num::pow(1.0 - mu, points.len());

            let mut b = bevy::math::Vec2::new(0.0, 0.0);
            
            let n = points.len() as f32;
            for (i, point) in points.iter().enumerate() {
                nn = n;
                kn = i as f32;
                nkn = n - i as f32;
                blend = muk * munk;
                muk *= mu;
                munk /= 1.0 - mu;
              
                while nn >= 1.0 {
                    blend *= nn;
                    nn = nn - 1.0;
                    
                    if kn > 1.0 {
                        blend = blend / kn;
                        kn = kn - 1.0;
                    }
                 
                    if nkn > 1.0 {
                       blend = blend / nkn;
                        nkn = nkn - 1.0;
                    }
                }

                b.set_x(b.x() + point.x() * blend);
                b.set_y(b.y() + point.y() * blend);
            }

            println!("x: {}, y: {}", b.x(), b.y());
        
           b
        }

        pub fn calculate(&mut self, step: f32) {
            self.curve_points.clear();

            assert_ne!(step, 0.0);

            for i in 0..(1.0 / step) as i32 {
                self.curve_points.push(self.calculate_curve_point(self.control_points.clone(), i as f32 * step));
            }
        }
    }
}

*/