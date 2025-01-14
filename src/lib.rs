use components::cohort::social_media::SocialMedia;
mod components;
mod pages;
use pages::home::Home;
use yew_router::prelude::*;


use stylist::yew::styled_component;
use stylist::yew::use_style;
use stylist::Style;
use yew::Html;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("main.css");
#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html!{
        <>
            <Home />
        </>
    }
}   