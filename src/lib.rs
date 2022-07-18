use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div id="app">
        <NavBar />
        <Header />
        <Bio />
        <LatestPosts/>
        

        </div>
    }
}


#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <navbar>
            <ul> 
            <li> <a href="#">{"Home"}</a></li>
            <li> <a href="#"> {"Projects"}</a></li>
            <li> <a href="#">{"Blog"}</a></li>
            <li> <a href="#"> {"Contact"}</a></li> 
            </ul>
        </navbar>
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div>
        <div class="profile-pic">
        <img height="200"  src="https://www.masaimahapa.co.za/_next/image?url=%2Fimages%2Fcoding-sai.png&w=384&q=75" />
        </div>
        <h1>{ "Masai Mahapa"}</h1>
        </div>
    }
}

#[function_component(Bio)]
pub fn bio() -> Html {
    html! {
        <div id="bio">
    
        <p>{"I am a self taught software developer from Johannesburg, South Africa."}</p>
        <p>{"Founder of Travcar🚗.
        Currently working at Telkom SA as a frontend developer."}</p>
        <p> {" I blog about;"}
        <ul>
            <li>{"Software Engineering"}</li>
            <li>{"Self Development"}</li>
            <li>{"Startups"}</li>
        </ul>
        </p>
        </div>
    }
}

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
        <span >{"July 15, 2022"}</span>
        </div>

        <div class="post-preview">
        <a href="#">{"30 days of Rust - Day Twenty One - Single Thread Web Server"}</a>
        <span >{"July 14, 2022"}</span>
        </div>
        </div>
    }
}