use crate::regex_radix_tree::regex_node::RegexNode;
use crate::regex_radix_tree::{Node, NodeItem, Storage, Trace};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct RegexRadixTree<T: NodeItem, S: Storage<T>> {
    root: RegexNode<T, S>,
}

impl<T: NodeItem, S: Storage<T>> Default for RegexRadixTree<T, S> {
    fn default() -> Self {
        RegexRadixTree {
            root: RegexNode::default(),
        }
    }
}

impl<T: NodeItem, S: Storage<T>> RegexRadixTree<T, S> {
    pub fn insert(&mut self, item: T) {
        self.root.insert(item, 0)
    }

    pub fn remove(&mut self, id: &str) -> bool {
        self.root.remove(id)
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn trace(&self, value: &str) -> Trace<T, S> {
        self.root.trace(value)
    }

    pub fn find(&self, value: &str) -> Vec<&S> {
        self.root.find(value)
    }

    pub fn cache(&mut self, limit: u64, level: u64) -> u64 {
        self.root.cache(limit, level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex_radix_tree::VecStorageItem;

    #[derive(Debug, Clone)]
    struct TestItem {
        regex: String,
        id: String,
    }

    impl NodeItem for TestItem {
        fn regex(&self) -> &str {
            self.regex.as_str()
        }

        fn case_insensitive(&self) -> bool {
            false
        }
    }

    impl VecStorageItem for TestItem {
        fn id(&self) -> &str {
            self.id.as_str()
        }
    }

    impl TestItem {
        pub fn new(regex: String) -> TestItem {
            TestItem { id: regex.clone(), regex }
        }
    }

    #[test]
    fn test_find_no_rule() {
        let tree: RegexRadixTree<TestItem, Vec<TestItem>> = RegexRadixTree::default();

        assert!(tree.find("tata").is_empty());
        assert!(tree.find("test").is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_find_one_rule() {
        let item1 = TestItem::new("tata".to_string());
        let mut tree: RegexRadixTree<TestItem, Vec<TestItem>> = RegexRadixTree::default();

        tree.insert(item1);

        assert!(!tree.find("tata").is_empty());
        assert!(tree.find("test").is_empty());
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_find_emoji_rule_regex() {
        let mut tree: RegexRadixTree<TestItem, Vec<TestItem>> = RegexRadixTree::default();

        tree.insert(TestItem::new("/emoji/(([\\p{Ll}]|\\-|➡️|🤘)+?)".to_string()));

        assert!(!tree.find("/emoji/test").is_empty());
        assert!(!tree.find("/emoji/➡️").is_empty());
        assert!(!tree.find("/emoji/🤘").is_empty());
        assert!(tree.find("/not-emoji").is_empty());
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_find_multiple_rule() {
        let mut tree: RegexRadixTree<TestItem, Vec<TestItem>> = RegexRadixTree::default();

        tree.insert(TestItem::new("/a/b".to_string()));
        tree.insert(TestItem::new("/a/b/c".to_string()));
        tree.insert(TestItem::new("/a/b/d".to_string()));
        tree.insert(TestItem::new("/b/a".to_string()));

        assert!(!tree.find("/a/b").is_empty());
        assert!(!tree.find("/a/b/c").is_empty());
        assert!(!tree.find("/a/b/d").is_empty());
        assert!(!tree.find("/b/a").is_empty());

        assert!(tree.find("/b").is_empty());
        assert!(tree.find("/a").is_empty());
        assert!(tree.find("/no-match").is_empty());
        assert_eq!(tree.len(), 4);
    }

    #[test]
    fn test_find_rule_with_regex() {
        let mut tree: RegexRadixTree<TestItem, Vec<TestItem>> = RegexRadixTree::default();

        tree.insert(TestItem::new("/a/(.+?)/c".to_string()));

        assert!(!tree.find("/a/b/c").is_empty());
        assert!(tree.find("/a/b/d").is_empty());
        assert!(tree.find("/a/b").is_empty());
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_find_multiple_rule_with_regex() {
        let mut tree: RegexRadixTree<TestItem, Vec<TestItem>> = RegexRadixTree::default();

        tree.insert(TestItem::new("/a/(.+?)/c".to_string()));
        tree.insert(TestItem::new("/a/(.+?)/b".to_string()));

        assert!(!tree.find("/a/b/c").is_empty());
        assert!(tree.find("/a/b/d").is_empty());
        assert!(tree.find("/a/b").is_empty());
        assert!(!tree.find("/a/b/b").is_empty());
        assert!(!tree.find("/a/c/b").is_empty());
        assert!(tree.find("/a/c/d").is_empty());
        assert!(tree.find("/a/c/").is_empty());
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn test_find_multiple_rule_after_remove() {
        let mut tree: RegexRadixTree<TestItem, Vec<TestItem>> = RegexRadixTree::default();

        tree.insert(TestItem::new("/a/b".to_string()));
        tree.insert(TestItem::new("/a/b/c".to_string()));
        tree.insert(TestItem::new("/a/b/d".to_string()));
        tree.insert(TestItem::new("/b/a".to_string()));

        assert!(!tree.find("/a/b").is_empty());
        assert!(!tree.find("/a/b/c").is_empty());
        assert_eq!(tree.len(), 4);

        tree.remove("/a/b");

        assert!(tree.find("/a/b").is_empty());
        assert!(!tree.find("/a/b/c").is_empty());
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn test_find_emoji_weird_rule_regex() {
        let mut tree: RegexRadixTree<TestItem, Vec<TestItem>> = RegexRadixTree::default();

        tree.insert(TestItem::new("/string/from/(?:)".to_string()));
        tree.insert(TestItem::new("/string\\-uppercase/from/(?:([\\p{Lu}\\p{Lt}])+?)".to_string()));
        tree.insert(TestItem::new("/string\\-ending/from/(?:([\\p{Ll}]|\\-)+?JOHN\\-SNOW)".to_string()));
        tree.insert(TestItem::new("/string\\-lowercase/from/(?:([\\p{Ll}])+?)".to_string()));
        tree.insert(TestItem::new(
            "/string\\-starting/from/(?:JOHN\\-SNOW([\\p{Ll}]|\\-)+?)".to_string(),
        ));
        tree.insert(TestItem::new(
            "/string\\-lowercase\\-uppercase\\-digits/from/(?:([\\p{Ll}\\p{Lu}\\p{Lt}0-9])+?)".to_string(),
        ));
        tree.insert(TestItem::new("/string\\-lowercase\\-uppercase\\-digits\\-allowPercentEncodedChars\\-specificCharacters/from/(?:([\\p{Ll}\\p{Lu}\\p{Lt}0-9]|\\-|\\.|\\(|\\)|%[0-9A-Z]{2})+?)".to_string()));
        tree.insert(TestItem::new(
            "/string\\-starting\\-shit/from/(?:\\(\\[A\\-Z\\]\\)\\+([\\p{Ll}]|\\-)+?)".to_string(),
        ));
        tree.insert(TestItem::new(
            "/string\\-lowercase\\-specificCharacters\\-emoji/from/(?:([\\p{Ll}]|\\-|🤘)+?)".to_string(),
        ));
        tree.insert(TestItem::new(
            "/string\\-lowercase\\-digits\\-allowPercentEncodedChars/from/(?:([\\p{Ll}0-9]|%[0-9A-Z]{2})+?)".to_string(),
        ));
        tree.insert(TestItem::new("/string\\-allowLowercaseAlphabet\\-specificCharacters\\-starting\\-containing/from/(?:JOHN\\-SNOW(([\\p{Ll}]|\\-)*?L33T([\\p{Ll}]|\\-)*?)+?)".to_string()));
        tree.insert(TestItem::new(
            "/string\\-allowPercentEncodedChars/from/(?:(%[0-9A-Z]{2})+?)".to_string(),
        ));
        tree.insert(TestItem::new("/string\\-containing/from/(?:(L33T)+?)".to_string()));
        tree.insert(TestItem::new(
            "/string\\-specificCharacters/from/(?:(\\.|\\-|\\+|_|/)+?)".to_string(),
        ));
        tree.insert(TestItem::new(
            "/string\\-specificCharacters\\-other/from/(?:(a|\\-|z)+?)".to_string(),
        ));

        assert!(!tree.find("/string-lowercase/from/coucou").is_empty());
        assert!(!tree
            .find("/string-lowercase-specificCharacters-emoji/from/you-rock-dude-🤘")
            .is_empty());
    }
}
