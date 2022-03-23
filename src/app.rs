use std::collections::HashSet;

use gloo::{events::EventListener, utils::document};
use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    CanvasRenderingContext2d, HtmlAudioElement, HtmlCanvasElement, HtmlImageElement, KeyboardEvent,
};
use yew::{html, Component, Context, NodeRef};

use crate::{
    audio::AudioManager,
    download::{download_audio, download_image, Download},
    geometry::Vector,
    level::{l1, l2, Level},
    textures::TextureManager,
    world::{BulletType, World},
};

pub enum Msg {
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
    Timer(f64),
    DownloadRequested(Download),
    ImageDownloaded(String, HtmlImageElement),
    AudioDownloaded(String, HtmlAudioElement),
}

pub struct App {
    canvas_ref: NodeRef,

    context: Option<CanvasRenderingContext2d>,

    world: World,

    last_tick: f64,
    gun_cooldown: f64,
    bullet_type: BulletType,

    game_state: GameState,

    down_list: HashSet<String>,

    texture_manager: TextureManager,
    audio_manager: AudioManager,
    unfinished_downloads: usize,

    current_level: usize,
    levels: Vec<Level>,

    _keydown_listener: EventListener,
    _keyup_listener: EventListener,
    _frame: Option<AnimationFrame>,
}

#[derive(PartialEq)]
enum GameState {
    Loading,
    FinishLoading,
    Playing,
    LevelFinished,
    Lost,
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

        let levels = vec![l1(), l2()];
        let current_level = 0;

        Self {
            canvas_ref: NodeRef::default(),
            context: None,
            world: World::new(Vector::new(600.0, 1000.0), levels[current_level].clone()),
            down_list: HashSet::new(),
            last_tick: -1.0,
            game_state: GameState::Loading,
            bullet_type: BulletType::PlayerSniper,
            gun_cooldown: 0.0,

            texture_manager: TextureManager::new(),
            audio_manager: AudioManager::new(),
            unfinished_downloads: 0,

            current_level,
            levels,

            _keydown_listener: keydown_listener,
            _keyup_listener: keyup_listener,
            _frame: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(e) => {
                let key = e.code();
                if self.game_state == GameState::FinishLoading && key.as_str() == "Enter" {
                    self.request_frame(ctx);
                    return false;
                }
                if (self.game_state == GameState::Lost
                    || self.game_state == GameState::LevelFinished)
                    && key.as_str() == "Enter"
                {
                    self.world.reset(self.levels[self.current_level].clone());
                    self.start(ctx);
                    return false;
                }
                if key.as_str() == "ControlLeft" {
                    if self.bullet_type == BulletType::PlayerSniper {
                        self.bullet_type = BulletType::PlayerHeavy;
                    } else {
                        self.bullet_type = BulletType::PlayerSniper;
                    }
                    return false;
                }
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
                match self.game_state {
                    GameState::Playing => self.request_frame(ctx),
                    GameState::FinishLoading => self.start(ctx),
                    GameState::Lost => {
                        AudioManager::stop(&self.audio_manager.get("resources/resurrection.mp3"));
                        return false;
                    }
                    _ => return false,
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
                                self.world.shoot(
                                    Vector::new(0.0, -500.0),
                                    self.bullet_type,
                                    &self.audio_manager,
                                );
                                self.gun_cooldown += 0.2;
                            }
                        }
                        _ => (),
                    }
                }
                self.world.move_player(delta * delta_time);
                match self.world.tick(delta_time, &self.audio_manager) {
                    crate::world::TickResult::None => {
                        self.world
                            .draw(self.context.as_ref().unwrap(), &self.texture_manager);
                    }
                    crate::world::TickResult::Win => {
                        self.current_level += 1;
                        self.world.player_bullets += 1;
                        if self.current_level != self.levels.len() {
                            self.game_state = GameState::LevelFinished;
                            self.game_over(GameOverKind::LevelFinished);
                        } else {
                            self.game_state = GameState::Lost;
                            self.game_over(GameOverKind::Won);
                        }
                    }
                    crate::world::TickResult::Loose => {
                        self.game_state = GameState::Lost;
                        self.game_over(GameOverKind::Lost);
                    }
                }
                false
            }
            Msg::ImageDownloaded(path, img) => {
                log::debug!("Msg::ImageDownloaded");
                self.texture_manager.insert(path, img);
                self.unfinished_downloads -= 1;

                if self.unfinished_downloads == 0 {
                    self.game_state = GameState::FinishLoading;
                }

                false
            }
            Msg::AudioDownloaded(path, audio) => {
                self.audio_manager.insert(path, audio);
                self.unfinished_downloads -= 1;

                if self.unfinished_downloads == 0 {
                    self.game_state = GameState::FinishLoading;
                }

                false
            }
            Msg::DownloadRequested(download) => {
                self.game_state = GameState::Loading;
                self.unfinished_downloads += 1;

                self.game_over(GameOverKind::Loading);

                match download {
                    Download::Image(path) => {
                        let callback = ctx
                            .link()
                            .clone()
                            .callback(|(str, img)| Msg::ImageDownloaded(str, img));

                        spawn_local(async move {
                            let img = download_image(&path).await;
                            callback.emit((path, img));
                        });
                    }
                    Download::Audio(path) => {
                        let callback = ctx
                            .link()
                            .clone()
                            .callback(|(str, audio)| Msg::AudioDownloaded(str, audio));

                        spawn_local(async move {
                            let img = download_audio(&path).await;
                            callback.emit((path, img));
                        });
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

            for file in self.required_textures() {
                ctx.link()
                    .send_message(Msg::DownloadRequested(Download::Image(file)));
            }
            for file in self.required_audio() {
                ctx.link()
                    .send_message(Msg::DownloadRequested(Download::Audio(file)));
            }
        }
    }
}

