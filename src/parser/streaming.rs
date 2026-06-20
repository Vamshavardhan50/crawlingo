use crate::error::{CrawlingoError, Result};
use crate::parser::document::{DomNode, DomTree};
use lol_html::{doc_text, element, HtmlRewriter, Settings};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Parses raw HTML bytes into a queryable memory-indexed `DomTree` using `lol-html`.
pub fn parse_html(html: &[u8]) -> Result<DomTree> {
    let tree = Rc::new(RefCell::new(DomTree::new()));
    let current_stack = Rc::new(RefCell::new(Vec::<usize>::new()));

    let tree_clone = Rc::clone(&tree);
    let stack_clone = Rc::clone(&current_stack);

    // Track start tags and parent relations
    let element_handler = element!("*", move |el| {
        let tag = el.tag_name().to_lowercase();
        let attrs: HashMap<String, String> = el
            .attributes()
            .iter()
            .map(|a| (a.name().to_string(), a.value().to_string()))
            .collect();

        let parent = stack_clone.borrow().last().copied();
        let depth = stack_clone.borrow().len();

        let node = DomNode {
            index: 0,
            parent,
            children: Vec::new(),
            tag: tag.clone(),
            text: String::new(),
            attrs,
            depth,
            html_snippet: format!("<{}>", tag),
        };

        let new_idx = tree_clone.borrow_mut().add_node(node);

        // Tags that are self-closing do not get pushed to the stack
        let self_closing_tags = [
            "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
            "source", "track", "wbr",
        ];

        if !self_closing_tags.contains(&tag.as_str()) {
            stack_clone.borrow_mut().push(new_idx);
            let stack_inner = Rc::clone(&stack_clone);

            let handler: Box<
                dyn FnOnce(
                    &mut lol_html::html_content::EndTag<'_>,
                ) -> std::result::Result<
                    (),
                    Box<dyn std::error::Error + Send + Sync + 'static>,
                >,
            > = Box::new(move |_end| {
                let mut stack = stack_inner.borrow_mut();
                if let Some(pos) = stack.iter().position(|&idx| idx == new_idx) {
                    stack.truncate(pos);
                }
                Ok(())
            });
            el.on_end_tag(handler)?;
        }

        Ok(())
    });

    let tree_text_clone = Rc::clone(&tree);
    let stack_text_clone = Rc::clone(&current_stack);

    // Track text and accumulate under the current active element
    let text_handler = doc_text!(move |t| {
        let text_content = t.as_str().to_string();
        if !text_content.is_empty() {
            if let Some(&active_node_idx) = stack_text_clone.borrow().last() {
                let mut tree_mut = tree_text_clone.borrow_mut();
                if let Some(node) = tree_mut.nodes.get_mut(active_node_idx) {
                    // Ignore scripts and stylesheet code blocks
                    if node.tag != "script" && node.tag != "style" {
                        node.text.push_str(&text_content);
                    }
                }
            }
        }
        Ok(())
    });

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element_handler],
            document_content_handlers: vec![text_handler],
            ..Settings::default()
        },
        |_: &[u8]| {},
    );

    rewriter
        .write(html)
        .map_err(|e| CrawlingoError::ParseError(e.to_string()))?;
    rewriter
        .end()
        .map_err(|e| CrawlingoError::ParseError(e.to_string()))?;

    // Safely extract the compiled DomTree
    let final_tree = Rc::try_unwrap(tree)
        .map_err(|_| CrawlingoError::ParseError("Failed to unwrap DOM tree Rc".to_string()))?
        .into_inner();

    Ok(final_tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_parsing_tree() {
        let html = b"<html><body><div class='product'><h1>Laptop</h1><span class='price'>$999</span></div></body></html>";
        let tree = parse_html(html).unwrap();

        assert_eq!(tree.nodes.len(), 5); // html, body, div, h1, span
        assert_eq!(tree.nodes[0].tag, "html");
        assert_eq!(tree.nodes[2].tag, "div");
        assert_eq!(tree.nodes[2].attrs.get("class").unwrap(), "product");

        // Children and hierarchy
        assert_eq!(tree.nodes[2].children, vec![3, 4]); // h1 and span
        assert_eq!(tree.nodes[3].parent, Some(2));

        // Text accumulation
        assert_eq!(tree.get_text(3), "Laptop");
        assert_eq!(tree.get_text(4), "$999");
        assert_eq!(tree.get_text(2), "Laptop $999"); // recursive descendants text
    }
}
