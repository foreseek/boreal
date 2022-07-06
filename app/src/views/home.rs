use yew::prelude::*;

use crate::components::boreal_box::BorealBox;
use crate::components::boreal_status::*;

pub struct Home {}
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div class={ classes!("position-center-vh", "scale-screen-h") }>
            <BorealBox>
              <div class={ classes!("flex-space-between", "vertical-center", "pb-10") }>
                <h2>{ "Welcome to boreal" }</h2>
                <BorealStatus/>
              </div>
              <p>{ "Your Boreal installation is currently up and running!" }</p>
              <p>{ "You can access your admin pannel via /admin" }</p>
            </BorealBox>
          </div>
        }
    }
}
