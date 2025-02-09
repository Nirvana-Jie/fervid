use fxhash::FxHashMap as HashMap;
use smallvec::SmallVec;
use swc_core::ecma::ast::{Expr, Id, Ident};

use crate::{BuiltinType, FervidAtom, BindingTypes, TemplateGenerationMode, VueImportsSet};

#[derive(Debug, Default)]
pub struct BindingsHelper {
    /// All components present in the `<template>`
    pub components: HashMap<FervidAtom, ComponentBinding>,
    /// All custom directives present in the `<template>`
    pub custom_directives: HashMap<FervidAtom, CustomDirectiveBinding>,
    /// Are we compiling for DEV or PROD
    pub is_prod: bool,
    /// Scopes of the `<template>` for in-template variable resolutions
    pub template_scopes: Vec<TemplateScope>,
    /// Bindings in `<script setup>`
    pub setup_bindings: Vec<SetupBinding>,
    /// Bindings in `<script>`
    pub options_api_bindings: Option<Box<OptionsApiBindings>>,
    /// The mode with which `<template>` variables are resolved.
    /// Also controls in which mode should the template be generated:
    /// - inline as last statement of `setup` or
    /// - as a `render` function.
    pub template_generation_mode: TemplateGenerationMode,
    /// Identifiers used in the template and their respective binding types
    pub used_bindings: HashMap<FervidAtom, BindingTypes>,
    /// Internal Vue imports used by built-in components, directives and others
    pub vue_imports: VueImportsSet
}

#[derive(Debug, Default)]
pub enum ComponentBinding {
    /// Component was resolved to something specific, e.g. an import.
    /// The contained `Expr` is for the resolved value (usually identifier or `unref(ident)`)
    Resolved(Box<Expr>),

    /// Component must be resolved in runtime, i.e. using `resolveComponent` call.
    /// The contained value is an identifier,
    /// e.g. `_component_custom` in `const _component_custom = resolveComponent('custom')`
    RuntimeResolved(Box<Ident>),

    /// Component was not resolved and would need to be resolved in runtime
    #[default]
    Unresolved,

    /// Component was resolved to be a Vue built-in
    Builtin(BuiltinType)
}

#[derive(Debug, Default)]
pub enum CustomDirectiveBinding {
    /// Custom directive was resolved,
    /// usually to an identifier which has a form `vCustomDirective` (corresponds to `v-custom-directive`).
    Resolved(Box<Expr>),

    /// Custom directive must be resolved in runtime, i.e. using `resolveDirective` call.
    /// The contained value is an identifier,
    /// e.g. `_directive_custom` in `const _directive_custom = resolveDirective('custom')`
    RuntimeResolved(Box<Ident>),

    /// Custom directive was not resolved and would need to be resolved in runtime
    #[default]
    Unresolved
}

// Todo maybe use SmallVec?
#[derive(Debug, Default, PartialEq)]
pub struct OptionsApiBindings {
    pub data: Vec<FervidAtom>,
    pub setup: Vec<SetupBinding>,
    pub props: Vec<FervidAtom>,
    pub inject: Vec<FervidAtom>,
    pub emits: Vec<FervidAtom>,
    pub components: Vec<FervidAtom>,
    pub computed: Vec<FervidAtom>,
    pub methods: Vec<FervidAtom>,
    pub expose: Vec<FervidAtom>,
    pub name: Option<FervidAtom>,
    pub directives: Vec<FervidAtom>,
    pub imports: Vec<Id>
}

#[derive(Debug, PartialEq)]
pub struct SetupBinding(pub FervidAtom, pub BindingTypes);

#[derive(Debug)]
pub struct TemplateScope {
    pub variables: SmallVec<[FervidAtom; 1]>,
    pub parent: u32,
}
