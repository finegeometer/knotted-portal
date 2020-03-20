#![forbid(unsafe_code)]

mod fps;
mod modeling;
mod render;

/// This module is a mirror of my GLSL code. It may not be good *Rust* code, but I think having it match the GLSL is worth it.
mod portal;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    web_sys::window()
        .unwrap_throw()
        .request_animation_frame(&State::new().0.borrow().animation_frame_closure)
        .unwrap_throw();
}

pub enum Msg {
    Click,
    MouseMove([i32; 2]),
    KeyDown(String),
    KeyUp(String),
}

#[derive(Clone)]
struct State(Rc<RefCell<Model>>);

struct Model {
    animation_frame_closure: js_sys::Function,
    keys: HashSet<String>,
    fps: Option<fps::FrameCounter>,
    renderer: render::Renderer,

    window: web_sys::Window,
    document: web_sys::Document,
    canvas: web_sys::HtmlCanvasElement,

    player: Player,
    balls: Vec<Ball>,
}

impl State {
    fn new() -> Self {
        let out = Self(Rc::new(RefCell::new(Model::new())));

        {
            let model: &mut Model = &mut out.0.borrow_mut();

            out.event_listener(&model.canvas, "mousedown", move |_| Msg::Click);
            out.event_listener(&model.canvas, "mousemove", |evt| {
                let evt = evt.dyn_into::<web_sys::MouseEvent>().unwrap_throw();
                Msg::MouseMove([evt.movement_x(), evt.movement_y()])
            });
            out.event_listener(&model.document, "keydown", |evt| {
                let evt = evt.dyn_into::<web_sys::KeyboardEvent>().unwrap_throw();
                Msg::KeyDown(evt.key())
            });
            out.event_listener(&model.document, "keyup", |evt| {
                let evt = evt.dyn_into::<web_sys::KeyboardEvent>().unwrap_throw();
                Msg::KeyUp(evt.key())
            });

            let state = out.clone();
            let closure: Closure<dyn FnMut(f64)> = Closure::wrap(Box::new(move |timestamp| {
                state.frame(timestamp);
            }));
            model.animation_frame_closure =
                closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
            closure.forget();
        }

        out
    }

    fn update(&self, msg: Msg) {
        let model: &mut Model = &mut self.0.borrow_mut();

        match msg {
            Msg::Click => {
                if model.document.pointer_lock_element().is_none() {
                    model.canvas.request_pointer_lock();
                }
            }
            Msg::KeyDown(k) => {
                model.keys.insert(k.to_lowercase());
            }
            Msg::KeyUp(k) => {
                model.keys.remove(&k.to_lowercase());
            }
            Msg::MouseMove([x, y]) => {
                if model.document.pointer_lock_element().is_some() {
                    model.player.theta += x as f32 * 3e-3;
                    model.player.phi -= y as f32 * 3e-3;

                    model.player.phi = model
                        .player
                        .phi
                        .max(-std::f32::consts::FRAC_PI_2 + 0.001)
                        .min(std::f32::consts::FRAC_PI_2 - 0.001);
                }
            }
        }
    }

    fn frame(&self, timestamp: f64) {
        let model: &mut Model = &mut self.0.borrow_mut();

        if let Some(fps) = &mut model.fps {
            let dt = fps.frame(timestamp);

            model.move_player(dt as f32);
            model.move_balls(dt as f32);
            model.view();
        } else {
            model.fps = Some(<fps::FrameCounter>::new(timestamp));
        }

        model
            .window
            .request_animation_frame(&model.animation_frame_closure)
            .unwrap_throw();
    }

    fn event_listener(
        &self,
        target: &web_sys::EventTarget,
        event: &str,
        msg: impl Fn(web_sys::Event) -> Msg + 'static,
    ) {
        let state = self.clone();
        let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::wrap(Box::new(move |evt| {
            state.update(msg(evt));
        }));
        target
            .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
            .unwrap_throw();
        closure.forget();
    }
}

