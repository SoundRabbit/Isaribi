use regex::Regex;
use std::any;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use wasm_bindgen::{prelude::*, JsCast};

thread_local! {
    static STYLED: RefCell<HashSet<u64>> = RefCell::new(HashSet::new());
    static SHEET: RefCell<Option<web_sys::CssStyleSheet>> = RefCell::new(None);
}

fn hash_of_type<C>() -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(any::type_name::<C>().as_bytes());
    hasher.finish()
}

fn styled_class_prefix<C>() -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(any::type_name::<C>().as_bytes());
    format!("{:X}", hasher.finish())
}

fn styled_class<C>(class_name: &str) -> String {
    format!("_{}__{}", styled_class_prefix::<C>(), class_name)
}

pub trait Styled: Sized {
    fn style() -> Style;
    fn styled<T>(node: T) -> T {
        STYLED.with(|styled| {
            let component_id = hash_of_type::<Self>();
            if styled.borrow().get(&component_id).is_none() {
                let style = Self::style();
                style.write::<Self>();
                styled.borrow_mut().insert(component_id);
            }
        });

        node
    }
    fn class(class_name: &str) -> String {
        styled_class::<Self>(class_name)
    }
}

#[derive(Clone)]
pub struct Style {
    style: Vec<(String, Vec<(String, String)>)>,
    media: Vec<(String, Self)>,
    class_selecter: Regex,
}

impl Style {
    fn class_selecter() -> Regex {
        Regex::new(r"\.([a-zA-Z][a-zA-Z0-9\-_]*)").unwrap()
    }

    pub fn new() -> Self {
        Self {
            style: vec![],
            media: vec![],
            class_selecter: Self::class_selecter(),
        }
    }

    pub fn add(
        &mut self,
        selector: impl Into<String>,
        property: impl Into<String>,
        value: impl Into<String>,
    ) {
        let selector = selector.into();
        let property = property.into();
        let value = value.into();

        if let Some(class_idx) = self.style.iter().position(|s| s.0 == selector) {
            if let Some(property_idx) = self.style[class_idx].1.iter().position(|p| p.0 == property)
            {
                self.style[class_idx].1[property_idx].1 = value;
            } else {
                self.style[class_idx].1.push((property, value));
            }
        } else {
            self.style.push((selector, vec![(property, value)]));
        }
    }

    pub fn add_media(&mut self, query: impl Into<String>, style: Style) {
        let query = query.into();
        self.media.push((query, style));
    }

    pub fn append(&mut self, other: &Self) {
        for (selector, definition_block) in &other.style {
            for (property, value) in definition_block {
                self.add(selector, property, value);
            }
        }

        for (query, style) in &other.media {
            self.add_media(query, style.clone());
        }
    }

    fn rules<C>(&self) -> Vec<String> {
        let mut res = vec![];

        for (selector, definition_block) in &self.style {
            let mut rule = String::new();
            let selector = self
                .class_selecter
                .replace_all(selector, format!("._{}__$1", styled_class_prefix::<C>()));
            rule += &selector;
            rule += "{";
            for (property, value) in definition_block {
                rule += format!("{}:{};", property, value).as_str();
            }
            rule += "}";

            res.push(rule);
        }

        for (query, media_style) in &self.media {
            let mut rule = String::from("@media ");
            rule += query;
            rule += "{";
            for child_rule in &media_style.rules::<C>() {
                rule += child_rule;
            }
            rule += "}";
            res.push(rule);
        }

        res
    }

    fn write<C>(&self) {
        Self::add_style_element();

        for rule in &self.rules::<C>() {
            SHEET.with(|sheet| {
                if let Some(sheet) = sheet.borrow().as_ref() {
                    if let Err(err) = sheet
                        .insert_rule_with_index(rule.as_str(), sheet.css_rules().unwrap().length())
                    {
                        web_sys::console::log_1(&JsValue::from(err));
                    }
                }
            });
        }
    }

    fn add_style_element() {
        SHEET.with(|sheet| {
            if sheet.borrow().is_none() {
                let style_element = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("style")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlStyleElement>()
                    .unwrap();

                let head = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_elements_by_tag_name("head")
                    .item(0)
                    .unwrap();

                let _ = head.append_child(&style_element);

                *sheet.borrow_mut() = Some(
                    style_element
                        .sheet()
                        .unwrap()
                        .dyn_into::<web_sys::CssStyleSheet>()
                        .unwrap(),
                );
            }
        });
    }
}

macro_rules! return_if {
    ($x:ident = $y:expr; $z: expr) => {{
        let $x = $y;
        if $z {
            return $x;
        }
    }};
}

impl std::fmt::Debug for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (selector, definition_block) in &self.style {
            return_if!(x = write!(f, "{} {}\n", selector, "{"); x.is_err());
            for (property, value) in definition_block {
                return_if!(x = write!(f, "    {}: {};\n", property, value); x.is_err());
            }
            return_if!(x = write!(f, "{}\n", "}"); x.is_err());
        }

        for (query, style) in &self.media {
            return_if!(x = write!(f, "@media {} {}\n", query, "{"); x.is_err());

            for a_line in format!("{:?}", style).split("\n") {
                if a_line != "" {
                    return_if!(x = write!(f, "    {}\n", a_line); x.is_err());
                }
            }

            return_if!(x = write!(f, "{}\n", "}"); x.is_err());
        }

        write!(f, "")
    }
}

