use std::collections::HashSet;

use gloo::{events::EventListener, utils::document};
use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use yew::{html, Component, Context, NodeRef};

use crate::{geometry::Vector, world::World};

pub enum Msg {
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
    Timer(f64),
}

pub struct App {
    canvas_ref: NodeRef,

    context: Option<CanvasRenderingContext2d>,

    world: World,

    last_tick: f64,
    gun_cooldown: f64,
    lost: bool,

    down_list: HashSet<String>,

    _keydown_listener: EventListener,
    _keyup_listener: EventListener,
    _frame: Option<AnimationFrame>,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let onkeydown = ctx.link().callback(Msg::KeyDown);
        let keydown_listener = EventListener::new(&document(), "keydown", move |e| {
            let e = e.clone().unchecked_into::<KeyboardEvent>();
            onkeydown.emit(e);
        });

        let onkeyup = ctx.link().callback(Msg::KeyUp);
        let keyup_listener = EventListener::new(&document(), "keyup", move |e| {
            let e = e.clone().unchecked_into::<KeyboardEvent>();
            onkeyup.emit(e);
        });

        Self {
            canvas_ref: NodeRef::default(),
            context: None,
            world: World::new(Vector::new(600.0, 1000.0)),
            down_list: HashSet::new(),
            last_tick: -1.0,
            lost: false,
            gun_cooldown: 0.0,
            _keydown_listener: keydown_listener,
            _keyup_listener: keyup_listener,
            _frame: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(e) => {
                let key = e.code();
                if !self.down_list.contains(&key) {
                    self.down_list.insert(key);
                }
                false
            }
            Msg::KeyUp(e) => {
                let key = e.code();
                if self.down_list.contains(&key) {
                    self.down_list.remove(&key);
                }
                false
            }
            Msg::Timer(time) => {
                if !self.lost {
                    self._frame = Some({
                        let link = ctx.link().clone();
                        request_animation_frame(move |time| link.send_message(Msg::Timer(time)))
                    });
                } else {
                    return false;
                }

                if self.last_tick < 0.0 {
                    self.last_tick = time;
                    return false;
                }
                let delta_time = (time - self.last_tick) / 1000.0;
                self.last_tick = time;

                if self.gun_cooldown > 0.0 {
                    self.gun_cooldown -= delta_time;
                }

                let mut delta = Vector::zero();

                for key in self.down_list.iter() {
                    match key.as_ref() {
                        "ArrowLeft" => delta.x += -300.0,
                        "ArrowRight" => delta.x += 300.0,
                        "ArrowUp" => delta.y += -300.0,
                        "ArrowDown" => delta.y += 300.0,
                        "Space" => {
                            if self.gun_cooldown <= 0.0 {
                                self.world.shoot(Vector::new(0.0, -500.0));
                                self.gun_cooldown += 0.2;
                            }
                        }
                        _ => (),
                    }
                }
                self.world.move_player(delta * delta_time);
                match self.world.tick(delta_time) {
                    crate::world::TickResult::None => {
                        self.world.draw(self.context.as_ref().unwrap());
                    }
                    crate::world::TickResult::Win => {
                        self.lost = true;
                        self.game_over("rgba(100,255,100,255)");
                    }
                    crate::world::TickResult::Loose => {
                        self.lost = true;
                        self.game_over("rgba(255,100,100,255)");
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        html! {
            <div class="main-block">
                <canvas
                    class="main-canvas"
                    ref={self.canvas_ref.clone()}
                    width={600}
                    height={1000}
                    onkeydown={ctx.link().callback(Msg::KeyDown)}
                />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

            self.context = Some(
                canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap(),
            );

            self._frame = Some({
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Timer(time)))
            });

            self.world.draw(self.context.as_ref().unwrap());
        }
    }
}

impl App {
    fn game_over(&self, color: &str) {
        let context = self.context.as_ref().unwrap();
        context.save();
        context.set_global_composite_operation("copy").unwrap();
        context.set_fill_style(&JsValue::from_str(color));
        context.fill_rect(0.0, 0.0, 601.0, 1000.0);
        context.restore();
    }
}
