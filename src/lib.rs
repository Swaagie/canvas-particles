mod utils;

use wasm_bindgen::prelude::*;
use nalgebra::Vector3;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, physics-engine!");
}

#[derive(Debug)]
struct Object {
    velocity: Vector3<f32>,
    position: Vector3<f32>,
    force: Vector3<f32>,
    mass: f32,
}

#[derive(Debug)]
struct World<'a> {
    objects: Vec<&'a mut Object>,
    gravity: Vector3<f32>,
}

impl<'a> World<'a> {
    fn new() -> World<'a> {
        World {
            objects: vec![],
            gravity: Vector3::new(0.0, -9.81, 0.0)
        }
    }

    fn addObject(&self, object: &'a mut Object) {
        self.objects.push(object);
    }

    fn removeObject(&self, object: &Object) {

    }

    fn step(&self, dt: f32) {
        for &mut obj in self.objects[..] {
            obj.force = obj.mass * self.gravity;
            obj.velocity += obj.force / obj.mass * dt;
            obj.position += obj.velocity * dt;
            obj.force = Vector3::new(0.0, 0.0, 0.0);
        }
	}
}