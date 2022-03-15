use std::{collections::HashSet, hash::Hash};

use gloo::{events::EventListener, utils::document};
use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use yew::{html, Component, Context, NodeRef, Properties};

pub enum Msg {
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
    Timer(f64),
}

pub struct App {
    canvas_ref: NodeRef,

    context: Option<CanvasRenderingContext2d>,

    gg_x: f64,
    gg_y: f64,

    gg_dx: f64,
    gg_dy: f64,

    last_tick: f64,

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
            gg_x: 300.0,
            gg_y: 850.0,
            gg_dx: 0.0,
            gg_dy: 0.0,
            down_list: HashSet::new(),
            last_tick: -1.0,
            _keydown_listener: keydown_listener,
            _keyup_listener: keyup_listener,
            _frame: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(e) => {
                let key = e.key();
                if self.down_list.contains(&key) {
                    return false
                }
                match key.as_ref() {
                    "ArrowLeft" => self.gg_dx += -100.0,
                    "ArrowRight" => self.gg_dx += 100.0,
                    "ArrowUp" => self.gg_dy += -100.0,
                    "ArrowDown" => self.gg_dy += 100.0,
                    _ => (),
                }
                self.down_list.insert(key);
                false
            }
            Msg::KeyUp(e) => {
                let key = e.key();
                if self.down_list.contains(&key) {
                    self.down_list.remove(&key);
                }
                match e.key().as_ref() {
                    "ArrowLeft" => self.gg_dx -= -100.0,
                    "ArrowRight" => self.gg_dx -= 100.0,
                    "ArrowUp" => self.gg_dy -= -100.0,
                    "ArrowDown" => self.gg_dy -= 100.0,
                    _ => (),
                }
                false
            }
            Msg::Timer(time) => {
                self._frame = Some({
                    let link = ctx.link().clone();
                    request_animation_frame(move |time| link.send_message(Msg::Timer(time)))
                });

                if self.last_tick < 0.0 {
                    self.last_tick = time;
                    return false;
                }
                let delta = (time - self.last_tick) / 1000.0;
                self.last_tick = time;

                self.gg_x += self.gg_dx * delta;
                self.gg_y += self.gg_dy * delta;
                self.draw();

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

            self.draw();
        }
    }
}

impl App {
    fn draw(&self) {
        let context = self.context.as_ref().unwrap();

        context.save();
        context.set_global_composite_operation("copy").unwrap();
        context.set_fill_style(&JsValue::from_str("rgba(0,0,0,0)"));
        context.fill_rect(0.0, 0.0, 600.0, 1000.0);
        context.restore();

        context.begin_path();
        context.set_fill_style(&JsValue::from_str("red"));
        context
            .arc(self.gg_x, self.gg_y, 20.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.fill();
        context.close_path();
    }
}
