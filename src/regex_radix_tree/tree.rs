use super::item::Item;
use super::trace::Trace;
#[cfg(feature = "dot")]
use crate::dot::DotBuilder;
use crate::regex_radix_tree::iter::{ItemIter, ItemIterMut};
#[cfg(feature = "dot")]
use dot_graph::Graph;

#[derive(Debug)]
pub struct RegexTreeMap<V> {
    pub(crate) root: Item<V>,
}

#[derive(Debug)]
pub struct UniqueRegexTreeMap<V> {
    pub(crate) tree: RegexTreeMap<V>,
}

impl<V> Clone for RegexTreeMap<V>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        RegexTreeMap { root: self.root.clone() }
    }
}

impl<V> RegexTreeMap<V> {
    pub fn new(ignore_case: bool) -> Self {
        RegexTreeMap {
            root: Item::Empty(ignore_case),
        }
    }

    pub fn insert(&mut self, regex: &str, id: &str, item: V) {
        let mut root = Item::Empty(false);
        std::mem::swap(&mut self.root, &mut root);

        self.root = root.insert(regex, id.to_string(), item);
    }

    pub fn remove(&mut self, id: &str) -> Option<V> {
        let mut root = Item::Empty(false);
        std::mem::swap(&mut self.root, &mut root);
        let (new_root, removed) = root.remove(id);
        self.root = new_root;

        removed
    }

    pub fn retain<F>(&mut self, f: &F)
    where
        F: Fn(&str, &mut V) -> bool,
    {
        let mut root = Item::Empty(false);
        std::mem::swap(&mut self.root, &mut root);
        let root = root.retain(f);
        self.root = root;
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn cached_len(&self) -> usize {
        self.root.cached_len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn find(&self, haystack: &str) -> Vec<&V> {
        self.root.find(haystack)
    }

    pub fn get(&self, regex: &str) -> Vec<&V> {
        self.root.get(regex)
    }

    pub fn get_mut(&mut self, regex: &str) -> Vec<&mut V> {
        self.root.get_mut(regex)
    }

    pub fn cache(&mut self, limit: u64, level: Option<u64>) -> u64 {
        let mut left = limit;

        if let Some(level) = level {
            return self.root.cache(left, level, 0);
        }

        let mut cache_level = 0;

        while left > 0 {
            let new_left = self.root.cache(left, cache_level, 0);

            // If we did not cache anything, we can stop
            if new_left == left {
                break;
            }

            left = new_left;
            cache_level += 1;
        }

        left
    }

    pub fn trace(&self, haystack: &str) -> Trace<V> {
        self.root.trace(haystack)
    }

    pub fn iter(&self) -> ItemIter<'_, V> {
        self.root.iter()
    }

    pub fn iter_mut(&mut self) -> ItemIterMut<'_, V> {
        self.root.iter_mut()
    }
}

#[cfg(feature = "dot")]
impl<V> DotBuilder for RegexTreeMap<V>
where
    V: DotBuilder,
{
    fn graph(&self, id: &mut u32, graph: &mut Graph) -> Option<String> {
        self.root.graph(id, graph)
    }
}

impl<V> Clone for UniqueRegexTreeMap<V>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        UniqueRegexTreeMap { tree: self.tree.clone() }
    }
}

impl<V> UniqueRegexTreeMap<V> {
    pub fn new(ignore_case: bool) -> Self {
        UniqueRegexTreeMap {
            tree: RegexTreeMap::new(ignore_case),
        }
    }

    pub fn insert(&mut self, regex: &str, item: V) {
        self.tree.insert(regex, regex, item);
    }

    pub fn remove(&mut self, regex: &str) -> Option<V> {
        self.tree.remove(regex)
    }

    pub fn retain<F>(&mut self, f: &F)
    where
        F: Fn(&str, &mut V) -> bool,
    {
        self.tree.retain(f)
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn find(&self, haystack: &str) -> Vec<&V> {
        self.tree.find(haystack)
    }

    pub fn get(&self, regex: &str) -> Option<&V> {
        self.tree.get(regex).pop()
    }

    pub fn get_mut(&mut self, regex: &str) -> Option<&mut V> {
        self.tree.get_mut(regex).pop()
    }

    pub fn cache(&mut self, limit: u64, level: Option<u64>) -> u64 {
        self.tree.cache(limit, level)
    }

    pub fn trace(&self, haystack: &str) -> Trace<V> {
        self.tree.trace(haystack)
    }

    pub fn iter(&self) -> ItemIter<'_, V> {
        self.tree.iter()
    }

    pub fn iter_mut(&mut self) -> ItemIterMut<'_, V> {
        self.tree.iter_mut()
    }
}

