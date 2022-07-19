use yew::prelude::*;

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