impl App {
    fn start(&mut self, ctx: &Context<Self>) {
        self.last_tick = -1.0;
        self.game_state = GameState::Playing;
        self.request_frame(ctx);
        AudioManager::play(
            self.audio_manager.get("resources/resurrection.mp3"),
            true,
            false,
            0.5,
        );
    }

    fn game_over(&self, kind: GameOverKind) {
        let context = self.context.as_ref().unwrap();
        context.save();
        context.set_global_composite_operation("copy").unwrap();
        context.set_fill_style(&JsValue::from_str(kind.color()));
        context.fill_rect(0.0, 0.0, 601.0, 1000.0);
        context.restore();
        context.set_font("48px bold");
        context.set_text_align("center");
        context.set_fill_style(&JsValue::from_str("white"));
        let mut top = 500.0;
        for text in kind.text(self) {
            context.fill_text(&text, 300.0, top).unwrap();
            top += 100.0;
        }
    }

    fn request_frame(&mut self, ctx: &Context<Self>) {
        self._frame = Some({
            let link = ctx.link().clone();
            request_animation_frame(move |time| link.send_message(Msg::Timer(time)))
        })
    }

    fn required_textures(&self) -> impl Iterator<Item = String> {
        [
            "resources/ghost.png".to_string(),
            "resources/witch.png".to_string(),
            "resources/missile.png".to_string(),
            "resources/missile_2.png".to_string(),
            "resources/hearth.png".to_string(),
            "resources/green_hearth.png".to_string(),
        ]
        .into_iter()
    }

    fn required_audio(&self) -> impl Iterator<Item = String> {
        [
            "resources/resurrection.mp3".to_string(),
            "resources/shoot.wav".to_string(),
            "resources/shoot_2.wav".to_string(),
            "resources/shoot_3.wav".to_string(),
        ]
        .into_iter()
    }
}

#[derive(Clone, Copy)]
enum GameOverKind {
    Lost,
    Won,
    LevelFinished,
    Loading,
}

impl GameOverKind {
    fn color(self) -> &'static str {
        match self {
            GameOverKind::Lost => "rgba(255, 100, 100, 255)",
            GameOverKind::Won | GameOverKind::LevelFinished => "rgba(100, 255, 100, 255)",
            GameOverKind::Loading => "rgba(100, 100, 255, 255)",
        }
    }

    fn text(self, app: &App) -> Vec<String> {
        match self {
            GameOverKind::Lost => vec!["Press Enter to try again.".into()],
            GameOverKind::Won => vec!["You have won!".into()],
            GameOverKind::Loading => vec![
                "Press Enter to start.".into(),
                "Use arrows, space and left ctrl.".into(),
            ],
            GameOverKind::LevelFinished => vec![
                format!("Level {} finished!", app.current_level),
                format!("Press Enter to start next"),
            ],
        }
    }
}
