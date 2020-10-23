
//#![cfg(feature = "stable_graph")]

extern crate petgraph;

use petgraph::prelude::*;
use petgraph::csr::DefaultIx;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::IntoEdgeReferences;

use bevy::{
    prelude::*,
};


use std::fmt;


use rand::Rng;


fn generate_random_color() -> Color {
    let mut rng = rand::thread_rng();

    Color::rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

struct GraphEntityIndex {
    entity: Entity
}

pub struct RoadSystem {
    graph: StableGraph::<RoadIntersection, ()>
}

pub struct Road {}

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
            graph: StableGraph::<RoadIntersection, ()>::new()
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

    pub fn point_intersect_connection(&self, point: Vec2) -> Option<Road> {
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

                    println!("{}", cross);
                    
                    
                }  
            }
        }

        None
    }

    fn find_intersections(&self, intersection1: NodeIndex, intersection2: NodeIndex) -> Vec<(EdgeIndex, Vec2)> {
        let mut intersections = Vec::new();

        for edge_index in self.graph.edge_indices() {
            if let Some((start, end)) = self.graph.edge_endpoints(edge_index) {
                if let Some(intersection) = self.intersects(intersection1, intersection2, start, end) {                
                    intersections.push((edge_index, intersection));
                }                
            }
        }

        intersections
    }

    /// Creates a street between the two intersections
    pub fn connect_intersections(&mut self, intersection1: NodeIndex<DefaultIx>, intersection2: NodeIndex<DefaultIx>) { 
        
        // Find all edges intersecting the new one
        let mut intersections = self.find_intersections(intersection1, intersection2);
        
        // Sort them by distance to start to avoid wrong edges
        {
            let intersection1 = self.graph.node_weight(intersection1).unwrap();        
            intersections.sort_by(|a, b| {
                let a = (a.1 - intersection1.position).length() as i32;
                let b = (b.1 - intersection1.position).length() as i32;
                a.cmp(&b)
            });
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

    /// Removes a street between the two intersections
    ///
    /// Warning: If one of the intersections has after removel of the street no further
    /// connections, it will also be removed
    pub fn disconnect_intersections(&self, intersection1: NodeIndex<DefaultIx>, intersection2: NodeIndex<DefaultIx>) {
        todo!();
    }

    pub fn update(&self, commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
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
                let street_center = source.position + street_vector / street_length * street_length / 2.0;
                let rotation = -street_vector.angle_between(Vec2::new(1.0, 0.0)); 

                //let new_street = city::StraightStreet::new(intersection, old_end);
                commands
                .spawn(SpriteComponents {
                    material: materials.add(generate_random_color().into()),         
                    transform: Transform::from_translation_rotation(Vec3::new(street_center.x(), street_center.y(), 0.0), Quat::from_rotation_z(rotation)),            
                    sprite: Sprite::new(Vec2::new(street_length, 10.0)),
                    ..Default::default()
                })
                .with(Road {
                });            
            }
        }
    }


    pub fn intersects(
        &self, 
        source: NodeIndex<DefaultIx>, 
        target: NodeIndex<DefaultIx>, 
        other_source: NodeIndex<DefaultIx>, 
        other_target: NodeIndex<DefaultIx>
    ) -> Option<Vec2> {
        let source = self.graph.node_weight(source);
        let target = self.graph.node_weight(target);

        let other_source = self.graph.node_weight(other_source);
        let other_target = self.graph.node_weight(other_target);

        if let (
            Some(source), 
            Some(target),
            Some(other_source), 
            Some(other_target)
        ) = (source, target, other_source, other_target) {
            let s1 = target.position - source.position;
            let s2 = other_target.position - other_source.position;
    
            let s = (-s1.y() * (source.position.x() - other_source.position.x()) + s1.x() * (source.position.y() - other_source.position.y())) / (-s2.x() * s1.y() + s1.x() * s2.y());
            let t = ( s2.x() * (source.position.y() - other_source.position.y()) - s2.y() * (source.position.x() - other_source.position.x())) / (-s2.x() * s1.y() + s1.x() * s2.y());
        
            if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
                return Some(Vec2::new(
                        source.position.x() + (t * s1.x()), 
                        source.position.y() + (t * s1.y())
                ));
            }  
        }

        None
    }     

    /*
    pub fn intersects(&self, edge: petgraph::stable_graph::EdgeReference<Directed>, other_edge: petgraph::stable_graph::EdgeReference<Directed>) -> Option<Vec2>{
        let source = self.graph.node_weight(edge.source());
        let target = self.graph.node_weight(edge.target());
        let other_source = self.graph.node_weight(other_edge.source());
        let other_target = self.graph.node_weight(other_edge.target());

        if let (
            Some(source), 
            Some(target),
            Some(other_source), 
            Some(other_target)
        ) = (source, target, other_source, other_target) {
            let s1 = target.position - source.position;
            let s2 = other_target.position - other_source.position;
    
            let s = (-s1.y() * (source.position.x() - other_source.position.x()) + s1.x() * (source.position.y() - other_source.position.y())) / (-s2.x() * s1.y() + s1.x() * s2.y());
            let t = ( s2.x() * (source.position.y() - other_source.position.y()) - s2.y() * (source.position.x() - other_source.position.x())) / (-s2.x() * s1.y() + s1.x() * s2.y());
        
            if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
                return Some(Vec2::new(
                        source.position.x() + (t * s1.x()), 
                        source.position.y() + (t * s1.y())
                ));
            }  
        }

        None
    }     
    */  
}

impl fmt::Display for RoadSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RoadSystem Num_Nodes:{}, Num_Edges:{}", self.graph.node_count(), self.graph.edge_count())
    }
}