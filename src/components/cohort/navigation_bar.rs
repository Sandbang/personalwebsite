use yew::prelude::*;

#[function_component(Navbar)]
pub fn navigation_bar() -> Html {
    html! {
        <>
        <nav class ="navbar">
            <ul>
               <li><a href={"http://[::1]:8080/projects"}>{"Projects"}</a></li> 
               <li><a href={"http://[::1]:8080/"}>{"Home"}</a></li> 
               <li><a href={"http://[::1]:8080/books"}>{"Books"}</a></li>    
            </ul>
        </nav>
        </>
    }
}