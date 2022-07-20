use yew::prelude::*;

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