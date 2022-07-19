use yew::prelude::*;

#[function_component(LatestPosts)]
pub fn lastest_posts() -> Html {
    html! {
        <div>
        <h2>{ "Lastest Blog Posts"}</h2>
        <div class="post-preview">
        <a href="#">{"30 days of Rust - Day Twenty Three - Macros"}</a>
        <span >{"July 13, 2022"}</span>
        </div>

        <div class="post-preview">
        <a href="#">{"30 days of Rust - Day Twenty Two - Multi Threaded Web Server"}</a>
        <span >{"July 12, 2022"}</span>
        </div>

        <div class="post-preview">
        <a href="#">{"30 days of Rust - Day Twenty One - Single Thread Web Server"}</a>
        <span >{"July 11, 2022"}</span>
        </div>
        </div>
    }
}