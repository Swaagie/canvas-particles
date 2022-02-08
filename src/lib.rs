extern crate web_sys;
extern crate cfg_if;

mod utils;

use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use nalgebra::Vector3;
use web_sys::{CanvasRenderingContext2d};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    id: String,
    color: JsValue,
    velocity: Vector3<f64>,
    position: Vector3<f64>,
    force: Vector3<f64>,
    mass: f64,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Position(pub f64, pub f64, pub f64);

#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {
    bodies: Vec<Body>,
    gravity: Vector3<f64>,
}

#[wasm_bindgen]
impl Body {
    pub fn new(mass: f64, color: JsValue, position: Option<Vec<f64>>, velocity: Option<Vec<f64>>, force: Option<Vec<f64>>) -> Body {
        let position = position.unwrap_or(vec![0.0, 0.0, 0.0]);
        let velocity = velocity.unwrap_or(vec![0.0, 0.0, 0.0]);
        let force = force.unwrap_or(vec![0.0, 0.0, 0.0]);

        // log!("mass {:?} pos {:?} v {:?}", mass, position, velocity);
        Body {
            id: String::from(""),
            color,
            velocity: Vector3::new(velocity[0],velocity[1], velocity[2]),
            position: Vector3::new(position[0], position[1], position[2]),
            force: Vector3::new(force[0], force[1], force[2]),
            mass
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        Universe {
            bodies: vec![],
            gravity: Vector3::new(0.0, -9.81, 0.0)
        }
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let canvas = ctx.canvas().unwrap();

        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        for body in self.bodies.iter() {
            let position = body.position;

            ctx.begin_path();
            ctx.arc(position[0], -position[1] + canvas.height() as f64, (body.mass / 10e3).round(), 0.0, 2.0 * PI)?;
            ctx.set_fill_style(&body.color);
            ctx.fill();
        }

        Ok(())
    }

    // Ideally this would push a reference, by wasm does not allow lifetime specifiers
    pub fn add_body(&mut self, id: String, mut body: Body) {
        body.id = id;
        self.bodies.push(body);
    }

    // change body to id
    pub fn remove_body(&mut self, body: Body) {
        if let Some(i) = self.bodies.iter().position(|b| b == &body) {
            self.bodies.remove(i);
        }
    }

    pub fn get_position(&mut self, id: String) -> Option<Position> {
        match self.bodies.iter().position(|b| b.id == id) {
            Some(i) => {
                let p = self.bodies[i].position;
                Some(Position(p[0], p[1], p[2]))
            },
            _ => None,
        }
    }

    pub fn tick(&mut self, dt: Option<f64>) {
        let dt = dt.unwrap_or(1.0/60.0);

        for obj in &mut self.bodies[..] {
            obj.force = obj.mass * self.gravity;
            obj.velocity += obj.force / obj.mass * dt;
            obj.position += obj.velocity * dt;
            obj.force = Vector3::new(0.0, 0.0, 0.0);
        }
	}
}