
//#![cfg(feature = "stable_graph")]

extern crate petgraph;

use petgraph::prelude::*;
use petgraph::csr::DefaultIx;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::IntoEdgeReferences;

use bevy::{
    prelude::*,
};

use bevy_prototype_lyon::prelude::*;
use std::f32::consts::{FRAC_PI_6, PI};


use std::fmt;

use crate::math::line::{ Line, Parallel };
use crate::math::operations:: { Valid };
use crate::math::polygon::Polygon;

use rand::Rng;

const MIN_ROAD_LENGTH: f32 = 300.0;


fn generate_random_color() -> Color {
    let mut rng = rand::thread_rng();

    Color::rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

struct GraphEntityIndex {
    entity: Entity
}

pub struct RoadSystem {
    graph: StableUnGraph::<RoadIntersection, ()>
}

pub struct RoadIntersection {
    pub position: Vec2,
}


impl RoadIntersection {
    pub fn new(position: Vec2) -> RoadIntersection {
        RoadIntersection { position : position }
    }
}

impl RoadSystem {
    pub fn new() -> RoadSystem {
        RoadSystem { 
            graph: StableUnGraph::<RoadIntersection, ()>::with_capacity(0,0)
        }
    }

    pub fn insert_intersection(&mut self, intersection: RoadIntersection) -> NodeIndex<DefaultIx> {
        self.graph.add_node(intersection)
  
    }

    /// Removes an intersection (node) of the road system. 
    /// Warning: Removes all connected roads as well.
    pub fn remove_intersection(&self, intersection: &RoadIntersection) {
        todo!();
    }

    pub fn point_intersect_connection(&self, point: Vec2) -> Option<Line> {
        for edge_index in self.graph.edge_indices() {
            if let Some((start, end)) = self.graph.edge_endpoints(edge_index) {

                let source = self.graph.node_weight(start);
                let target = self.graph.node_weight(end);
        
                if let (
                    Some(source), 
                    Some(target),
                ) = (source, target) {
                    let source = source.position;
                    let target = target.position;

                    let cross = (point - source).perp_dot(target - source);
                   
                }  
            }
        }

        None
    }

    fn find_intersections(&self, intersection1: NodeIndex, intersection2: NodeIndex) -> Vec<(EdgeIndex, Vec2)> {
        let mut intersections = Vec::new();

        for edge_index in self.graph.edge_indices() {
            if let Some((start, end)) = self.graph.edge_endpoints(edge_index) {
                if let (Some(source), Some(target)) = (self.graph.node_weight(intersection1), self.graph.node_weight(intersection2)) {
                    if let Some(intersection) = self.intersects(&source.position, &target.position, start, end) {                
                        intersections.push((edge_index, intersection));
                    }  
                }              
            }
        }

        intersections
    }

    fn find_intersections_vec2(&self, intersection1: &Vec2, intersection2: &Vec2) -> Vec<(EdgeIndex, Vec2)> {
        let mut intersections = Vec::new();
       
        for edge_index in self.graph.edge_indices().into_iter().step_by(2) {
            if let Some((start, end)) = self.graph.edge_endpoints(edge_index) {
                if let Some(intersection) = self.intersects(intersection1, intersection2, start, end) {                
                    intersections.push((edge_index, intersection));
                }                
            }
        }

        intersections
    }

    /// Sorts found intersections by the distance of each intersection to a reference intersection
    fn sort_intersections_by_distance(&self, reference_intersection: &Vec2, intersections: &mut Vec<(EdgeIndex, Vec2)>) {
             
        intersections.sort_by(|a, b| {
            let a = (a.1 - *reference_intersection).length() as i32;
            let b = (b.1 - *reference_intersection).length() as i32;
            a.cmp(&b)
        });
    }

    /// Creates a street between the two intersections
    pub fn connect_intersections(&mut self, intersection1: NodeIndex<DefaultIx>, intersection2: NodeIndex<DefaultIx>) { 
        
        // Find all edges intersecting the new one
        let mut intersections = self.find_intersections(intersection1, intersection2);
        
        // Sort them by distance to start to avoid wrong edges
        {
            let intersection1 = self.graph.node_weight(intersection1).unwrap();   
            self.sort_intersections_by_distance(&intersection1.position, &mut intersections);
        }
        
        if intersections.is_empty() {
            self.graph.add_edge(intersection1, intersection2, ());
        }
        
        let mut current = intersection2;
        while let Some(current_intersection) = intersections.pop() {
            let next = self.insert_intersection(RoadIntersection::new(current_intersection.1));

            self.graph.add_edge(current, next, ());

            

            // Split each road into two which are intersected by the new road
            if let Some((old_start, old_end)) = self.graph.edge_endpoints(current_intersection.0) {
                self.graph.add_edge(old_start, next, ());
                self.graph.add_edge(next, old_end, ());

                self.graph.remove_edge(current_intersection.0);            
            }

            current = next;
        }

        self.graph.add_edge(current, intersection1, ());
    }

    /// Returns if a new connection is valid in the road network by looking into the length of the new connection as well as
    /// into the length of segments of existing connection if they are intersected by the new connection. 
    /// 
    /// * `point1` - First point of the connection to be validated
    /// * `point2` - Second point of the connection to be validated
    pub fn valid_connection(&mut self, point1: &Vec2, point2: &Vec2) -> bool {
        let mut intersections = self.find_intersections_vec2(point1, point2);
        self.sort_intersections_by_distance(&point1, &mut intersections);  

        // The new road would not intersect any existing one so we only need to check for minimal road length
        if intersections.is_empty() && (*point2 - *point1).length() >= MIN_ROAD_LENGTH {
            return true;
        }

        // The new road would intersect existing roads. We need to check two things:
        // 1. If the new road segments all have minimal road length (road segment from the intersections of exsiting roads)
        // 2. All existing, intersected roads are separated into two road segments: Each of this segment must also have the minimal road length
        let mut current = *point1;
        while let Some(current_intersection) = intersections.pop() {
            let next = &current_intersection.1;

            // Check for (1): minimal road length of individual road segments
            if (current - *next).length() < MIN_ROAD_LENGTH {     
                println!("1 {}", (current - *next).length());           
                return false;
            }
      
            // Check for (2)
            if let Some((old_start, old_end)) = self.graph.edge_endpoints(current_intersection.0) {
                let old_start = self.graph.node_weight(old_start).unwrap();
                let old_end = self.graph.node_weight(old_end).unwrap();

                if (old_start.position - *next).length() < MIN_ROAD_LENGTH {
                    println!("2.1. {}", (old_start.position - *next).length());
                    return false;
                }

                if (*next - old_end.position).length() < MIN_ROAD_LENGTH {
                    println!("2.2. {}", (current - current_intersection.1).length());
                    return false;
                }          
            }

            current = next.clone();
        }

        true
    }

    /// Removes a street between the two intersections
    ///
    /// Warning: If one of the intersections has after removel of the street no further
    /// connections, it will also be removed
    pub fn disconnect_intersections(&self, intersection1: NodeIndex<DefaultIx>, intersection2: NodeIndex<DefaultIx>) {
        todo!();
    }

    pub fn update(&self, commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>, mut meshes: &mut ResMut<Assets<Mesh>>) {
        // build the intersections
        for (_, node) in self.graph.node_references() {
            commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),            
                transform: Transform::from_translation(Vec3::new(node.position.x(), node.position.y(), 2.0)),            
                sprite: Sprite::new(Vec2::new(40.0, 40.0)),
                ..Default::default()
            })
            .with(RoadIntersection{
                position: node.position
            });
        }

        // build the connections
        for a in self.graph.edge_references() {
            let source = self.graph.node_weight(a.source());
            let target = self.graph.node_weight(a.target());

            if let (Some(source), Some(target)) = (source, target) {
                let street_vector = target.position - source.position;
                let street_length = street_vector.length();

                // Avoid short road segments
                if street_length < 10.0 {
                    continue;
                }
                //let street_center = source.position + street_vector / street_length * street_length / 2.0;
                //let rotation = -street_vector.angle_between(Vec2::new(1.0, 0.0)); 

                let line = Line {
                    point1: source.position,
                    point2: target.position
                };

                let parallel_line = line.parralel(5.0);
                let blue = materials.add(Color::rgb(0.1, 0.4, 0.5).into());

                let mut builder = PathBuilder::new();

                if !line.valid() || !parallel_line.valid() {
                    println!("FOo");
                }

                // Using that builder, you can build any shape:
                builder.move_to(point(line.point1.x(), line.point1.y()));
                builder.line_to(point(line.point2.x(), line.point2.y()));
                builder.line_to(point(parallel_line.point2.x(), parallel_line.point2.y()));
                builder.line_to(point(parallel_line.point1.x(), parallel_line.point1.y()));
                
                builder.close(); // This draws a line to (0.0, 0.0)                            

                let path = builder.build();
                commands
                .spawn(path.fill(
                    blue,
                    &mut meshes,
                    Vec3::new(0.0, 0.0, 0.0),
                    &FillOptions::default(),
                ))
                .with(line);      
            }
        }
    }

    pub fn intersects(
        &self, 
        source: &Vec2, 
        target: &Vec2,
        other_source: NodeIndex<DefaultIx>, 
        other_target: NodeIndex<DefaultIx>
    ) -> Option<Vec2> {
        let other_source = self.graph.node_weight(other_source);
        let other_target = self.graph.node_weight(other_target);

        if let (
            Some(other_source), 
            Some(other_target)
        ) = (other_source, other_target) {
            let s1 = *target - *source;
            let s2 = other_target.position - other_source.position;
    
            let s = (-s1.y() * (source.x() - other_source.position.x()) + s1.x() * (source.y() - other_source.position.y())) / (-s2.x() * s1.y() + s1.x() * s2.y());
            let t = ( s2.x() * (source.y() - other_source.position.y()) - s2.y() * (source.x() - other_source.position.x())) / (-s2.x() * s1.y() + s1.x() * s2.y());
        
            if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
                return Some(Vec2::new(
                        source.x() + (t * s1.x()), 
                        source.y() + (t * s1.y())
                ));
            }  
        }

        None
    }     
}

impl fmt::Display for RoadSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RoadSystem Num_Nodes:{}, Num_Edges:{}", self.graph.node_count(), self.graph.edge_count())
    }
}