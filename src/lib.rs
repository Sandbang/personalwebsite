use components::cohort::navigation_bar::Navbar;
use components::cohort::social_media::SocialMedia;
mod components;
mod router;
use yew_router::prelude::*;
use crate::router::{switch, Route};


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
            <BrowserRouter>
                <Switch<Route>render={switch} />
            </BrowserRouter>
        </>
    }
}   