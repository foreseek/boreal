use yew::prelude::*;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum BorealStatusEnum {
    OK,
    KO,
    WARN,
}
impl BorealStatusEnum {
    fn as_str(&self) -> &'static str {
        match self {
            BorealStatusEnum::OK => "bg-color-green",
            BorealStatusEnum::KO => "bg-color-red",
            BorealStatusEnum::WARN => "bg-color-yellow",
        }
    }
}

fn _create_default_status() -> BorealStatusEnum {
    BorealStatusEnum::OK
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    #[prop_or_else(_create_default_status)]
    pub status: BorealStatusEnum,
}

pub struct BorealStatus {
    status_class: &'static str,
}

impl Component for BorealStatus {
    type Message = ();
    type Properties = LinkProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            status_class: ctx.props().status.as_str(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={ classes!("boreal-status", self.status_class) }>{ "" }</div>
        }
    }
}
