pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::{edge::Edge, node::Node};

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }
        pub fn with_attrs(mut self, arg: &[(&str, &str)]) -> Self {
            arg.iter().for_each(|(a, b)| {
                self.attrs.insert(String::from(*a), String::from(*b));
            });
            self
        }

        pub fn with_nodes(mut self, vec: &[Node]) -> Self {
            self.nodes = vec.to_vec();
            self
        }

        pub fn get_node(&self, s: &str) -> Option<&Node> {
            self.nodes
                .iter()
                .find(|x| -> bool { x.to_owned().name == *s })
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
    }

    pub mod graph_items {

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {
                        from: String::from(a),
                        to: String::from(b),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, arg: &[(&str, &str)]) -> Self {
                    arg.iter().for_each(|(a, b)| {
                        self.attrs.insert(String::from(*a), String::from(*b));
                    });
                    self
                }
            }

            impl PartialEq for Edge {
                fn eq(&self, other: &Self) -> bool {
                    self.from == other.from && self.to == other.to && self.attrs == other.attrs
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(a: &str) -> Self {
                    Node {
                        name: String::from(a),
                        attrs: HashMap::new(),
                    }
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(&String::from(key)).map(|a| a.as_str())
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Node {
                    attrs.iter().for_each(|(a, b)| {
                        self.attrs.insert(String::from(*a), String::from(*b));
                    });
                    self
                }
            }

            impl PartialEq for Node {
                fn eq(&self, other: &Self) -> bool {
                    self.name == other.name && self.attrs == other.attrs
                }
            }
        }
    }
}

mod testerer {
    use maplit::hashmap;

    use crate::graph::{
        graph_items::{edge::Edge, node::Node},
        Graph,
    };

    #[test]
    #[ignore]
    fn test_graph_with_attributes() {
        let nodes = vec![
            Node::new("a").with_attrs(&[("color", "green")]),
            Node::new("c"),
            Node::new("b").with_attrs(&[("label", "Beta!")]),
        ];

        let edges = vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue")]),
        ];

        let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

        let expected_attrs = hashmap! {
            "foo".to_string() => "1".to_string(),
            "title".to_string() => "Testing Attrs".to_string(),
            "bar".to_string() => "true".to_string(),
        };

        let graph = Graph::new()
            .with_nodes(&nodes)
            .with_edges(&edges)
            .with_attrs(&attrs);

        assert_eq!(
            graph.nodes,
            vec![
                Node::new("a").with_attrs(&[("color", "green")]),
                Node::new("c"),
                Node::new("b").with_attrs(&[("label", "Beta!")]),
            ]
        );

        assert_eq!(
            graph.edges,
            vec![
                Edge::new("b", "c"),
                Edge::new("a", "b").with_attrs(&[("color", "blue")]),
            ]
        );

        assert_eq!(graph.attrs, expected_attrs);
    }
}