impl Model {
    fn new() -> Self {
        let window = web_sys::window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let body = document.body().unwrap_throw();

        body.style()
            .set_property("background-color", "#111111")
            .unwrap_throw();

        let canvas = document
            .create_element("canvas")
            .unwrap_throw()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap_throw();
        canvas.set_attribute("width", "800px").unwrap_throw();
        canvas.set_attribute("height", "800px").unwrap_throw();
        canvas
            .style()
            .set_property("grid-area", "canvas")
            .unwrap_throw();
        body.append_child(&canvas).unwrap_throw();

        let static_geometry = modeling::trefoil()
            .chain(modeling::skybox())
            .chain(modeling::ground());

        let balls = vec![
            Ball::new([0.6, 0.6, 0.8, 1.0], 0, |t| {
                let (s, c) = t.sin_cos();
                nalgebra::Vector3::new(2. * s, -2. * c, 0.)
            }),
            Ball::new([0.8, 0.6, 0.2, 1.0], 3, |t| {
                let (s, c) = t.sin_cos();
                nalgebra::Vector3::new(0.1, -3. + c, s)
            }),
            Ball::new([0.2, 0.3, 0.9, 1.0], 3, |t| {
                let (s, c) = t.sin_cos();
                let (s2, c2) = (2. * t).sin_cos();
                nalgebra::Vector3::new(s + 2. * s2, c - 2. * c2 + 0.1, (3. * t).sin() + 0.5)
            }),
        ];

        Self {
            animation_frame_closure: JsValue::undefined().into(),
            fps: None,
            keys: HashSet::new(),
            renderer: render::Renderer::new(&canvas, static_geometry),

            player: Player::new(),

            window,
            document,
            canvas,

            balls,
        }
    }

    fn move_player(&mut self, dt: f32) {
        let speed = 0.5;

        let mut v = nalgebra::Vector3::zeros();
        if self.keys.contains(" ") {
            v += nalgebra::Vector3::z() * dt * speed;
        }
        if self.keys.contains("shift") {
            v -= nalgebra::Vector3::z() * dt * speed;
        }
        if self.keys.contains("w") {
            v -= nalgebra::Vector3::x() * dt * speed;
        }
        if self.keys.contains("s") {
            v += nalgebra::Vector3::x() * dt * speed;
        }
        if self.keys.contains("a") {
            v -= nalgebra::Vector3::y() * dt * speed;
        }
        if self.keys.contains("d") {
            v += nalgebra::Vector3::y() * dt * speed;
        }

        v = nalgebra::UnitQuaternion::new(-self.player.theta * nalgebra::Vector3::z()) * v;
        self.player.travel(v);
    }

    fn move_balls(&mut self, dt: f32) {
        for ball in self.balls.iter_mut() {
            ball.travel(dt);
        }
    }

    fn view(&self) {
        self.renderer.render(
            render::Uniforms {
                light_dir: nalgebra::Vector3::new(1.0, 1.0, 1.0).normalize(),
                player_isometry: self.player.isometry(),
                player_world: self.player.world,
            },
            self.balls.iter().flat_map(Ball::geometry).collect(),
        )
    }
}

struct Player {
    pos: nalgebra::Vector3<f32>,
    theta: f32,
    phi: f32,
    world: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            pos: nalgebra::Vector3::new(5.0, 0.0, 0.0),
            theta: 0.,
            phi: 0.,
            world: 0,
        }
    }

    // Player space -> World space
    fn isometry(&self) -> nalgebra::Isometry3<f32> {
        let (st, ct) = self.theta.sin_cos();
        let (sp, cp) = self.phi.sin_cos();

        nalgebra::Isometry3::face_towards(
            &nalgebra::Point3 { coords: self.pos },
            &nalgebra::Point3 {
                coords: self.pos + nalgebra::Vector3::new(ct * cp, -st * cp, -sp),
            },
            &nalgebra::Vector3::z(),
        )
    }

    fn travel(&mut self, v: nalgebra::Vector3<f32>) {
        let newpos = self.pos + v;
        portal::travel(&mut self.world, self.pos, newpos);
        self.pos = newpos;
    }
}

struct Ball {
    color: [f32; 4],
    path: fn(f32) -> nalgebra::Vector3<f32>,
    pos: nalgebra::Vector3<f32>,
    t: f32,
    world: i32,
}

impl Ball {
    fn new(color: [f32; 4], world: i32, path: fn(f32) -> nalgebra::Vector3<f32>) -> Self {
        Self {
            color,
            path,
            t: 0.,
            pos: path(0.),
            world,
        }
    }

    fn travel(&mut self, dt: f32) {
        let t = self.t + dt;
        let pos = (self.path)(t);
        portal::travel(&mut self.world, self.pos, pos);
        self.t = t;
        self.pos = pos;
    }

    fn geometry(&self) -> impl IntoIterator<Item = modeling::Triangle> {
        modeling::ball((self.path)(self.t), self.world, self.color)
    }
}
