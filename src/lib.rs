use yew::prelude::*;
use yew_router::prelude::*;


pub mod components;
pub mod pages;
pub mod routes;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch <routes::Route> render={Switch::render(routes::switch)} />
        </BrowserRouter>
    }
}
