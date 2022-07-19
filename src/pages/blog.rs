use yew::prelude::*;
use crate::components::{
     layout::Layout,
     header::Header,
};



#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <div id="home">
        <Layout >
        <Header />
        <h1>{"Blog"}</h1>
        </Layout>
        </div>
    }
}
