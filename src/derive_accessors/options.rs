use macroific::prelude::*;
use proc_macro2::Ident;
use syn::{Expr, Visibility};

#[derive(AttributeOptions)]
#[cfg_attr(feature = "_debug", derive(Debug))]
pub struct ContainerOptions {
    pub get: bool,
    pub get_mut: bool,
    pub set: bool,

    pub defaults: ContainerDefaults,
}

#[derive(AttributeOptions)]
#[cfg_attr(feature = "_debug", derive(Debug))]
pub struct FieldOptions {
    pub skip: bool,
    pub all: Option<VariationOptions>,
    pub get: Option<VariationOptions>,
    pub get_mut: Option<VariationOptions>,
    pub set: Option<VariationOptions>,
}

#[derive(ParseOption, Default)]
#[cfg_attr(feature = "_debug", derive(Debug))]
pub struct ContainerDefaults {
    pub all: VariationDefaults,
    pub get: VariationDefaults,
    pub get_mut: VariationDefaults,
    pub set: VariationDefaults,
}

#[derive(ParseOption, Clone)]
#[cfg_attr(feature = "_debug", derive(Debug))]
pub struct VariationOptions {
    pub owned: Option<bool>,
    pub const_fn: Option<bool>,
    pub skip: Option<bool>,
    pub cp: Option<bool>,
    pub ty: Option<syn::Type>,
    pub prefix: Option<Ident>,
    pub suffix: Option<Ident>,
    pub vis: Option<Visibility>,
}

#[derive(ParseOption, Default)]
#[cfg_attr(feature = "_debug", derive(Debug))]
pub struct VariationDefaults {
    pub owned: Option<bool>,
    pub const_fn: Option<bool>,
    pub cp: Option<bool>,
    pub prefix: Option<Ident>,
    pub suffix: Option<Ident>,
    pub vis: Option<Visibility>,
}

impl FromExpr for VariationDefaults {
    fn from_expr(expr: Expr) -> syn::Result<Self> {
        Err(syn::Error::new_spanned(
            expr,
            "VariationDefaults can't be constructed this way",
        ))
    }
}

impl From<&VariationDefaults> for VariationOptions {
    fn from(defaults: &VariationDefaults) -> Self {
        Self {
            owned: defaults.owned,
            const_fn: defaults.const_fn,
            skip: None,
            cp: defaults.cp,
            ty: None,
            prefix: defaults.prefix.clone(),
            suffix: defaults.suffix.clone(),
            vis: defaults.vis.clone(),
        }
    }
}

macro_rules! assign_defaults {
    (cp $self: ident $from: ident => $($prop: ident),+) => {
        $(
            if $self.$prop.is_none() {
                if let Some(default_val) = $from.$prop {
                    $self.$prop = Some(default_val);
                }
            }
        )+
    };
    (clone $self: ident $from: ident => $($prop: ident),+) => {
        $(
            if $self.$prop.is_none() {
                if let Some(ref default_val) = $from.$prop {
                    $self.$prop = Some(default_val.clone());
                }
            }
        )+
    };
}

impl VariationOptions {
    pub fn assign_defaults_from_struct(&mut self, defaults: &VariationDefaults) {
        assign_defaults!(cp self defaults => owned, const_fn, cp);
        assign_defaults!(clone self defaults => prefix, suffix, vis);
    }

    pub fn assign_defaults_from_prop_all(&mut self, defaults: &Option<Self>) {
        if let Some(defaults) = defaults {
            assign_defaults!(cp self defaults => owned, const_fn, cp);
            assign_defaults!(clone self defaults => prefix, suffix, vis);
        }
    }
}
