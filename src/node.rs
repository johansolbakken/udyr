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
    node_type: NodeType,
    children: Vec<Rc<Node>>,
    token: token::Token,
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
}
