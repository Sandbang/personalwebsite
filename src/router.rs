use yew::{html, Html};
use yew_router::prelude::*;
use crate::components::pages::books::Books;
use crate::components::pages::home::Home;
use crate::components::pages::projects::Projects;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/books")]
    Books,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Projects => html! {<Projects />},
        Route::Books => html! {<Books />},
    }
}
