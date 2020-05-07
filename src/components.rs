use yew::prelude::*;

pub struct App {
    _link: ComponentLink<Self>,
}

impl Component for App {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { <h3 align="center">{"\u{1F980} He-hey, this stuff really works! Yay! \u{2764}\u{fe0f}"}</h3> }
    }
}
