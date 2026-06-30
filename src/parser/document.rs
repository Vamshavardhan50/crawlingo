use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;

/// A node in the custom in-memory DOM Tree.
#[derive(Debug, Clone)]
pub struct DomNode {
    pub index: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub tag: String,
    pub text: String,
    pub attrs: HashMap<String, String>,
    pub depth: usize,
    pub html_snippet: String,
}

/// The parsed HTML DOM Tree using vector-based index routing to avoid reference cycles.
#[derive(Debug, Clone)]
pub struct DomTree {
    pub nodes: Vec<DomNode>,
}

impl DomTree {
    /// Creates a new, empty DOM Tree.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Appends a node and updates the parent relationship.
    pub fn add_node(&mut self, mut node: DomNode) -> usize {
        let new_idx = self.nodes.len();
        node.index = new_idx;

        if let Some(parent_idx) = node.parent {
            if parent_idx < self.nodes.len() {
                self.nodes[parent_idx].children.push(new_idx);
            }
        }

        self.nodes.push(node);
        new_idx
    }

    /// Gets the text content of a node and all of its descendants recursively.
    pub fn get_text(&self, idx: usize) -> String {
        let mut result = String::new();
        self.gather_text(idx, &mut result);
        result
    }

    fn gather_text(&self, idx: usize, buf: &mut String) {
        if let Some(node) = self.nodes.get(idx) {
            // Append local text
            let local_text = node.text.trim();
            if !local_text.is_empty() {
                if !buf.is_empty() && !buf.ends_with(' ') {
                    buf.push(' ');
                }
                buf.push_str(local_text);
            }
            for &child_idx in &node.children {
                self.gather_text(child_idx, buf);
            }
        }
    }
}

impl Default for DomTree {
    fn default() -> Self {
        Self::new()
    }
}

