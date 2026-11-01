//! Reading values from the invoice XML.

pub const RSM: &str = "urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100";
pub const RAM: &str =
    "urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100";

/// The text at `path` below a `root` element. `None` when the XML cannot be parsed or its
/// root is not `root`; an empty string when nothing is found.
pub fn first_text(xml: &str, root: (&str, &str), path: &[(&str, &str)]) -> Option<String> {
    let doc = roxmltree::Document::parse(xml).ok()?;
    let (ns, name) = root;
    if !is_element_named(doc.root_element(), ns, name) {
        return None;
    }
    let mut current = vec![doc.root_element()];
    for &(ns, name) in path {
        current = current
            .iter()
            .flat_map(roxmltree::Node::children)
            .filter(|c| is_element_named(*c, ns, name))
            .collect();
    }
    let mut text = String::new();
    if let Some(element) = current.first() {
        // Every text piece, comments skipped: how the official rules (XPath) read a value.
        for child in element.children() {
            if child.is_text() {
                text.push_str(child.text().unwrap_or(""));
            }
        }
    }
    Some(text)
}

fn is_element_named(node: roxmltree::Node, ns: &str, name: &str) -> bool {
    node.is_element() && node.tag_name().namespace() == Some(ns) && node.tag_name().name() == name
}
