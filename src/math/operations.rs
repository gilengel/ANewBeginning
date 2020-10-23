use bevy::prelude::*;

pub trait Center {
    fn center(&self) -> Vec2;
}

pub trait Intersects<T> {
    fn intersects(&self, other: &T) -> bool;
}

pub trait Inside<T> {
    fn inside(&self, other_object: &T) -> bool;
}

pub trait Overlaps<T> {
    fn overlaps(&self, other_object: &T) -> bool;
}