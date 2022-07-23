use yew::prelude::*;
use crate::components::{
     layout::Layout,
     header::Header,
};

use serde::Deserialize;


use reqwasm::http::Request;

#[derive(Deserialize, Clone)]
struct Post {
    title: String,
    content: String
}

// async fn get_post(id: &String) -> Post {
//     let url = format!("/api/blog/{}", id);
//     return Request::get(&url).send().await.unwrap().json().await.unwrap();
// }



#[function_component(Blog)]
pub fn blog() -> Html {

     html! {
            <div id="home">
            <Layout >
            <Header />
            <h1>{"Empty Blog"}</h1>
         
            </Layout>
            </div>
        }
    }
        
    