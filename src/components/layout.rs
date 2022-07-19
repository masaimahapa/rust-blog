use yew::prelude::*;
use crate::components::{footer::Footer,
    navbar::NavBar
};


#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <>
        <NavBar />
            {for props.children.iter()}
        <Footer />
        </>
    }
}