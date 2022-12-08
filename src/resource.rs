use crate::prelude::*;

pub mod camera;
pub mod map;

#[derive(Resource, Default)]
pub struct Key {
    pub key: Option<VirtualKeyCode>,
}