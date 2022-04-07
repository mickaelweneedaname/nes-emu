use gloo_render::AnimationFrame;
use gloo_timers::callback::Timeout;
use nes_emu::{joypad::Button, screen::Color, Nes, Player};
use reqwasm::http::Request;
use std::{cell::RefCell, pin::Pin, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlSpanElement};
use yew::{events::KeyboardEvent, html, html::Scope, Component, Context, Html, NodeRef};

pub enum Msg {
    Render { timestamp: f64 },
    KeyDown { key: KeyboardEvent },
    KeyUp { key: KeyboardEvent },
    Run,
}

pub struct App {
    canvas_ref: NodeRef,
    key_pressed: NodeRef,
    rendering_context: Option<CanvasRenderingContext2d>,
    _animation_frame: Option<AnimationFrame>,
    on_key_down: Option<Closure<dyn Fn(KeyboardEvent)>>,
    on_key_up: Option<Closure<dyn Fn(KeyboardEvent)>>,
    nes: Rc<RefCell<Pin<Box<Nes>>>>,
    scale: u8,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            key_pressed: NodeRef::default(),
            rendering_context: None,
            _animation_frame: None,
            on_key_down: None,
            on_key_up: None,
            nes: Rc::new(RefCell::new(Nes::new())),
            scale: 15,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render { timestamp } => {
                self.render_frame(timestamp);
                self.request_animation_frame(ctx.link().clone());
                false
            }
            Msg::KeyDown { key } => {
                let key_pressed = self.key_pressed.cast::<HtmlSpanElement>().unwrap();
                unsafe {
                    match key.key().as_ref() {
                        "z" => self
                            .nes
                            .as_ref()
                            .borrow_mut()
                            .press(Player::One, Button::UP),
                        "q" => self
                            .nes
                            .as_ref()
                            .borrow_mut()
                            .press(Player::One, Button::LEFT),
                        "s" => self
                            .nes
                            .as_ref()
                            .borrow_mut()
                            .press(Player::One, Button::DOWN),
                        "d" => self
                            .nes
                            .as_ref()
                            .borrow_mut()
                            .press(Player::One, Button::RIGHT),
                        _ => {}
                    }
                }
                let str = format!("pressed [{}]", key.key());
                key_pressed.set_text_content(Some(&str));
                false
            }
            Msg::KeyUp { key } => {
                let key_pressed = self.key_pressed.cast::<HtmlSpanElement>().unwrap();
                unsafe {
                    match key.key().as_ref() {
                        "z" => self
                            .nes
                            .as_ref()
                            .borrow_mut()
                            .release(Player::One, Button::UP),
                        "q" => self
                            .nes
                            .as_ref()
                            .borrow_mut()
                            .release(Player::One, Button::LEFT),
                        "s" => self
                            .nes
                            .as_ref()
                            .borrow_mut()
                            .release(Player::One, Button::DOWN),
                        "d" => self
                            .nes
                            .as_ref()
                            .borrow_mut()
                            .release(Player::One, Button::RIGHT),
                        _ => {}
                    }
                }
                let str = format!("released [{}]", key.key());
                key_pressed.set_text_content(Some(&str));
                false
            }
            Msg::Run => {
                //We need to find another solution for that code....
                for _ in 0..8 {
                    unsafe {
                        self.nes.as_ref().borrow_mut().run();
                    }
                }
                let link = ctx.link().clone();
                let timeout = Timeout::new(1, move || {
                    link.send_message(Msg::Run);
                });
                timeout.forget();

                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "NES Emulator" }</h1>
                <canvas
                    ref={ self.canvas_ref.clone() }
                />
                <h1>{"key pressed : "}<span ref={self.key_pressed.clone()}>{"None"}</span></h1>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

        let rendering_context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.rendering_context = Some(rendering_context);

        if first_render {
            canvas.set_width(32 * self.scale as u32);
            canvas.set_height(32 * self.scale as u32);
            self.listen_keyboard_events(ctx.link().clone());
            self.request_animation_frame(ctx.link().clone());
            self.load_program(ctx.link().clone());
        }
    }
}

impl App {
    fn render_frame(&mut self, _timestamp: f64) {
        let rendering_context = self.rendering_context.as_ref().unwrap();
        unsafe {
            let screen_data = self.nes.as_ref().borrow().get_screen_data();
            for (c, line) in screen_data.iter().enumerate() {
                for (l, pixel_color) in line.iter().enumerate() {
                    let fill_color = JsValue::from_str(&String::from(Color::from(*pixel_color)));
                    rendering_context.set_fill_style(&fill_color);
                    rendering_context.fill_rect(
                        l as f64 * self.scale as f64,
                        c as f64 * self.scale as f64,
                        self.scale as f64,
                        self.scale as f64,
                    );
                }
            }
        }
    }

    fn request_animation_frame(&mut self, link: Scope<Self>) {
        let handle = {
            gloo_render::request_animation_frame(move |time| {
                link.send_message(Msg::Render { timestamp: time });
            })
        };
        self._animation_frame = Some(handle);
    }

    fn listen_keyboard_events(&mut self, link: Scope<Self>) {
        let window = window().unwrap();
        let link_copy = link.clone();
        self.on_key_down = Some(Closure::wrap(Box::new(move |event: KeyboardEvent| {
            link_copy.send_message(Msg::KeyDown { key: event });
        }) as Box<dyn Fn(KeyboardEvent)>));
        self.on_key_up = Some(Closure::wrap(Box::new(move |event: KeyboardEvent| {
            link.send_message(Msg::KeyUp { key: event });
        }) as Box<dyn Fn(KeyboardEvent)>));
        window
            .add_event_listener_with_callback(
                "keydown",
                self.on_key_down.as_ref().unwrap().as_ref().unchecked_ref(),
            )
            .expect("Unable to set callback on window");
        window
            .add_event_listener_with_callback(
                "keyup",
                self.on_key_up.as_ref().unwrap().as_ref().unchecked_ref(),
            )
            .expect("Unable to set callback on window");
    }

    fn load_program(&mut self, link: Scope<Self>) {
        let nes = self.nes.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let program = asm_6502::compile(
                Request::get("./snake.asm")
                    .header("Accept", "text")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap(),
                0x8000,
            );
            unsafe {
                nes.as_ref().borrow_mut().load(&program[..], 0x8000);
            }
            link.send_message(Msg::Run);
        })
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