// PyO3 Bindings exposing DOM elements to Python
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "Element")]
#[derive(Clone)]
pub struct PyElement {
    pub tree: Arc<DomTree>,
    pub node_index: usize,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyElement {
    /// Get the text content of the element.
    pub fn text(&self) -> String {
        self.tree.get_text(self.node_index)
    }

    /// Get the HTML snippet of the element.
    pub fn html(&self) -> String {
        self.tree
            .nodes
            .get(self.node_index)
            .map(|n| n.html_snippet.clone())
            .unwrap_or_default()
    }

    /// Get an attribute value.
    pub fn attr(&self, name: &str) -> Option<String> {
        self.tree
            .nodes
            .get(self.node_index)
            .and_then(|n| n.attrs.get(name).cloned())
    }

    /// Get all attributes as a dictionary.
    pub fn attrs(&self) -> HashMap<String, String> {
        self.tree
            .nodes
            .get(self.node_index)
            .map(|n| n.attrs.clone())
            .unwrap_or_default()
    }

    /// Get the parent element.
    pub fn parent(&self) -> Option<PyElement> {
        let node = self.tree.nodes.get(self.node_index)?;
        let parent_idx = node.parent?;
        Some(PyElement {
            tree: self.tree.clone(),
            node_index: parent_idx,
        })
    }

    /// Get children.
    pub fn children(&self) -> PyElementCollection {
        let child_indices = self
            .tree
            .nodes
            .get(self.node_index)
            .map(|n| n.children.clone())
            .unwrap_or_default();
        PyElementCollection {
            tree: self.tree.clone(),
            node_indices: child_indices,
        }
    }

    /// Get next sibling.
    pub fn next(&self) -> Option<PyElement> {
        let node = self.tree.nodes.get(self.node_index)?;
        let parent_idx = node.parent?;
        let parent_node = self.tree.nodes.get(parent_idx)?;
        let pos = parent_node
            .children
            .iter()
            .position(|&idx| idx == self.node_index)?;
        let next_sibling_idx = *parent_node.children.get(pos + 1)?;
        Some(PyElement {
            tree: self.tree.clone(),
            node_index: next_sibling_idx,
        })
    }

    /// Get previous sibling.
    pub fn prev(&self) -> Option<PyElement> {
        let node = self.tree.nodes.get(self.node_index)?;
        let parent_idx = node.parent?;
        let parent_node = self.tree.nodes.get(parent_idx)?;
        let pos = parent_node
            .children
            .iter()
            .position(|&idx| idx == self.node_index)?;
        if pos == 0 {
            return None;
        }
        let prev_sibling_idx = *parent_node.children.get(pos - 1)?;
        Some(PyElement {
            tree: self.tree.clone(),
            node_index: prev_sibling_idx,
        })
    }

    /// Get all other siblings.
    pub fn siblings(&self) -> PyElementCollection {
        let mut sibling_indices = Vec::new();
        if let Some(node) = self.tree.nodes.get(self.node_index) {
            if let Some(parent_idx) = node.parent {
                if let Some(parent_node) = self.tree.nodes.get(parent_idx) {
                    sibling_indices = parent_node
                        .children
                        .iter()
                        .copied()
                        .filter(|&idx| idx != self.node_index)
                        .collect();
                }
            }
        }
        PyElementCollection {
            tree: self.tree.clone(),
            node_indices: sibling_indices,
        }
    }
}

/// PyO3 Python wrapper for a collection of elements.
#[cfg(feature = "python")]
#[pyclass(name = "ElementCollection")]
#[derive(Clone)]
pub struct PyElementCollection {
    pub tree: Arc<DomTree>,
    pub node_indices: Vec<usize>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyElementCollection {
    /// Get first element.
    pub fn first(&self) -> Option<PyElement> {
        let &first_idx = self.node_indices.first()?;
        Some(PyElement {
            tree: self.tree.clone(),
            node_index: first_idx,
        })
    }

    /// Get last element.
    pub fn last(&self) -> Option<PyElement> {
        let &last_idx = self.node_indices.last()?;
        Some(PyElement {
            tree: self.tree.clone(),
            node_index: last_idx,
        })
    }

    /// Get nth element.
    pub fn nth(&self, n: usize) -> Option<PyElement> {
        let &idx = self.node_indices.get(n)?;
        Some(PyElement {
            tree: self.tree.clone(),
            node_index: idx,
        })
    }

    /// Join and return text of all matched nodes.
    pub fn text(&self) -> String {
        self.node_indices
            .iter()
            .map(|&idx| self.tree.get_text(idx))
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Get list of text values.
    pub fn texts(&self) -> Vec<String> {
        self.node_indices
            .iter()
            .map(|&idx| self.tree.get_text(idx))
            .collect()
    }

    /// Get attribute value of the first matching node.
    pub fn attr(&self, name: &str) -> Option<String> {
        let idx = *self.node_indices.first()?;
        self.tree.nodes[idx].attrs.get(name).cloned()
    }

    /// Get attributes of the first matching node.
    pub fn attrs(&self) -> HashMap<String, String> {
        if let Some(&idx) = self.node_indices.first() {
            self.tree.nodes[idx].attrs.clone()
        } else {
            HashMap::new()
        }
    }

    /// Length of the collection.
    pub fn __len__(&self) -> usize {
        self.node_indices.len()
    }
}

/// The core Page object representing a fetched and parsed web page document.
pub struct Page {
    url: String,
    status: u16,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    html: String,
    tree: Arc<DomTree>,
    text_cache: OnceCell<String>,
    links_cache: OnceCell<Vec<String>>,
}

impl Page {
    pub fn new(
        url: String,
        status: u16,
        headers: HashMap<String, String>,
        cookies: HashMap<String, String>,
        html: String,
        tree: DomTree,
    ) -> Self {
        Self {
            url,
            status,
            headers,
            cookies,
            html,
            tree: Arc::new(tree),
            text_cache: OnceCell::new(),
            links_cache: OnceCell::new(),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn cookies(&self) -> &HashMap<String, String> {
        &self.cookies
    }

    pub fn html(&self) -> &str {
        &self.html
    }

    pub fn dom_tree(&self) -> &Arc<DomTree> {
        &self.tree
    }

    pub fn text(&self) -> &str {
        self.text_cache.get_or_init(|| self.tree.get_text(0))
    }

    pub fn links(&self) -> &[String] {
        self.links_cache.get_or_init(|| {
            let mut resolved = Vec::new();
            let matches = crate::selector::css::query(&self.tree, "a");
            for &idx in &matches {
                if let Some(href) = self.tree.nodes[idx].attrs.get("href") {
                    if let Ok(base) = url::Url::parse(&self.url) {
                        if let Ok(abs_url) = base.join(href) {
                            resolved.push(abs_url.to_string());
                            continue;
                        }
                    }
                    resolved.push(href.clone());
                }
            }
            resolved
        })
    }

    pub fn images(&self) -> Vec<(String, Option<String>)> {
        let mut imgs = Vec::new();
        let matches = crate::selector::css::query(&self.tree, "img");
        for &idx in &matches {
            if let Some(src) = self.tree.nodes[idx].attrs.get("src") {
                let alt = self.tree.nodes[idx].attrs.get("alt").cloned();
                imgs.push((src.clone(), alt));
            }
        }
        imgs
    }

    pub fn query(
        &self,
        query: crate::selector::SelectorQuery<'_>,
    ) -> crate::error::Result<Vec<usize>> {
        crate::selector::SelectorEngine::query(&self.tree, query)
    }

    pub fn get_nodes_combined_text(&self, indices: &[usize]) -> String {
        indices
            .iter()
            .map(|&idx| self.tree.get_text(idx))
            .collect::<Vec<String>>()
            .join(" ")
    }
}
