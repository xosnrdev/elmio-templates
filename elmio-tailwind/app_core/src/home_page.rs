use elmio_core::{
    browser::{
        dom_id::DomId,
        effect::Effect,
        subscription::{event_listener::on_click, Subscription},
    },
    page::{self, Page, PageMarkup},
};
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub count: isize,
}

pub struct HomePage {
    pub current_url: Url,
}

impl Page<Model, Msg, Markup> for HomePage {
    fn id(&self) -> &'static dyn DomId {
        &Id::ElmioTailwind
    }

    fn init(&self) -> Result<(Model, Effect<Msg>), String> {
        let model = Model { count: 0 };

        Ok((model, Effect::None))
    }

    fn subscriptions(&self, _model: &Model) -> Subscription<Msg> {
        Subscription::Batch(vec![
            on_click(Id::Increment, Msg::Increment),
            on_click(Id::Decrement, Msg::Decrement),
        ])
    }

    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effect<Msg>, String> {
        match msg {
            Msg::Increment => {
                model.count += 1;
                Ok(Effect::None)
            }

            Msg::Decrement => {
                model.count -= 1;
                Ok(Effect::None)
            }
        }
    }

    fn view(&self, model: &Model) -> PageMarkup<Markup> {
        PageMarkup {
            head: view_head(),
            body: view_body(model),
        }
    }

    fn render(&self, markup: Markup) -> String {
        markup.into_string()
    }

    fn render_page(&self, markup: PageMarkup<Markup>) -> String {
        page::render_page_maud(markup)
    }
}

#[derive(strum_macros::Display, elmio_macro::DomId)]
#[strum(serialize_all = "kebab-case")]
enum Id {
    ElmioTailwind,
    Increment,
    Decrement,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Msg {
    Increment,
    Decrement,
}

fn view_head() -> Markup {
    html! {
        title { "Home Page" }
        meta name="viewport" content="width=device-width, initial-scale=1";
        link rel="stylesheet" href="/app.css";
        script defer type="module" src="/home_page.js" {}
    }
}

fn view_body(model: &Model) -> Markup {
    html! {
        div id=(Id::ElmioTailwind) {
            div class="flex p-4" {
                button id=(Id::Decrement) class="w-28 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                    "Decrement"
                }
                div class="mx-4 w-28" {
                    input value=(model.count) class="text-center shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" type="text" readonly;
                }
                button id=(Id::Increment) class="w-28 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                    "Increment"
                }
            }
        }
    }
}
