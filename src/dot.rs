use dot_graph::Graph;

pub trait DotBuilder {
    fn graph(&self, id: &mut u32, graph: &mut Graph) -> Option<String>;
}
