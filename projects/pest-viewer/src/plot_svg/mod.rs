use pest::iterators::{Pair, Pairs};
use std::borrow::Cow;

use crate::helper::{get_children, has_child, width_hint};
use pest::RuleType;
use shape_svg::ToSVG;
use svg::{
    node::element::{Text, SVG},
    Document,
};
use tree_layout::{layout, Line, NodeInfo, Point, TreeBox};

/// Plot a svg structure
#[derive(Debug)]
pub struct SvgPlotter {
    style: Cow<'static, str>,
}

impl Default for SvgPlotter {
    fn default() -> Self {
        Self { style: Cow::Borrowed(include_str!("style.css")) }
    }
}

#[derive(Debug)]
struct SvgTree<'i, R>
where
    R: RuleType,
{
    cst: Pairs<'i, R>,
}

impl<'i, R> NodeInfo<Pair<'i, R>> for SvgTree<'i, R>
where
    R: RuleType,
{
    type Key = Pair<'i, R>;

    fn key(&self, node: Pair<'i, R>) -> Self::Key {
        node
    }

    fn children(&self, node: Pair<'i, R>) -> impl Iterator<Item = Pair<'i, R>> {
        get_children(&node).into_iter()
    }

    fn dimensions(&self, node: Pair<'i, R>) -> TreeBox {
        let chars = width_hint(node);
        TreeBox::rectangle(chars * 6.0, 16.0)
    }
    fn border(&self, _: Pair<'i, R>) -> TreeBox {
        TreeBox::rectangle(16.0, 8.0)
    }
}

impl<'i, R> SvgTree<'i, R>
where
    R: RuleType,
{
    fn as_svg(&self) -> SVG {
        let mut document = Document::new();
        let root = self.cst.clone().into_iter().next().unwrap();
        let layout = layout(self, root);
        let mut max = Point::default();
        for node in layout.clone() {
            let area = node.data.boundary();
            if area.max.x > max.x {
                max.x = area.max.x;
            }
            if area.max.y > max.y {
                max.y = area.max.y;
            }
            let pair = node.data.key.clone();

            match layout.find_parent(&node) {
                Some(s) => {
                    let parent_box = s.data.boundary();
                    let parent_lower = Point { x: (parent_box.min.x + parent_box.max.x) / 2.0, y: parent_box.max.y };
                    let this_upper = Point { x: (area.min.x + area.max.x) / 2.0, y: area.min.y };
                    document = document.add(Line::new(parent_lower, this_upper).to_svg())
                }
                None => {}
            }

            let mut text = Text::new().set("x", area.min.x + area.width() / 2.0).set("y", area.min.y + area.height() / 2.0);
            if has_child(&pair) {
                text = text.add(svg::node::Text::new(format!("{:?}", pair.as_rule()).trim_matches('"'))).set("class", "node");
                document = document.add(area.to_svg().set("rx", 5).set("ry", 5).set("class", "node"));
            }
            else {
                text = text.add(svg::node::Text::new(safe_html(pair.as_str()))).set("class", "leaf");
                document = document.add(area.to_svg().set("rx", 5).set("ry", 5).set("class", "leaf"));
            }
            document = document.add(text);
        }
        document.add(svg::node::element::Style::new(include_str!("style.css"))).set("viewBox", (0, 0, max.x, max.y))
    }
}

fn safe_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

impl SvgPlotter {
    /// Custom style css
    pub fn with_style(self, s: impl Into<Cow<'static, str>>) -> Self {
        Self { style: s.into(), ..self }
    }

    /// Draw a svg
    pub fn draw<R>(&self, tree: Pairs<R>) -> SVG
    where
        R: RuleType,
    {
        SvgTree { cst: tree }.as_svg()
    }
}
