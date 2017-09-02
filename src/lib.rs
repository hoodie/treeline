use std::fmt::{self, Display};

/// a simple recursive type which is able to render its
/// components in a tree-like format
#[derive(Debug)]
pub struct Tree<D: Display>(D, Vec<Tree<D>>);

impl<D: Display> Tree<D> {
    pub fn new(root: D, leaves: Vec<Tree<D>>) -> Tree<D> {
        Tree(root, leaves)
    }

    pub fn root(root: D) -> Tree<D> {
        Tree(root, Vec::new())
    }

    pub fn push(&mut self, leaf: Tree<D>) -> &mut Self {
        self.1.push(leaf);
        self
    }

    pub fn write<T: TreeConfig>(&self, f: &mut fmt::Formatter, config: &T) -> fmt::Result {
        let Tree(ref root, ref leaves) = *self;
        let _ = writeln!(f, "{}", root);
        Self::display_leaves(f, leaves, Vec::new(), config)
    }

    fn display_leaves<T: TreeConfig>(f: &mut fmt::Formatter,
                      leaves: &Vec<Tree<D>>,
                      spaces: Vec<bool>,
                      config: &T)
                      -> fmt::Result {
        for (i, leaf) in leaves.iter().enumerate() {
            let last = i >= leaves.len() - 1;
            let mut clone = spaces.clone();
            // print single line
            for s in &spaces {
                if *s {
                    let _ = write!(f, "{}{} ", config.space(), config.space().repeat(config.depth()));
                } else {
                    let _ = write!(f, "{}{} ", config.bar(), config.space().repeat(config.depth()));
                }
            }
            if last {
                let _ = writeln!(f, "{}{} {}", config.last(), config.line().repeat(config.depth()), leaf.0);
            } else {
                let _ = writeln!(f, "{}{} {}", config.join(), config.line().repeat(config.depth()), leaf.0);
            }

            // recurse
            if !leaf.1.is_empty() {
                clone.push(last);
                let _ = Self::display_leaves(f, &leaf.1, clone, config);
            }
        }
        write!(f, "")
    }
}

pub trait TreeConfig {
    fn space(&self) -> &str {" "}
    fn line(&self)  -> &str {"─"}
    fn last(&self)  -> &str {"└"}
    fn join(&self)  -> &str {"├"}
    fn bar(&self)   -> &str {"|"}
    fn depth(&self) -> usize {2}
}

pub struct DefaultTreeConfig;
impl TreeConfig for DefaultTreeConfig {}

impl<D: Display> Display for Tree<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "{}", self.0);
        Self::display_leaves(f, &self.1, Vec::new(), &DefaultTreeConfig)
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;
    #[test]
    fn render_tree_root() {
        let tree = Tree::root("foo");
        assert_eq!(format!("{}", tree), "foo\n")
    }

    #[test]
    fn render_tree_with_leaves() {
        let tree = Tree::new(
            "foo", vec![
               Tree::new(
                   "bar", vec![
                    Tree::root("baz")
                   ]
               )
            ]
        );
        assert_eq!(format!("{}", tree), r#"foo
└── bar
    └── baz
"#)
    }
}
