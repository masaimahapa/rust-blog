use yew::prelude::*;
use crate::components::{
     layout::Layout,
     header::Header,
};



#[function_component(FourOhFour)]
pub fn four_oh_four() -> Html {
    html! {
        <div id="home">
        <Layout >
        <Header />
        <h1>{"page not found"}</h1>
        </Layout>
        </div>
    }
}
