use stylist::{yew::styled_component, Style};
use yew::{html, Html};
use crate::components::cohort::navigation_bar::Navbar;
use crate::components::cohort::social_media::SocialMedia;




const STYLE_FILE: &str = include_str!("../../main.css");
#[styled_component(Home)]
pub fn home() -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html!{
        <>
            <body  class = {stylesheet.unwrap()}>
                <Navbar />
                <div class={"Content2"}>
                <h1 >{"about me"}</h1>
                <p>{"EE at UC Davis, interested in HDL, DSA and backend. I also like to read"}</p>
                <h1 >{"languages"}</h1>
                <p><bdi>{"Interested in "}</bdi><bdi style={"color: red"}>{"SystemVerilog"}</bdi> <bdi>{", "}</bdi> <bdi style = {"color: #d05a27"}>{"Rust"}</bdi> <bdi>{", "}</bdi> <bdi style = {"color: #228587"}>{"Matlab"}</bdi> <bdi>{" and "}</bdi> <bdi style = {"color: 	#FFA500"}>{"Java"}</bdi></p>
                <h1>{"inquiry"}</h1>
                <p>{"micserokurov@ucdavis.edu"}</p>

                </div>

                <div class={"SocialMedia"}><SocialMedia /></div>
                <div class={"Design"}> <p> <bdi>{"built entirely in"}</bdi> <bdi style = {"color: #d05a27"}>{" Rust "}</bdi> {"using"} {" "} <a href={"https://docs.rs/yew/latest/yew/"} target={"_blank"} >{"yew.rs"}</a>{", "} <a href={"https://actix.rs/"} target={"_blank"}>{"actix-web"}</a> {", and "} <a href={"https://docs.rs/sqlx/latest/sqlx/"} target={"_blank"}>{"sqlx"}</a></p></div>
            </body>
            
        </>
    }
}