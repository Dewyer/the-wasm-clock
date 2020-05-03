use yew::prelude::*;
use yew::services::{ConsoleService,IntervalService};
use yew::{ComponentLink};
use wasm_bindgen::__rt::core::time::Duration;

pub struct App
{
    link:ComponentLink<Self>,
    counter:isize,
    console:ConsoleService,
    //interval:IntervalService,
    //job:Box<dyn Task>
}

pub enum Msg {
    HelloWorld,
    Tick
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut is = IntervalService::new();
        let _handle = is.spawn(Duration::from_secs(1),link.callback(|_| Msg::Tick));

        App {
            counter : 0,
            console:ConsoleService::new(),
            link:link.clone(),
            //interval:is,
            //job:Box::new(handle)
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg
        {
            Msg::HelloWorld => {
                self.console.log("Hello from wasm!");
                false
            },
            Msg::Tick =>
            {
                self.counter += 1;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p style={"color:red"}>{ "Hello world!" }</p>
                <p>{self.counter}</p>
                <button onclick=self.link.callback(|_| Msg::HelloWorld )>{"Lol"}</button>
            </div>
        }
    }
}
