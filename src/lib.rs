use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yolk::ast::YololNode;
use yolk::{optimize, parse, transpile};

pub struct Model {
    output: String,
}

pub enum Msg {
    Input(String),
}

impl Model {
    fn new() -> Self {
        Model {
            output: String::new(),
        }
    }

    fn transpile(&mut self, input: String) {
        let yolk = match parse(&input) {
            Ok(yolk) => yolk,
            Err(err) => {
                self.output = err.to_string();
                return;
            }
        };
        let (yolol, context) = match transpile(&yolk) {
            Ok((yolol, context)) => (yolol, context),
            Err(err) => {
                self.output = err.to_string();
                return;
            }
        };
        let optimized = optimize(&yolol, &context);
        self.output = YololNode::format_as_program(&optimized);
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model::new()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(s) => {
                self.transpile(s);
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="yolk-web">
                <div class="box">
                    <textarea
                        class=("text",)
                        cols="1000" rows="10"
                        placeholder="Type Yolk code here ..."
                        oninput=|e| Msg::Input(e.value)
                    />
                </div>
                <div class="box">
                    <textarea
                        class=("text",)
                        cols="1000" rows="10"
                        readonly="readonly"
                        placeholder="YOLOL will appear here"
                        value={self.output.to_string()}
                    />
                </div>
            </div>
        }
    }
}
