use fervid_core::{ElementNode, VueImports};
use swc_core::ecma::ast::{Expr, Ident};

use crate::CodegenContext;

impl CodegenContext {
    pub fn generate_transition_group(&mut self, element_node: &ElementNode) -> Expr {
        let span = element_node.span;

        // _TransitionGroup
        let transition_group_identifier = Expr::Ident(Ident {
            span,
            sym: self.get_and_add_import_ident(VueImports::TransitionGroup),
            optional: false,
        });

        let transition_group_attrs =
            self.generate_builtin_attrs(&element_node.starting_tag.attributes, span);

        let transition_group_slots = self.generate_builtin_slots(element_node);

        self.generate_componentlike(
            transition_group_identifier,
            transition_group_attrs,
            transition_group_slots,
            &element_node.patch_hints,
            false,
            span,
        )
    }
}

#[cfg(test)]
mod tests {
    use fervid_core::{AttributeOrBinding, BuiltinType, ElementKind, Node, StartingTag};
    use swc_core::common::DUMMY_SP;

    use crate::test_utils::js;

    use super::*;

    #[test]
    fn it_generates_empty_transition_group() {
        // <transition-group></transition-group>
        test_out(
            ElementNode {
                kind: ElementKind::Builtin(BuiltinType::TransitionGroup),
                starting_tag: StartingTag {
                    tag_name: "transition-group".into(),
                    attributes: vec![],
                    directives: None,
                },
                children: vec![],
                template_scope: 0,
                patch_hints: Default::default(),
                span: DUMMY_SP,
            },
            r#"_createVNode(_TransitionGroup)"#,
        )
    }

    #[test]
    fn it_generates_transition_group_attrs() {
        // <transition-group foo="bar" :baz="qux"></transition-group>
        test_out(
            ElementNode {
                kind: ElementKind::Builtin(BuiltinType::TransitionGroup),
                starting_tag: StartingTag {
                    tag_name: "transition-group".into(),
                    attributes: vec![
                        AttributeOrBinding::RegularAttribute {
                            name: "foo".into(),
                            value: "bar".into(),
                        },
                        AttributeOrBinding::VBind(fervid_core::VBindDirective {
                            argument: Some("baz".into()),
                            value: js("qux"),
                            is_camel: false,
                            is_prop: false,
                            is_attr: false,
                        }),
                    ],
                    directives: None,
                },
                children: vec![],
                template_scope: 0,
                patch_hints: Default::default(),
                span: DUMMY_SP,
            },
            r#"_createVNode(_TransitionGroup,{foo:"bar",baz:qux})"#,
        )
    }

    #[test]
    fn it_generates_transition_group_children() {
        // <transition-group>foobar</transition-group>
        test_out(
            ElementNode {
                kind: ElementKind::Builtin(BuiltinType::TransitionGroup),
                starting_tag: StartingTag {
                    tag_name: "transition-group".into(),
                    attributes: vec![],
                    directives: None,
                },
                children: vec![Node::Text("foobar".into(), DUMMY_SP)],
                template_scope: 0,
                patch_hints: Default::default(),
                span: DUMMY_SP,
            },
            r#"_createVNode(_TransitionGroup,null,{"default":_withCtx(()=>[_createTextVNode("foobar")]),_:1})"#,
        )
    }

    #[test]
    fn it_generates_full_transition_group() {
        // <transition-group foo="bar" :baz="qux">foobar</transition-group>
        test_out(
            ElementNode {
                kind: ElementKind::Builtin(BuiltinType::TransitionGroup),
                starting_tag: StartingTag {
                    tag_name: "transition-group".into(),
                    attributes: vec![
                        AttributeOrBinding::RegularAttribute {
                            name: "foo".into(),
                            value: "bar".into(),
                        },
                        AttributeOrBinding::VBind(fervid_core::VBindDirective {
                            argument: Some("baz".into()),
                            value: js("qux"),
                            is_camel: false,
                            is_prop: false,
                            is_attr: false,
                        }),
                    ],
                    directives: None,
                },
                children: vec![Node::Text("foobar".into(), DUMMY_SP)],
                template_scope: 0,
                patch_hints: Default::default(),
                span: DUMMY_SP,
            },
            r#"_createVNode(_TransitionGroup,{foo:"bar",baz:qux},{"default":_withCtx(()=>[_createTextVNode("foobar")]),_:1})"#,
        )
    }

    fn test_out(input: ElementNode, expected: &str) {
        let mut ctx = CodegenContext::default();
        let out = ctx.generate_transition_group(&input);
        assert_eq!(crate::test_utils::to_str(out), expected)
    }
}
