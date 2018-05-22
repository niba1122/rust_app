use yew::prelude::*;

pub struct Card {
    name: String,
    image_url: String,
    link_url: String,
}

pub struct Msg {}

#[derive(Default, Clone, PartialEq)]
pub struct CardProps {
    pub name: String,
    pub image_url: String,
    pub link_url: String,
}

impl<CTX> Component<CTX> for Card
where
    CTX: 'static,
{
    type Message = Msg;
    type Properties = CardProps;
    fn create(props: Self::Properties, _env: &mut Env<'_, CTX, Self>) -> Self {
        Card {
            name: props.name,
            image_url: props.image_url,
            link_url: props.link_url,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.name = props.name;
        self.image_url = props.image_url;
        self.link_url = props.link_url;
        true
    }
}

impl<CTX> Renderable<CTX, Card> for Card
where
    CTX: 'static,
{
    fn view(&self) -> Html<CTX, Card> {
        html! {
            <div>
                <a href=self.link_url.clone(), target="_blank", >
                    <img src=self.image_url.clone(), />
                    { self.name.clone() }
                </a>
            </div>
        }
    }
}
