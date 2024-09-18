use tracing::info;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlVideoElement, MediaStream, MediaStreamConstraints};

pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream {
            el
        }
    }

    pub async  fn set_video_src(&self, video_constraints: &serde_json::Value) {
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        let devices = navigator.media_devices().expect("no `mediaDevices` object exists");

        info!("devices(tracing_wasm): {:?}", devices);

        web_sys::console::log_1(&devices);

        let constraints = MediaStreamConstraints::new();
        constraints.set_video(&serde_wasm_bindgen::to_value(video_constraints).unwrap());
        constraints.set_audio(&false.into());

        let media = JsFuture::from(
            devices.get_user_media_with_constraints(&constraints).unwrap()
        )
        .await
        .unwrap();

        let media_stream = media.unchecked_into::<MediaStream>();

        info!("media_stream(tracing_wasm): {:?}", media_stream);

        self.el.set_src_object(Some(&media_stream));

    }
}