#[cfg(feature = "dot")]
impl<V> DotBuilder for UniqueRegexTreeMap<V>
where
    V: DotBuilder,
{
    fn graph(&self, id: &mut u32, graph: &mut Graph) -> Option<String> {
        self.tree.graph(id, graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unique() {
        let mut tree = UniqueRegexTreeMap::<String>::new(false);
        tree.insert("/a/b", "tata".to_string());
        tree.insert("/b/a", "yolo".to_string());
        tree.insert("/a/b", "tutu".to_string());
        tree.insert("/a/b", "titi".to_string());
        tree.insert("/a/b", "tyty".to_string());
        tree.insert("/a/b", "toto".to_string());

        assert!(tree.find("test").is_empty());
        assert!(!tree.find("/a/b").is_empty());
        assert_eq!(tree.get("/a/b").unwrap(), "toto");

        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn test_get_unique_update() {
        let mut tree = UniqueRegexTreeMap::<String>::new(false);
        tree.insert("/a/b", "tata".to_string());
        tree.insert("/b/a", "yolo".to_string());

        tree.get_mut("/a/b").unwrap().push_str("toto");

        assert!(tree.find("test").is_empty());
        assert!(!tree.find("/a/b").is_empty());
        assert_eq!(tree.get("/a/b").unwrap(), "tatatoto");

        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn test_find_no_rule() {
        let tree = RegexTreeMap::<String>::new(false);

        assert!(tree.find("tata").is_empty());
        assert!(tree.find("test").is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_find_one_rule() {
        let mut tree = RegexTreeMap::<String>::new(false);
        tree.insert("tata", "tata", "tata".to_string());

        assert!(!tree.find("tata").is_empty());
        assert!(tree.find("test").is_empty());
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_find_emoji_rule_regex() {
        let mut tree = RegexTreeMap::<String>::new(false);
        tree.insert("/emoji/(([\\p{Ll}]|\\-|➡️|🤘)+?)", "tata", "tata".to_string());

        assert!(!tree.find("/emoji/test").is_empty());
        assert!(!tree.find("/emoji/➡️").is_empty());
        assert!(!tree.find("/emoji/🤘").is_empty());
        assert!(tree.find("/not-emoji").is_empty());
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_find_multiple_rule_simple() {
        let mut tree = RegexTreeMap::<String>::new(false);
        tree.insert("/a/b", "tata", "tata".to_string());
        tree.insert("/a/b/c", "tata", "tata".to_string());
        tree.insert("/a/b/d", "tata", "tata".to_string());
        tree.insert("/b/a", "tata", "tata".to_string());

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
        let mut tree = RegexTreeMap::<String>::new(false);
        tree.insert("/a/(.+?)/c", "tata", "tata".to_string());

        assert!(!tree.find("/a/b/c").is_empty());
        assert!(tree.find("/a/b/d").is_empty());
        assert!(tree.find("/a/b").is_empty());
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_find_multiple_rule_with_regex() {
        let mut tree = RegexTreeMap::<String>::new(false);
        tree.insert("/a/(.+?)/c", "tata", "tata".to_string());
        tree.insert("/a/(.+?)/b", "tata", "tata".to_string());

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
        let mut tree = RegexTreeMap::<String>::new(false);
        tree.insert("/a/b", "1", "tata".to_string());
        tree.insert("/a/b/c", "2", "tata".to_string());
        tree.insert("/a/b/d", "3", "tata".to_string());
        tree.insert("/b/a", "4", "tata".to_string());

        assert!(!tree.find("/a/b").is_empty());
        assert!(!tree.find("/a/b/c").is_empty());
        assert_eq!(tree.len(), 4);

        tree.remove("1");

        assert!(tree.find("/a/b").is_empty());
        assert!(!tree.find("/a/b/c").is_empty());
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn test_find_emoji_weird_rule_regex() {
        let mut tree = RegexTreeMap::<String>::new(false);
        tree.insert("/string/from/(?:)", "1", "tata".to_string());
        tree.insert("/string\\-uppercase/from/(?:([\\p{Lu}\\p{Lt}])+?)", "2", "tata".to_string());
        tree.insert("/string\\-ending/from/(?:([\\p{Ll}]|\\-)+?JOHN\\-SNOW)", "3", "tata".to_string());
        tree.insert("/string\\-lowercase/from/(?:([\\p{Ll}])+?)", "4", "tata".to_string());
        tree.insert("/string\\-starting/from/(?:JOHN\\-SNOW([\\p{Ll}]|\\-)+?)", "5", "tata".to_string());
        tree.insert(
            "/string\\-lowercase\\-uppercase\\-digits/from/(?:([\\p{Ll}\\p{Lu}\\p{Lt}0-9])+?)",
            "6",
            "tata".to_string(),
        );
        tree.insert("/string\\-lowercase\\-uppercase\\-digits\\-allowPercentEncodedChars\\-specificCharacters/from/(?:([\\p{Ll}\\p{Lu}\\p{Lt}0-9]|\\-|\\.|\\(|\\)|%[0-9A-Z]{2})+?)", "7", "tata".to_string());
        tree.insert(
            "/string\\-starting\\-shit/from/(?:\\(\\[A\\-Z\\]\\)\\+([\\p{Ll}]|\\-)+?)",
            "8",
            "tata".to_string(),
        );
        tree.insert(
            "/string\\-lowercase\\-specificCharacters\\-emoji/from/(?:([\\p{Ll}]|\\-|🤘)+?)",
            "9",
            "tata".to_string(),
        );
        tree.insert(
            "/string\\-lowercase\\-digits\\-allowPercentEncodedChars/from/(?:([\\p{Ll}0-9]|%[0-9A-Z]{2})+?)",
            "10",
            "tata".to_string(),
        );
        tree.insert("/string\\-allowLowercaseAlphabet\\-specificCharacters\\-starting\\-containing/from/(?:JOHN\\-SNOW(([\\p{Ll}]|\\-)*?L33T([\\p{Ll}]|\\-)*?)+?)", "11", "tata".to_string());
        tree.insert(
            "/string\\-allowPercentEncodedChars/from/(?:(%[0-9A-Z]{2})+?)",
            "12",
            "tata".to_string(),
        );
        tree.insert("/string\\-containing/from/(?:(L33T)+?)", "13", "tata".to_string());
        tree.insert(
            "/string\\-specificCharacters/from/(?:(\\.|\\-|\\+|_|/)+?)",
            "14",
            "tata".to_string(),
        );
        tree.insert(
            "/string\\-specificCharacters\\-other/from/(?:(a|\\-|z)+?)",
            "15",
            "tata".to_string(),
        );

        assert!(!tree.find("/string-lowercase/from/coucou").is_empty());
        assert!(
            !tree
                .find("/string-lowercase-specificCharacters-emoji/from/you-rock-dude-🤘")
                .is_empty()
        );
    }
}
