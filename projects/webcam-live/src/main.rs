use serde_json::json;
use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlVideoElement;
use webcam_live::VideoStream;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(|ctx| {
        view! { ctx,
            div(class="container p-2") {
                Video()
            }
        }
    });
}

#[component]
fn Video<G: Html>(ctx: Scope) -> View<G> {
    let video_ref = create_node_ref(ctx);
    

    on_mount(ctx, || {
        let el = video_ref.get::<DomNode>().unchecked_into::<HtmlVideoElement>();
        let video_stream = VideoStream::new(el);

        spawn_local(async move {
            video_stream.set_video_src(&json!({
                "audio": false,
                "video": {
                    "facingMode": "user",
                    "width": 640,
                    "height": 480
                }
            })).await;
        });

    });

    view! { ctx,
        div {
            video(
                ref=video_ref,
                class="border border-gray-400 rounded-lg",
                autoplay=true,
                width=640,
                height=480,
                // src="https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
            )
        }
    }
}