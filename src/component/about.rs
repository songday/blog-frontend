use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub(crate) struct Model {
}

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

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{ "This is ..., view more: " }</p>
                <p><a href="https://zxzheaven.songday.com">{"文学社"}</a></p>
                <p><a href="https://allu.songday.com">{"卡通驿站"}</a></p>
            </div>
        }
    }
}
