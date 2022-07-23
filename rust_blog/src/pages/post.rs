use yew::prelude::*;
use crate::components::{
     layout::Layout,
     header::Header,
};
use reqwasm::http::Request;
use serde::Deserialize;

#[derive(Properties,Clone, PartialEq)]
pub struct PostProps {
    pub id: String
}


#[derive(Deserialize, Clone)]
struct Post {
    title: String,
    content: String
}

#[function_component(BlogPost)]
pub fn post(props: &PostProps) -> Html {
    let id = props.id.clone();
    let blog_post = use_state(|| Box::new(None));
    let error = use_state(|| Box::new(None));

    {
        let blog_post = blog_post.clone();
        use_effect_with_deps(move |_| {
            let blog_post = blog_post.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // let fetched_post = get_post(&post_id).await;
                let url = format!("http://localhost:8000/api/blog/{}", &id);
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
            <h1> { post.title.clone() } </h1>

           <p> { post.content.clone() } </p>
            </Layout>
            </div>
        },
        None => html! {
            <div id="home">
            <Layout >
            <Header />
            <h3>{"Blog Post of ID :"}{&props.id} {" not Found"}</h3>
         
            </Layout>
            </div>
        }
    }
}
