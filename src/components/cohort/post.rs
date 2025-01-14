use stylist::{yew::styled_component, Style};
use yew::prelude::*;



#[derive(Properties, PartialEq)]
pub struct Props {
    pub Title: String,
    pub Cont: String,
}


const STYLE_FILE: &str = include_str!("cohort.css");
#[styled_component(Content)]
pub fn content(props: &Props) -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html! {
        <>
        <div class = {"Content"}>
            <div class = {"Content"}>
                <h1>{&props.Title}</h1>
                <p>{&props.Cont}</p>
            </div>
        </div>
        </>
    }
}