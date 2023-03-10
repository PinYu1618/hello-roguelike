use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
