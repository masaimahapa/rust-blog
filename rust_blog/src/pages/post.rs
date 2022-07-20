use yew::prelude::*;
use crate::components::{
     layout::Layout,
     header::Header,
};

#[derive(Properties,Clone, PartialEq)]
pub struct PostProps {
    pub id: String
}


#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let id = props.id.clone();
    html! {
        <div id="home">
        <Layout >
        <Header />
        <h1>{"Post id is : "} {id}</h1>
        </Layout>
        </div>
    }
}
