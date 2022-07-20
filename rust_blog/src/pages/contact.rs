use yew::prelude::*;
use crate::components::{
     layout::Layout,
     header::Header,
};



#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <>
        <Layout >
        <Header />
        <h1>{"Contact me"}</h1>
        // contact masai on this beautiful page
        </Layout>
        </>
    }
}
