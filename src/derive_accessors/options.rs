use macroific::prelude::*;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    Error, Expr, GenericParam, LitStr, PredicateLifetime, PredicateType, Token, TypePath,
    Visibility, WherePredicate,
};

#[derive(AttributeOptions)]
#[cfg_attr(feature = "_debug", derive(Debug))]
pub struct ContainerOptions {
    pub get: bool,
    pub get_mut: bool,
    pub set: bool,

    pub defaults: ContainerDefaults,
    pub bounds: Punctuated<WhereParam, Token![,]>,
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
    pub prefix: Option<SkippableIdent>,
    pub suffix: Option<SkippableIdent>,
    pub vis: Option<Visibility>,
    pub bounds: Punctuated<WhereParam, Token![,]>,
}

#[derive(ParseOption, Default)]
#[cfg_attr(feature = "_debug", derive(Debug))]
pub struct VariationDefaults {
    pub owned: Option<bool>,
    pub const_fn: Option<bool>,
    pub cp: Option<bool>,
    pub prefix: Option<SkippableIdent>,
    pub suffix: Option<SkippableIdent>,
    pub vis: Option<Visibility>,
    pub bounds: Punctuated<WhereParam, Token![,]>,
}

impl FromExpr for VariationDefaults {
    fn from_expr(expr: Expr) -> syn::Result<Self> {
        Err(Error::new_spanned(
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
            bounds: defaults.bounds.clone(),
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
        self.apply_default_bounds(&defaults.bounds);
    }

    pub fn assign_defaults_from_prop_all(&mut self, defaults: &Option<Self>) {
        if let Some(defaults) = defaults {
            assign_defaults!(cp self defaults => owned, const_fn, cp);
            assign_defaults!(clone self defaults => prefix, suffix, vis);
            self.apply_default_bounds(&defaults.bounds);
        }
    }

    fn apply_default_bounds(&mut self, default_bounds: &Punctuated<WhereParam, Token![,]>) {
        if self.bounds.is_empty() && !default_bounds.is_empty() {
            self.bounds = default_bounds.clone();
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "_debug", derive(Debug))]
pub enum SkippableIdent {
    Ident(Ident),
    Skip,
}

impl FromExpr for SkippableIdent {
    fn from_expr(expr: Expr) -> syn::Result<Self> {
        Ok(Self::Ident(Ident::from_expr(expr)?))
    }
}

impl Parse for SkippableIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Lit) {
            let lit: LitStr = input.parse()?;
            if lit.value().is_empty() {
                Ok(Self::Skip)
            } else {
                Err(Error::new_spanned(lit, "Expected empty string"))
            }
        } else {
            Ok(Self::Ident(input.parse()?))
        }
    }
}

const _: () = {
    use std::fmt::{Display, Formatter, Result, Write};
    #[derive(Copy, Clone)]
    struct Renderer<'a> {
        ident: &'a SkippableIdent,
        is_prefix: bool,
    }

    impl Display for Renderer<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            const CHAR: char = '_';

            match self.ident {
                SkippableIdent::Skip => Ok(()),
                SkippableIdent::Ident(ident) => {
                    if self.is_prefix {
                        Display::fmt(ident, f)?;
                        f.write_char(CHAR)
                    } else {
                        f.write_char(CHAR)?;
                        Display::fmt(ident, f)
                    }
                }
            }
        }
    }

    impl<'a> SkippableIdent {
        #[inline]
        pub fn as_suffix(&'a self) -> impl Display + Copy + Clone + 'a {
            Renderer {
                ident: self,
                is_prefix: false,
            }
        }

        #[inline]
        pub fn as_prefix(&'a self) -> impl Display + Copy + Clone + 'a {
            Renderer {
                ident: self,
                is_prefix: true,
            }
        }
    }
};

impl ParseOption for SkippableIdent {
    #[inline]
    fn from_stream(input: ParseStream) -> syn::Result<Self> {
        input.parse()
    }
}

#[derive(ParseOption, Clone)]
#[cfg_attr(feature = "_debug", derive(Debug))]
#[attr_opts(from_parse)]
pub struct WhereParam {
    p: WherePredicate,
}

impl TryFrom<GenericParam> for WhereParam {
    type Error = Error;

    fn try_from(param: GenericParam) -> syn::Result<Self> {
        Ok(Self {
            p: match param {
                GenericParam::Type(param) => WherePredicate::Type(PredicateType {
                    lifetimes: None,
                    bounded_ty: syn::Type::Path(TypePath {
                        qself: None,
                        path: param.ident.into(),
                    }),
                    colon_token: param.colon_token.unwrap_or_default(),
                    bounds: param.bounds,
                }),
                GenericParam::Lifetime(param) => WherePredicate::Lifetime(PredicateLifetime {
                    lifetime: param.lifetime,
                    colon_token: param.colon_token.unwrap_or_default(),
                    bounds: param.bounds,
                }),
                GenericParam::Const(param) => {
                    return Err(Error::new_spanned(param, "Const params not supported"));
                }
            },
        })
    }
}

impl Parse for WhereParam {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<GenericParam>()?.try_into()
    }
}

impl FromExpr for WhereParam {
    fn from_expr(expr: Expr) -> syn::Result<Self> {
        GenericParam::from_expr(expr)?.try_into()
    }
}

impl From<WhereParam> for WherePredicate {
    #[inline]
    fn from(value: WhereParam) -> Self {
        value.p
    }
}

impl ToTokens for WhereParam {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.p.to_tokens(tokens);
    }
}
