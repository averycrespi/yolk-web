#![recursion_limit = "256"]

use yew::format::Json;
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use yolk::{format_as_program, optimize, parse, transpile};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

const KEY: &str = "yolk-web";

pub struct Model {
    storage: StorageService,
    input: String,
    output: String,
}

pub enum Message {
    Input(String),
}

impl Model {
    fn new() -> Self {
        let mut model = Model {
            storage: StorageService::new(Area::Local),
            input: String::new(),
            output: String::new(),
        };
        model.load();
        model.transpile();
        model
    }

    fn save(&mut self) {
        self.storage.store(KEY, Json(&self.input));
    }

    fn load(&mut self) {
        if let Json(Ok(input)) = self.storage.restore(KEY) {
            self.input = input;
        }
    }

    fn transpile(&mut self) {
        self.output = match parse(&self.input) {
            Ok(yolk) => match transpile(&yolk) {
                Ok(yolol) => format_as_program(&optimize(yolol)),
                Err(err) => err.to_string(),
            },
            Err(err) => err.to_string(),
        };
    }
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model::new()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Input(s) => {
                self.input = s;
                self.save();
                self.transpile();
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let (_, yolk_version) = built_info::DEPENDENCIES
            .iter()
            .find(|(name, _)| name == &"yolk")
            .expect("cannot read Yolk version");
        html! {
            <div class="yolk-web">
                <div class="box">
                    <textarea
                        class="textbox"
                        cols="1000" rows="10"
                        placeholder="Type Yolk code here ..."
                        oninput=|e| Message::Input(e.value)
                        value={self.input.to_string()} />
                </div>
                <div class="box">
                    <textarea
                        class="textbox"
                        cols="1000" rows="10"
                        readonly="readonly"
                        placeholder="Yolol will appear here"
                        value={self.output.to_string()} />
                </div>
                <div class="box">
                    <p class="note">
                        <a class="note" href="https://github.com/averycrespi/yolk-web">{"Yolk Web"}</a>
                        {format!(" v{}", built_info::PKG_VERSION)}
                    </p>
                    <p class="note">
                        {"Powered by "}
                        <a class="note" href="https://github.com/averycrespi/yolk">{"Yolk"}</a>
                        {format!(" v{}", yolk_version)}
                    </p>
                </div>
            </div>
        }
    }
}
