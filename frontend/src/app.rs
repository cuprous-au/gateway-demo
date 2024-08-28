use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use gloo_timers::callback::Timeout;
use js_sys::Math;
use yew::{platform::spawn_local, prelude::*};

pub enum TamperState {
    Open,
    Closed,
    Unknown,
}

pub enum Message {
    ResetEventSource,
    TamperChangedRemotely(TamperState),
}

pub struct App {
    _es: EventSource,
    tamper_state: TamperState,
}

impl Component for App {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let es = new_event_source(ctx);

        Self {
            _es: es,
            tamper_state: TamperState::Unknown,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ResetEventSource => {
                self._es = new_event_source(ctx);
                false
            }
            Message::TamperChangedRemotely(tamper_state) => {
                self.tamper_state = tamper_state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let tamper_state = match self.tamper_state {
            TamperState::Open => "Open",
            TamperState::Closed => "Closed",
            TamperState::Unknown => "Unknown",
        };

        html! {
            <main>
                <h1>{ format!("Tamper state: {tamper_state}") }</h1>
            </main>
        }
    }
}

fn new_event_source(ctx: &Context<App>) -> EventSource {
    let mut es = EventSource::new("/api/v1/events").unwrap();
    let mut events = es.subscribe("message").unwrap();

    spawn_local({
        let app = ctx.link().clone();

        async move {
            const TAMPER_CLOSED_MESSAGE_DATA: Option<&str> = Some("Tamper closed");

            while let Some(Ok((_event_type, message))) = events.next().await {
                let tamper_state =
                    if message.data().as_string().as_deref() == TAMPER_CLOSED_MESSAGE_DATA {
                        TamperState::Closed
                    } else {
                        TamperState::Open
                    };
                let message = Message::TamperChangedRemotely(tamper_state);
                app.send_message(message);
            }

            app.send_message(Message::TamperChangedRemotely(TamperState::Unknown));

            let timeout_ms = (Math::random() * 4.0) as u32 * 1000 + 1000;
            Timeout::new(timeout_ms, move || {
                app.send_message(Message::ResetEventSource);
            })
            .forget();
        }
    });

    es
}
