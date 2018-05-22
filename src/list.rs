#[macro_use]

use yew::prelude::*;


pub struct SubComponent {
    title: String
}

pub enum Msg {

}

#[derive(Default, Clone, PartialEq)]
pub struct SubComponentProperty {
    pub title: String
}

impl<CTX> Component<CTX> for SubComponent
where
    CTX: 'static
{
    type Message = Msg;
    type Properties = SubComponentProperty;
    fn create(properties: Self::Properties, _env: &mut Env<'_, CTX, Self>) -> Self {
        SubComponent {
            title: properties.title
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }
}

impl<CTX> Renderable<CTX, SubComponent> for SubComponent
where
    CTX: 'static
 {
    fn view(&self) -> Html<CTX, SubComponent> {
        html! {
            <div>
                {format!("{}", self.title)}
                <ul>
                    <li>{ "hogehoge" }</li>
                    <li>{ "fugafuga" }</li>
                </ul>
            </div>
        }
    }
}