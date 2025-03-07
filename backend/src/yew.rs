use std::collections::HashMap;

use axum::http::{HeaderName, HeaderValue};
use frontend::{ServerApp, ServerAppProps};
use serve_yew::{Process, ServeYew, WriteHeaders};
use stylist::manager::StyleManager;

serve_yew::index!(INDEX);
serve_yew::identity!(Files);
serve_yew::brotli_code!(BrotliTrunkPacked);
// if you have any compressable assets (like svg files), comment out the line below
// serve_yew::brotli_assets!(BrotliAssets);

// swap NoAssets with BrotliAssets if you have any assets
#[cfg(feature = "compression")]
pub fn make_service(_: ()) -> ServeYew<Files, BrotliTrunkPacked, serve_yew::NoAssets, G, ()> {
    use std::collections::BTreeMap;

    // compressable assets like svg need to be manually added to the map
    // run `trunk build --release` first and `ls frontend/dist/identity/assets` to get the list of
    // assets. Then add them to the map below (with the br suffix)
    // let mut m = BTreeMap::new();
    // m.insert("logo.svg", "logo-6fe88bf3de22ed271405d7597167aa85.svg.br");

    ServeYew::new(G, (), Default::default(), Default::default(), INDEX)
}

#[cfg(not(feature = "compression"))]
pub fn make_service(_: ()) -> ServeYew<Files, G, ()> {
    ServeYew::new(G, (), Default::default())
}

// Your "middleware"
#[derive(Clone)]
pub struct G;

// Your cookie collection type
#[derive(Clone)]
pub struct Cookies;

// You can modify response headers here
impl WriteHeaders for Cookies {
    fn write_headers(&self, _headers: &mut axum::http::header::HeaderMap) {}
}

impl Process for G {
    type State = ();

    type Cookies = Cookies;


    // You can extract cookies here
    async fn get_cookies(
        &self,
        _request: axum::extract::Request,
        _app_state: &Self::State,
    ) -> Self::Cookies { Cookies }

    // render your response here
    fn render(
        &self,
        data: std::borrow::Cow<'static, [u8]>,
        _path: String,
        _queries: HashMap<String, String>,
        _app_state: &Self::State,
        _headers: HashMap<HeaderName, HeaderValue>,
        _: Self::Cookies,
    ) -> impl std::future::Future<Output = (String, Self::Cookies)> + Send {
        let html = unsafe { std::str::from_utf8_unchecked(&data) }.to_string();

        async move {
            let (writer, reader) = stylist::manager::render_static();

            let body_s = yew::ServerRenderer::<ServerApp>::with_props(move || {
                let manager = StyleManager::builder()
                    .writer(writer)
                    .build()
                    .expect("failed to create style manager.");
                ServerAppProps { manager }
            })
            .render()
            .await;
            let data = reader.read_style_data();

            let mut style_s = String::new();
            data.write_static_markup(&mut style_s)
                .expect("failed to read styles from style manager");

            let head_split = html.split("</head>").collect::<Vec<_>>();
            let before_head = head_split[0];

            let after_head = head_split[1];
            let body_split = after_head.split("</body>").collect::<Vec<_>>();
            let before_body = body_split[0];
            let after_body = body_split[1];

            (
                format!("{before_head}{style_s}</head>{before_body}{body_s}</body>{after_body}"),
                Cookies,
            )
        }
    }
}
