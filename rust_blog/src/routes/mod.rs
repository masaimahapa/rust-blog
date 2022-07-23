use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{home::Home, 
    four_oh_four::FourOhFour,
     post::BlogPost,
    projects::Projects,
    blog::Blog,
contact::Contact};

#[derive( Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/blog")]
    Blog,
    #[at("/contact")]
    Contact,
    #[at("/post/:id")]
    Post {id: String},
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) ->  Html{
    match routes {
        Route::Home => html! {<Home />},
        Route::Blog => html! {<Blog />},
        Route::Projects => html! {
            <Projects />
        },
        Route::Contact => html! {<Contact />},
        Route::Post {id} => html! {
            <BlogPost id={id.clone()}/>
        },
        Route::NotFound => html! {<FourOhFour />}
    }
}