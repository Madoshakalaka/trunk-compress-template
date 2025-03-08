use app::Appapp;
use bounce::BounceRoot;
// use http::Uri;
use stylist::{manager::StyleManager, yew::ManagerProvider};
use yew::{function_component, html, Html, Properties};
pub mod app;

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    html! {
        <BounceRoot>
            <ManagerProvider manager={props.manager.clone()}>
                <Appapp />
            </ManagerProvider>
        </BounceRoot>
    }
}

#[derive(Properties, PartialEq)]
pub struct ServerAppProps {
    pub manager: StyleManager,
}
