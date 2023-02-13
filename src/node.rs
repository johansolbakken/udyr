use std::rc::Rc;

use crate::token;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Program,
    Expression,
    Binary,
    Unary,
    Grouping,
    Operator,
    Literal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub(crate) node_type: NodeType,
    pub(crate) children: Vec<Rc<Node>>,
    pub(crate) token: token::Token,
}

impl Node {
    /// Creates a new [`Node`].
    pub fn new(node_type: NodeType, children: &[Rc<Node>]) -> Node {
        let mut node = Node {
            node_type,
            children: Vec::new(),
            token: token::Token::empty(),
        };

        for child in children {
            node.children.push(child.clone())
        }

        return node;
    }

    pub fn print(&self) {
        self.print_aux(0);
    }

    fn print_aux(&self, indent: usize) {
        println!(
            "{}{:?} {:?}",
            " ".repeat(indent),
            self.node_type,
            self.token
        );
        for child in &self.children {
            child.print_aux(indent + 2);
        }
    }
}
