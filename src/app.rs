use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::{prelude::*, Switch};
use yew_router::switch::{AllowMissing, Permissive};

use crate::component::{index, about};

#[derive(Switch, Clone)]
pub(crate) enum AppRoute {
    #[to = "/#/about"]
    About,
    #[to = "/#/user/login"]
    UserLogin,
    #[to = "/#/user/register"]
    UserRegister,
    #[to = "/#/post/{id}"]
    ViewPost(u64),
    #[to = "/#/posts/list"]
    ListPosts,
    #[to = "/#/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/"]
    Home,
}

pub(crate) struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                <nav class="menu",>
                    <RouterButton<AppRoute> route=AppRoute::Home> {"Index"} </RouterButton<AppRoute>>
                    // <RouterAnchor<AppRoute> route=AppRoute::ListPosts> {"Blog"} </RouterAnchor<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::About> {"About"} </RouterButton<AppRoute>>
                    // <RouterButton<AppRoute> route=AppRoute::UserLogin> {"Login"} </RouterButton<AppRoute>>
                </nav>
                <div>
                    <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Home => html!{<index::Model/>},
                                AppRoute::About => html!{<about::Model/>},
                                _ => html!{"Page not found 111111111"},
                            }
                        })
                        redirect = Router::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </div>
                <footer>
                    <div class="container">
                        <RouterAnchor<AppRoute> route=AppRoute::Home classes="logo-font">{ "conduit" }</RouterAnchor<AppRoute>>
                        <span class="attribution">
                            { "&copyright; 2020. An interactive learning project from" }
                            <a href="https://thinkster.io"> { "Thinkster" } </a>
                            { ". Code licensed under MIT.aaa" }
                        </span>
                    </div>
                </footer>
            </div>
        }
    }
}