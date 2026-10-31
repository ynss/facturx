//! Reading values from the invoice XML.

/// First element text reachable by a path of local names from the root.
pub fn first_text_by_local(xml: &str, locals: &[&str]) -> Option<String> {
    let doc = roxmltree::Document::parse(xml).ok()?;
    let mut current = vec![doc.root_element()];
    for &name in locals {
        let mut next = Vec::new();
        for n in &current {
            for c in n
                .children()
                .filter(|c| c.is_element() && c.tag_name().name() == name)
            {
                next.push(c);
            }
        }
        current = next;
    }
    current
        .first()
        .and_then(roxmltree::Node::text)
        .map(String::from)
}
