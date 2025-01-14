use stylist::{yew::styled_component, Style};
use yew::{html, Html};
use crate::components::cohort::{navigation_bar::Navbar, post::Content};


const STYLE_FILE: &str = include_str!("../../main.css");
#[styled_component(Books)]
pub fn books() -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html!{
        <>
            <body  class = {stylesheet.unwrap()}>
                <Navbar />
                <div style = {"text-al"}>
                    <Content Title="Coming Soon" Cont = "" />    
                </div>
            </body>
            
        </>
    }
}