impl PartialEq for Style {
    fn eq(&self, other: &Self) -> bool {
        self.style.eq(&other.style) && self.media.eq(&other.media)
    }
}

#[macro_export]
macro_rules! style {
    {
        $(
            @import $import:expr;
        )*
        $(
            $selector:literal {$(
                $property:tt : $value:expr
            );*;}
        )*
        $(
            @media $query:tt {$(
                $media_style:tt
            )*}
        )*
    } => {{
        #[allow(unused_mut)]
        let mut style = Style::new();
        $(
            style.append(&($import));
        )*
        $(
            $(
                style.add(format!("{}", $selector), format!("{}", $property), format!("{}", $value));
            )*
        )*
        $(
            style.add_media($query, style!{$($media_style)*});
        )*
        style
    }};
}

#[cfg(test)]
mod tests {
    use super::Style;

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn debug_style() {
        let style = Style {
            style: vec![
                (
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                (
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
            ],
            media: vec![],
            class_selecter: Style::class_selecter(),
        };

        let style_str = concat!(
            "foo {\n",
            "    width: 100px;\n",
            "    height: 100px;\n",
            "}\n",
            "bar {\n",
            "    width: 100px;\n",
            "    height: 100px;\n",
            "}\n",
        );

        assert_eq!(format!("{:?}", style), style_str);
    }

    #[test]
    fn debug_style_with_media() {
        let media_style = Style {
            style: vec![
                (
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                (
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
            ],
            media: vec![],
            class_selecter: Style::class_selecter(),
        };
        let style = Style {
            style: vec![
                (
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                (
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
            ],
            media: vec![(String::from("query"), media_style)],
            class_selecter: Style::class_selecter(),
        };

        let style_str = concat!(
            "foo {\n",
            "    width: 100px;\n",
            "    height: 100px;\n",
            "}\n",
            "bar {\n",
            "    width: 100px;\n",
            "    height: 100px;\n",
            "}\n",
            "@media query {\n",
            "    foo {\n",
            "        width: 100px;\n",
            "        height: 100px;\n",
            "    }\n",
            "    bar {\n",
            "        width: 100px;\n",
            "        height: 100px;\n",
            "    }\n",
            "}\n",
        );

        assert_eq!(format!("{:?}", style), style_str);
    }

    #[test]
    fn gen_style_by_manual() {
        let style_a = Style {
            style: vec![
                (
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                (
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
            ],
            media: vec![],
            class_selecter: Style::class_selecter(),
        };

        let mut style_b = Style::new();
        style_b.add("foo", "width", "100px");
        style_b.add("foo", "height", "100px");
        style_b.add("bar", "width", "100px");
        style_b.add("bar", "height", "100px");

        assert_eq!(style_a, style_b);
    }

    #[test]
    fn gen_style_with_media_by_manual() {
        let media_style_a = Style {
            style: vec![
                (
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                (
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
            ],
            media: vec![],
            class_selecter: Style::class_selecter(),
        };
        let style_a = Style {
            style: vec![
                (
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                (
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
            ],
            media: vec![(String::from("query"), media_style_a)],
            class_selecter: Style::class_selecter(),
        };

        let mut media_style_b = Style::new();
        media_style_b.add("foo", "width", "100px");
        media_style_b.add("foo", "height", "100px");
        media_style_b.add("bar", "width", "100px");
        media_style_b.add("bar", "height", "100px");
        let mut style_b = Style::new();
        style_b.add("foo", "width", "100px");
        style_b.add("foo", "height", "100px");
        style_b.add("bar", "width", "100px");
        style_b.add("bar", "height", "100px");
        style_b.add_media("query", media_style_b);

        assert_eq!(style_a, style_b);
    }

    #[test]
    fn gen_style_by_macro() {
        let mut style_a = Style::new();
        style_a.add("foo", "width", "100px");
        style_a.add("foo", "height", "100px");
        style_a.add("bar", "width", "100px");
        style_a.add("bar", "height", "100px");

        let style_b = style! {
            "foo" {
                "width": "100px";
                "height": "100px";
            }

            "bar" {
                "width": "100px";
                "height": "100px";
            }
        };

        assert_eq!(style_a, style_b);
    }

    #[test]
    fn gen_style_with_media_by_macro() {
        let mut media_style_a = Style::new();
        media_style_a.add("foo", "width", "100px");
        media_style_a.add("foo", "height", "100px");
        media_style_a.add("bar", "width", "100px");
        media_style_a.add("bar", "height", "100px");
        let mut style_a = Style::new();
        style_a.add("foo", "width", "100px");
        style_a.add("foo", "height", "100px");
        style_a.add("bar", "width", "100px");
        style_a.add("bar", "height", "100px");
        style_a.add_media("query", media_style_a);

        let style_b = style! {
            "foo" {
                "width": "100px";
                "height": "100px";
            }

            "bar" {
                "width": "100px";
                "height": "100px";
            }

            @media "query" {
                "foo" {
                "width": "100px";
                "height": "100px";
                }

                "bar" {
                    "width": "100px";
                    "height": "100px";
                }
            }
        };

        assert_eq!(style_a, style_b);
    }
}
