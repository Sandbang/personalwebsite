use stylist::{yew::styled_component, Style};
use yew::{html, Html};
use crate::components::cohort::social_media::SocialMedia;




const STYLE_FILE: &str = include_str!("../main.css");
#[styled_component(Home)]
pub fn home() -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html!{
        <>
            <body  class = {stylesheet.unwrap()}>
                <div class={"Content"}>
                <h1 >{"about me"}</h1>
                <p>{"Electrical Engineering and Pure Math at UC Davis; interested in: HDL, low level programming and theoretical condensed matter research. I also like to read"} </p>
                <h2 ><bdi style={"color: red"}><a href={"https://www.youtube.com/watch?v=zoo9Vu1a9bU"} target={"_blank"}>{" 'You Can Just Do Things'"}</a></bdi></h2>
                
                <bdi ></bdi>
                <h1 >{"languages"}</h1>
                <p><bdi>{"Interested in "}</bdi><bdi style={"color: red"}>{"SystemVerilog"}</bdi> <bdi>{", "}</bdi> <bdi style = {"color: #d05a27"}>{"Rust"}</bdi> <bdi>{", "}</bdi> <bdi style = {"color: #228587"}>{"Matlab"}</bdi> <bdi>{", "}</bdi> <bdi style = {"color: 	#FFA500"}>{"Python"}</bdi> <bdi>{" and "} </bdi> <bdi style = {"color: 	#00FF00"}>{"C"}</bdi> <bdi></bdi></p>
                <h1><a href={"https://www.betaredaction.com/"}>{"βredaction"}</a></h1><p>{"A blog with writings and my projects. // WORK IN PROGRESS"}</p>
                <h1><a href={"https://github.com/Sandbang/3_Body_Problem_Sim"}>{"Three Body Problem Sim"}</a></h1><p>{"A simulation of the three body problem written in C, compiled into Web Asssembly. // WORK IN PROGRESS"}</p>
                <h1>{"inquiry"}</h1>
                <p>{"micserokurov@ucdavis.edu"}</p>
                
                </div>

                <div class={"SocialMedia"}><SocialMedia /></div>
                <div class={"Built"}> <p> <bdi>{"built entirely in"}</bdi> <bdi style = {"color: #d05a27"}>{" Rust "}</bdi> {"using"} {" "} <a href={"https://docs.rs/yew/latest/yew/"} target={"_blank"} >{"yew.rs"}</a></p></div>
                <div class = {"Content"}>
                
                <p>{""}</p>
                <a/>
            </div>
            </body>
            
        </>
    }
}