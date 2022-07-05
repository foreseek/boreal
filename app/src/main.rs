use yew::prelude::*;

enum Msg {}

struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <div class={ classes!("position-center-vh", "scale-screen-h") }>
            <div class={ "boreal-box" }>
                <h2>{ "Welcome to boreal" }</h2>
                <p>{ "Your Boreal installation is currently up and running!" }</p>
                <p>{ "You can access your admin pannel via /admin" }</p>
            </div>
          </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
