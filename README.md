# Isaribi

```rust
use kagura::prelude::*;
use isaribi::styled::{Styled, Style};
use isaribi::style;

struct Props{}

enum Msg {}

enum On {}

struct HelloComponent{}

impl Constructor for HelloComponent {
    fn constructor(_: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {}
    }
}

impl Component for HelloComponent {
    type Props = Props;
    type Msg = Msg;
    type Sub = Sub;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, _: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, children: Vec<Html>) -> Html {
        self.styled(children)
    }
}

impl Styled for HelloComponent {
    fn render(&self, _: Vec<Html>) -> Html {
        Html::h1(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![Html::text("Hello Isaribi")],
        )
    }

    fn style() -> Style {
        style! {
            "base" {
                "color": "red";
            }
        }
    }
}
```
