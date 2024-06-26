use chrono::{DateTime, Local};
use futures::StreamExt;
use std::time::Duration;
use yew::platform::time::interval;
use yew::{html, Component, Context, Html};

pub struct ClockComponent {
    current_time: DateTime<Local>,
}

pub enum Msg {
    ClockTicked(DateTime<Local>),
}

impl Component for ClockComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let time_steam = interval(Duration::from_secs(1)).map(|_| Local::now());
        ctx.link().send_stream(time_steam.map(Msg::ClockTicked));

        Self {
            current_time: Local::now(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClockTicked(current_time) => {
                self.current_time = current_time;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="fs-1 text-end fw-bold text-white">
                { format!("{}", self.current_time.format("%d %b %Y")) }
                <br/>
                { format!("{}", self.current_time.format("%H : %M : %S")) }
            </div>
        }
    }
}
