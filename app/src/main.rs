use yew::prelude::*;
use yew_router::prelude::*;

mod router;
mod views;

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<router::Route> render={Switch::render(router::switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
