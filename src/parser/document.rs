use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;

/// A fetched image resource with optional alt text.
#[derive(Debug, Clone)]
pub struct ImageResource {
    pub src: String,
    pub alt: Option<String>,
}

/// Parsed tabular data from HTML tables.
#[derive(Debug, Clone)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

/// Extracted form controls from HTML forms.
#[derive(Debug, Clone)]
pub struct FormDetails {
    pub action: Option<String>,
    pub method: Option<String>,
    pub inputs: Vec<String>,
}

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
    markdown_cache: OnceCell<String>,
    links_cache: OnceCell<Vec<String>>,
    images_cache: OnceCell<Vec<ImageResource>>,
    tables_cache: OnceCell<Vec<TableData>>,
    forms_cache: OnceCell<Vec<FormDetails>>,
    scripts_cache: OnceCell<Vec<String>>,
    styles_cache: OnceCell<Vec<String>>,
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
            markdown_cache: OnceCell::new(),
            links_cache: OnceCell::new(),
            images_cache: OnceCell::new(),
            tables_cache: OnceCell::new(),
            forms_cache: OnceCell::new(),
            scripts_cache: OnceCell::new(),
            styles_cache: OnceCell::new(),
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

    /// Lazy-converts the DOM to clean markdown text.
    pub fn markdown(&self) -> &str {
        self.markdown_cache.get_or_init(|| {
            let mut md = String::new();
            if let Some(root) = self.tree.nodes.first() {
                Self::dom_to_markdown(&self.tree, root.index, &mut md, 0);
            }
            md
        })
    }

    fn dom_to_markdown(tree: &DomTree, idx: usize, buf: &mut String, depth: usize) {
        if let Some(node) = tree.nodes.get(idx) {
            let tag = node.tag.to_lowercase();
            let text = node.text.trim();
            match tag.as_str() {
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    let level = tag[1..].parse::<usize>().unwrap_or(1);
                    let prefix = "#".repeat(level);
                    if !text.is_empty() {
                        buf.push_str(&format!("\n{prefix} {text}\n\n"));
                    }
                }
                "a" => {
                    if let Some(href) = node.attrs.get("href") {
                        if !text.is_empty() {
                            buf.push_str(&format!("[{text}]({href})"));
                        } else {
                            buf.push_str(&format!("<{href}>"));
                        }
                    } else if !text.is_empty() {
                        buf.push_str(text);
                    }
                }
                "img" => {
                    if let Some(src) = node.attrs.get("src") {
                        let alt = node.attrs.get("alt").map(|s| s.as_str()).unwrap_or("");
                        buf.push_str(&format!("![{alt}]({src})"));
                    }
                }
                "br" => buf.push('\n'),
                "p" | "div" | "li" => {
                    if !text.is_empty() {
                        buf.push_str(&format!("\n{text}\n"));
                    }
                }
                "hr" => buf.push_str("\n---\n"),
                _ => {
                    if !text.is_empty() {
                        buf.push_str(text);
                    }
                }
            }
            for &child_idx in &node.children {
                Self::dom_to_markdown(tree, child_idx, buf, depth + 1);
            }
        }
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

    pub fn images(&self) -> &[ImageResource] {
        self.images_cache.get_or_init(|| {
            let mut imgs = Vec::new();
            let matches = crate::selector::css::query(&self.tree, "img");
            for &idx in &matches {
                if let Some(src) = self.tree.nodes[idx].attrs.get("src") {
                    let alt = self.tree.nodes[idx].attrs.get("alt").cloned();
                    let resolved_src = if let Ok(base) = url::Url::parse(&self.url) {
                        if let Ok(abs_url) = base.join(src) {
                            abs_url.to_string()
                        } else {
                            src.clone()
                        }
                    } else {
                        src.clone()
                    };
                    imgs.push(ImageResource {
                        src: resolved_src,
                        alt,
                    });
                }
            }
            imgs
        })
    }

    pub fn tables(&self) -> &[TableData] {
        self.tables_cache.get_or_init(|| {
            let mut tables = Vec::new();
            let matches = crate::selector::css::query(&self.tree, "table");
            for &table_idx in &matches {
                let node = &self.tree.nodes[table_idx];
                let mut headers = Vec::new();
                let mut rows = Vec::new();

                for &child_idx in &node.children {
                    let child = &self.tree.nodes[child_idx];
                    if child.tag.to_lowercase() == "thead" {
                        for &th_idx in &child.children {
                            let h = self.tree.get_text(th_idx).trim().to_string();
                            if !h.is_empty() {
                                headers.push(h);
                            }
                        }
                    }
                }

                for &child_idx in &node.children {
                    let child = &self.tree.nodes[child_idx];
                    let tagname = child.tag.to_lowercase();
                    if tagname == "tr" {
                        let row = Self::extract_table_row(&self.tree, child_idx);
                        if !row.is_empty() && headers.is_empty() {
                            headers = row;
                        } else if !row.is_empty() {
                            rows.push(row);
                        }
                    } else if tagname == "tbody" || tagname == "tfoot" {
                        for &tr_idx in &child.children {
                            if let Some(tr) = self.tree.nodes.get(tr_idx) {
                                if tr.tag.to_lowercase() == "tr" {
                                    let row = Self::extract_table_row(&self.tree, tr_idx);
                                    if !row.is_empty() {
                                        rows.push(row);
                                    }
                                }
                            }
                        }
                    }
                }

                tables.push(TableData { headers, rows });
            }
            tables
        })
    }

    fn extract_table_row(tree: &DomTree, tr_idx: usize) -> Vec<String> {
        let mut row = Vec::new();
        if let Some(tr) = tree.nodes.get(tr_idx) {
            for &cell_idx in &tr.children {
                let cell = &tree.nodes[cell_idx];
                let tagname = cell.tag.to_lowercase();
                if tagname == "td" || tagname == "th" {
                    let text = tree.get_text(cell_idx).trim().to_string();
                    row.push(text);
                }
            }
        }
        row
    }

    pub fn forms(&self) -> &[FormDetails] {
        self.forms_cache.get_or_init(|| {
            let mut forms = Vec::new();
            let matches = crate::selector::css::query(&self.tree, "form");
            for &form_idx in &matches {
                let node = &self.tree.nodes[form_idx];
                let action = node.attrs.get("action").cloned();
                let method = node
                    .attrs
                    .get("method")
                    .cloned()
                    .or_else(|| Some("get".to_string()));
                let mut inputs = Vec::new();
                for &child_idx in &node.children {
                    Self::collect_form_inputs(&self.tree, child_idx, &mut inputs);
                }
                forms.push(FormDetails {
                    action,
                    method,
                    inputs,
                });
            }
            forms
        })
    }

    fn collect_form_inputs(tree: &DomTree, idx: usize, inputs: &mut Vec<String>) {
        if let Some(node) = tree.nodes.get(idx) {
            let tag = node.tag.to_lowercase();
            if tag == "input" || tag == "textarea" || tag == "select" {
                if let Some(name) = node.attrs.get("name") {
                    inputs.push(name.clone());
                }
            }
            for &child_idx in &node.children {
                Self::collect_form_inputs(tree, child_idx, inputs);
            }
        }
    }

    pub fn scripts(&self) -> &[String] {
        self.scripts_cache.get_or_init(|| {
            let mut scripts = Vec::new();
            let matches = crate::selector::css::query(&self.tree, "script");
            for &idx in &matches {
                if let Some(src) = self.tree.nodes[idx].attrs.get("src") {
                    if let Ok(base) = url::Url::parse(&self.url) {
                        if let Ok(abs_url) = base.join(src) {
                            scripts.push(abs_url.to_string());
                            continue;
                        }
                    }
                    scripts.push(src.clone());
                }
            }
            scripts
        })
    }

    pub fn styles(&self) -> &[String] {
        self.styles_cache.get_or_init(|| {
            let mut styles = Vec::new();
            let matches = crate::selector::css::query(&self.tree, "link[rel='stylesheet']");
            for &idx in &matches {
                if let Some(href) = self.tree.nodes[idx].attrs.get("href") {
                    if let Ok(base) = url::Url::parse(&self.url) {
                        if let Ok(abs_url) = base.join(href) {
                            styles.push(abs_url.to_string());
                            continue;
                        }
                    }
                    styles.push(href.clone());
                }
            }
            styles
        })
    }

    /// Returns page metadata as key-value pairs (title, description, charset, etc.).
    pub fn metadata(&self) -> HashMap<String, String> {
        let mut meta = HashMap::new();
        let title_matches = crate::selector::css::query(&self.tree, "title");
        if let Some(&title_idx) = title_matches.first() {
            let title = self.tree.get_text(title_idx).trim().to_string();
            if !title.is_empty() {
                meta.insert("title".to_string(), title);
            }
        }
        let meta_matches = crate::selector::css::query(&self.tree, "meta");
        for &meta_idx in &meta_matches {
            let node = &self.tree.nodes[meta_idx];
            if let Some(name) = node.attrs.get("name").or_else(|| node.attrs.get("property")) {
                if let Some(content) = node.attrs.get("content") {
                    meta.insert(name.to_lowercase().replace(':', "_"), content.clone());
                }
            }
            if let Some(charset) = node.attrs.get("charset") {
                meta.insert("charset".to_string(), charset.clone());
            }
        }
        if !meta.contains_key("description") {
            if let Some(desc) = meta.get("og_description") {
                meta.insert("description".to_string(), desc.clone());
            }
        }
        meta
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
