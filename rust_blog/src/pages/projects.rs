use yew::prelude::*;
use crate::components::{
     layout::Layout,
     header::Header,
};



#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div id="home">
        <Layout >
        <Header />
        <h1>{"My Projects"}</h1>
        </Layout>
        </div>
    }
}
