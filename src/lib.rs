extern crate stdweb;

extern crate failure;

#[macro_use]
extern crate yew;

#[macro_use]
extern crate serde_derive;

use failure::Error;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask};

use yew::format::{Json, Nothing};

use yew::services::fetch::{Request, Response};

mod card;

#[derive(Deserialize, Debug)]
pub struct Person {
    id: u32,
    login: String,
    avatar_url: String,
    html_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Results {
    items: Vec<Person>,
}

pub struct Model {
    fs: Option<FetchTask>,
    people: Vec<Person>,
    query: String,
    input_text_is_disabled: bool,
}

pub enum Msg {
    FetchData,
    SuccessFetchData(Vec<Person>),
    InputSearchQuery(String),
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<ConsoleService> + AsMut<FetchService> + 'static,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            fs: None,
            people: vec![],
            query: "".to_string(),
            input_text_is_disabled: true,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                let handler =
                    env.send_back(move |response: Response<Json<Result<Results, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        Msg::SuccessFetchData(data.ok().unwrap().items)
                    });
                let fetch: &mut FetchService = env.as_mut();
                let request = Request::get(&format!(
                    "https://api.github.com/search/users?q={}",
                    self.query
                )).body(Nothing)
                    .expect("Failed to build request.");

                self.fs = Some(fetch.fetch(request, handler));
            }
            Msg::SuccessFetchData(data) => {
                self.people = data;
            }
            Msg::InputSearchQuery(query) => {
                self.input_text_is_disabled = query.is_empty();
                self.query = query;
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<ConsoleService> + 'static + AsMut<FetchService>,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <p>{ "GitHub Account Searcher" }</p>
                <p>
                    <input type="text",
                           oninput=|e: InputData| Msg::InputSearchQuery(e.value), />
                    <button onclick=|_| Msg::FetchData,
                            disabled=self.input_text_is_disabled,>{ "search" }</button>
                </p>
                <ul>{ for self.people.iter().map(|person| self.person_view(person)) }</ul>
            </div>
        }
    }
}

impl Model {
    fn person_view<CTX>(&self, person: &Person) -> Html<CTX, Model>
    where
        CTX: AsMut<FetchService> + 'static + AsMut<ConsoleService>,
    {
        html! {
            <li>
                <card::Card: name=person.login.clone(),
                             image_url=person.avatar_url.clone(),
                             link_url=person.html_url.clone(), />
            </li>
        }
    }
}
