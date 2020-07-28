use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yew_router::{prelude::*};

use crate::util::Error;
use crate::app::AppRoute;
use crate::model::User;
use crate::component::error::ShowErrors;

pub(crate) struct Model {
    user: User,
    error: Option<Error>,
    link: ComponentLink<Self>,
}

pub(crate) enum Msg {
    UpdateEmail(String),
    UpdatePassword(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            user: User::default(),
            error: None,
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <RouterAnchor<AppRoute> route=AppRoute::UserRegister>
                    { "Need an account?" }
                </RouterAnchor<AppRoute>>
                <h1>{"Sign In"}</h1>
                <ShowErrors error=&self.error />
                <form>
                    <fieldset>
                        <fieldset class="form-group">
                            <input
                                class="form-control form-control-lg"
                                type="email"
                                placeholder="Email"
                                value=&self.user.email
                                oninput=self.link.callback(|e: InputData| Msg::UpdateEmail(e.value))
                                />
                        </fieldset>
                        <fieldset class="form-group">
                            <input
                                class="form-control form-control-lg"
                                type="password"
                                placeholder="Password"
                                value=&self.user.password
                                oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value))
                                />
                        </fieldset>
                        <button
                            class="btn btn-lg btn-primary pull-xs-right"
                            type="submit"
                            disabled=false>
                            { "Sign in" }
                        </button>
                    </fieldset>
                </form>
            </div>
        }
    }
}
