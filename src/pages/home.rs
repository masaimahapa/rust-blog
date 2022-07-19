use yew::prelude::*;
use crate::components::{
     layout::Layout,
     header::Header,
     bio::Bio,
     latest_posts::LatestPosts
};



#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div id="home">
        <Layout >
        <Header />
        <Bio />
        <LatestPosts/>
        </Layout>

        </div>
    }
}
