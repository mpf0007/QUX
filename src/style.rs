//! Code for applying CSS styles to the DOM.
//!
//! This is not very interesting at the moment.  It will get much more
//! complicated if I add support for compound selectors.

use crate::css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value};
use crate::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;

/*
The output of style module is something I call the style tree.
Each node in this tree includes a pointer to a DOM node, plus its CSS property values
*/

// Map from CSS property names to values.
pub type PropertyMap = HashMap<String, Value>;

// a node with associated style data.
pub struct StyledNode<'a> {
    pub node: &'a Node, // pointer to a dom
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
    // What’s with all the 'a stuff? Those are lifetimes, part of how Rust guarantees that pointers are memory-safe without requiring garbage collection.
    //  If you’re not working in Rust you can ignore them; they aren’t critical to the code’s meaning.
}

#[derive(PartialEq)]
pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyledNode<'a> {
    /// Return the specified value of a property if it exists, otherwise `None`.
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    /// Return the specified value of property `name`, or property `fallback_name` if that doesn't
    /// exist, or value `default` if neither does.
    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }

    /// The value of the `display` property (defaults to inline).
    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}

/// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
///
/// This finds only the specified values at the moment. Eventually it should be extended to find the
/// computed values too, including inherited values.
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => HashMap::new(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}

/// Apply styles to a single element, returning the specified styles.
///
/// To do: Allow multiple UA/author/user stylesheets, and implement the cascade.
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Go through the rules from lowest to highest specificity.
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

/// A single CSS rule and the specificity of its most specific matching selector.
type MatchedRule<'a> = (Specificity, &'a Rule);

/// Find all CSS rules that match the given element.
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    // For now, we just do a linear scan of all the rules.  For large
    // documents, it would be more efficient to store the rules in hash tables
    // based on tag name, id, class, etc.
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

/// If `rule` matches `elem`, return a `MatchedRule`. Otherwise return `None`.
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    // Find the first (most specific) matching selector.
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

// The first step in building the style tree is selector matching.
// This will be very easy, since my CSS parser supports only simple selectors.
// You can tell whether a simple selector matches an element just by looking at the element itself.
// Matching compound selectors would require traversing the DOM tree to look at the element’s siblings, parents, etc.
/// Selector matching:
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

// To test whether a simple selector matches an element, just look at each selector component,
// and return false if the element doesn’t have a matching class, ID, or tag name.

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check type selector
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // Check ID selector
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // Check class selectors
    let elem_classes = elem.classes();
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    }

    // We didn't find any non-matching selector components.
    true

    // Rust note: This function uses the any method, which returns true if an iterator contains an element that passes the provided test.
    // This is the same as the any function in Python (or Haskell), or the some method in JavaScript.
}
