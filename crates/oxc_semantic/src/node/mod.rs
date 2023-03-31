#![allow(non_upper_case_globals)] // for bitflags

mod id;
mod tree;

use bitflags::bitflags;
use oxc_ast::AstKind;

pub use self::{id::AstNodeId, tree::AstNodes};
use crate::scope::{Scope, ScopeId};

/// Indextree node containing a semantic node
pub type AstNode<'a> = indextree::Node<SemanticNode<'a>>;

/// Semantic node contains all the semantic information about an ast node.
#[derive(Debug, Clone, Copy)]
pub struct SemanticNode<'a> {
    /// A pointer to the ast node, which resides in the `bumpalo` memory arena.
    kind: AstKind<'a>,

    /// Associated Scope (initialized by binding)
    scope_id: ScopeId,

    flags: NodeFlags,

    /// All AST Nodes whose index falls in the range of [node_index, last_child_index]
    /// is a descendant of current Node.
    node_index: u32,
    last_child_index: u32,
}

bitflags! {
    #[derive(Default)]
    pub struct NodeFlags: u8 {
        const Class = 1 << 0; // If Node is inside a class
    }
}

impl<'a> SemanticNode<'a> {
    #[must_use]
    pub fn new(kind: AstKind<'a>, scope_id: ScopeId, flags: NodeFlags, node_index: u32) -> Self {
        // Initially my only child is myself
        Self { kind, scope_id, flags, node_index, last_child_index: node_index }
    }

    pub(crate) fn last_child_index(&mut self, last_child_index: u32) {
        self.last_child_index = last_child_index;
    }

    #[must_use]
    pub fn kind(&self) -> AstKind<'a> {
        self.kind
    }

    #[must_use]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id
    }

    #[must_use]
    pub fn strict_mode(&self, scope: &Scope) -> bool {
        // All parts of a ClassDeclaration or a ClassExpression are strict mode code.
        scope.strict_mode() || self.in_class()
    }

    #[must_use]
    pub fn in_class(self) -> bool {
        self.flags.contains(NodeFlags::Class)
    }

    /// Checks if self is the descendent of parent.
    /// The operation is a cheap comparison of index.
    #[must_use]
    pub fn is_descendant(&self, parent: &Self) -> bool {
        (parent.node_index..=parent.last_child_index).contains(&self.node_index)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use indextree::Node;
    use oxc_allocator::Allocator;
    use oxc_ast::SourceType;
    use oxc_parser::Parser;

    use super::*;
    use crate::{Semantic, SemanticBuilder};

    fn check_node_descendant<'a>(node: &Node<SemanticNode<'a>>, semantic: &Semantic<'a>) {
        let node_id = semantic.nodes().get_node_id(node).unwrap();
        let descendants = node_id.descendants(semantic.nodes());
        for desc in descendants {
            let desc_node = &semantic.nodes()[desc];
            assert!(SemanticNode::is_descendant(desc_node.get(), node.get()));
        }
    }

    #[test]
    fn test_is_descendant() {
        let source = "
      class A {
        foo() {
          if (b) {
            return c;
          } else {
            for (let i = 0; i < 10; i += 1) {
              console.log(i);
            }
            return b;
          }
        }
      }

      function foo() {
        const c = 1;
      }
    ";

        let allocator: Allocator = Allocator::default();
        let parse = Parser::new(&allocator, source, SourceType::default()).parse();
        assert!(parse.errors.is_empty());

        let program = allocator.alloc(parse.program);

        let semantic_builder =
            SemanticBuilder::new(source, SourceType::default(), &Rc::new(parse.trivias));
        let semantic = semantic_builder.build(program);

        assert!(semantic.errors.is_empty());

        let semantic = semantic.semantic;

        for node in semantic.nodes().iter() {
            check_node_descendant(node, &semantic);
        }
    }
}
