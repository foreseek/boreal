use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct BorealBox {}
impl Component for BorealBox {
    type Message = ();
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={ "boreal-box" }>
              { for ctx.props().children.iter() }
            </div>
        }
    }
}
