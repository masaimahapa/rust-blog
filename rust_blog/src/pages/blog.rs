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

    let blog_post = use_state(|| Box::new(None));
    let error = use_state(|| Box::new(None));

    {
        let blog_post = blog_post.clone();
        use_effect_with_deps(move |_| {
            let blog_post = blog_post.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // let fetched_post = get_post(&post_id).await;
                let url = format!("http://localhost:8000/api/blog/{}", 1);
                let fetched_post  = Request::get(&url).send().await;
                println!("getting {}", url);
                match fetched_post {
                    Ok(response)=>{
                        let json: Result<Post, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                blog_post.set(Box::new(Some(f)))
                            }
                            Err(e) => error.set(Box::new(Some(e.to_string()))),
                        }
                    }
                Err(e) => error.set(Box::new(Some(e.to_string()))),
                }
            });
            || ()
        }, ());
    }

    
    
    match (*blog_post).as_ref() {
        Some(post) => html! {
            <div id="home">
            <Layout >
            <Header />
            <h1>{"Blog"}</h1>
           <p> { post.title.clone() } </p>
           <p> { post.content.clone() } </p>
            </Layout>
            </div>
        },
        None => html! {
            <div id="home">
            <Layout >
            <Header />
            <h1>{"Empty Blog"}</h1>
         
            </Layout>
            </div>
        }
    }
        
    }
