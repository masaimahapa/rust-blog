use yew::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <navbar>
            <ul> 
            <li> <a href="/">{"Home"}</a></li>
            <li> <a href="/projects"> {"Projects"}</a></li>
            <li> <a href="/blog">{"Blog"}</a></li>
            <li> <a href="/contact"> {"Contact"}</a></li> 
            </ul>
        </navbar>
    }
}