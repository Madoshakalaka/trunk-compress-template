use bounce::BounceRoot;
use stylist::{
    manager::StyleManager,
    yew::{styled_component, Global, ManagerProvider},
};
use yew::prelude::*;
use yew_html_ext::html;

#[styled_component]
pub fn Appapp() -> Html {
    html! {
        <Global
            css={css!(
                    body {
                        background-color: black;
                    }
                    )}
        />
        <h1 style="color: white;">{ "Hello, world!" }</h1>
    }
}

#[function_component]
pub fn App() -> Html {
    let style_mgr = (*use_memo((), |_| {
        StyleManager::new().expect("failed to create style manager.")
    }))
    .to_owned();

    dev_reload::use_reload();

    html! {
        <BounceRoot>
            <ManagerProvider manager={style_mgr}>
                <Appapp />
            </ManagerProvider>
        </BounceRoot>
    }
}
