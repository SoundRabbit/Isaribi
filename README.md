# Isaribi

You can use unique class name by the component. Class names in the component will be mapped to the format: __\[RandomStr\]_\[class name\]. This means that you can write unique style-sheet by the component.

Isaribi can be used not only in Yew, but also in Kagura and so on.

## use in Yew

```rust
extern crate isaribi;

use isaribi::{
    style,
    styled::{Style, Styled},
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {}

enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        Self::styled(html! {
            <div class=Self::class("base")>
                <h1>{"Helllo from Isaribi"}</h1>
            </div>
        })
    }
}

impl Styled for Model {
    fn style() -> Style {
        style! {
            ".base" {
                "position": "fixed";
                "top": "0";
                "left": "0";
                "width": "100vw";
                "height": "100vh";
                "color": "#ffffff";
            }

            ".base > h1" {
                "width": "max-content";
                "margin-left": "auto";
                "margin-right": "auto";
            }

            @media "(orientation: landscape)" {
                ".base" {
                    "background-color": "#0366d6";
                }
            }

            @media "(orientation: portrait)" {
                ".base" {
                    "background-color": "#d73a49";
                }
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
```
