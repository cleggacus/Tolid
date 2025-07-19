extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result}, parse_macro_input, ItemFn,
};

struct Attribute {
    prefix: Option<syn::Ident>,
    name: syn::Ident,
    value: AttributeValue,
}

struct Element {
    name: syn::Ident,
    attrs: Vec<Attribute>,
    children: Children,
}

enum AttributeValue {
    Literal(syn::Lit),
    Expr(syn::Expr),
}

enum Children {
    ElementList(Vec<Element>),
    Expr(syn::Expr),
}

impl Parse for AttributeValue {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::Lit) {
            let lit: syn::Lit = input.parse()?;
            Ok(AttributeValue::Literal(lit))
        } else if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            let expr = content.parse::<syn::Expr>()?;
            Ok(AttributeValue::Expr(expr))
        } else {
            Err(input.error("expected literal or `{ expr }`"))
        }
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name = input.parse::<syn::Ident>()?;

        let prefix = if input.peek(syn::token::Colon) {
            let prefix = name;

            input.parse::<syn::token::Colon>()?;
            name = input.parse::<syn::Ident>()?;

            Some(prefix)
        } else {
            None
        };

        input.parse::<syn::token::Eq>()?;

        let value: AttributeValue = input.parse()?;

        Ok(Attribute {
            prefix,
            name,
            value,
        })
    }
}


impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::token::Lt>()?;
        let name: syn::Ident = input.parse()?;

        let mut attrs: Vec<Attribute> = vec![];

        while input.peek(syn::Ident) {
            let attr: Attribute = input.parse()?;
            attrs.push(attr);
        }

        if input.peek(syn::token::Slash) {
            input.parse::<syn::token::Slash>()?;
            input.parse::<syn::token::Gt>()?;

            return Ok(Element { 
                name, 
                attrs, 
                children: Children::ElementList(vec![])
            });
        }

        input.parse::<syn::token::Gt>()?;

        let children = if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            let expr = content.parse::<syn::Expr>()?;

            Children::Expr(expr)
        } else {
            let mut children: Vec<Element> = vec![];

            while !(input.peek(syn::token::Lt) && input.peek2(syn::token::Slash)) {
                let child: Element = input.parse()?;
                children.push(child);
            }

            Children::ElementList(children)
        };

        input.parse::<syn::token::Lt>()?;
        input.parse::<syn::token::Slash>()?;
        let closing_name: syn::Ident = input.parse()?;
        input.parse::<syn::token::Gt>()?;

        if name != closing_name {
            return Err(input.error(format!(
                "expected </{}> closing tag",
                name
            )));
        }

        Ok(Element { 
            name, 
            attrs, 
            children
        })
    }
}

impl Element {
    fn generate_tokens(&self) -> proc_macro2::TokenStream {
        let name = &self.name;

        let props_name = syn::Ident::new(&format!("{}Props", name), name.span());

        let mut fields: Vec<proc_macro2::TokenStream> = self.attrs.iter().map(|attr| {
            let key = &attr.name;
            let prefix = &attr.prefix;

            match &attr.value {
                AttributeValue::Literal(lit) => {
                    if self.name == "Text" && key == "value" {
                        quote! { value: (#lit).into_component_value() }
                    } else {
                        quote! { #key: #lit }
                    }
                },
                AttributeValue::Expr(expr) => {
                    if let Some(prefix) = prefix {
                        if prefix == "on" {
                            let combined = format_ident!("{}_{}", prefix, key);
                            return quote! { #combined: Some(Box::new(#expr)) };
                        }
                    }

                    if self.name == "Text" && key == "value" {
                        quote! { value: (#expr).into_component_value() }
                    } else {
                        quote! { #key: (#expr) }
                    }
                }
            }
        }).collect();

        match &self.children {
            Children::ElementList(elements) => {
                let child_exprs = elements.iter().map(|child| {
                    let child_tokens = child.generate_tokens();
                    quote! {
                        Box::new(#child_tokens)
                    }
                });

                if !elements.is_empty() {
                    fields.push(
                        quote! {
                            children: vec![
                                #(#child_exprs),*
                            ]
                        }
                    );
                }
            },
            Children::Expr(expr) => {
                fields.push(
                    quote! {
                        children: #expr
                    }
                );
            },
        }

        quote! {
            #name(
                ctx.clone(),
                #props_name {
                    #(#fields,)*
                    ..Default::default()
                }
            )
        }
    }
}

#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as Element);
    let output = parsed.generate_tokens();

    let output_string = output.to_string();
    eprintln!("Generated code:\n{}", output_string);

    TokenStream::from(output)
}




#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.sig.ident.clone();
    let block = input_fn.block;
    let vis = input_fn.vis.clone();

    let inputs = &input_fn.sig.inputs;

    let props_struct_name = format_ident!("{}Props", fn_name);

    let mut props_fields = Vec::new();
    let mut param_idents = Vec::new();

    for input in inputs.iter() {
        if let syn::FnArg::Typed(arg) = input {
            if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = &*arg.pat {
                let ty = &arg.ty;
                props_fields.push(quote! { pub #ident: #ty });
                param_idents.push(ident.clone());
            } else {
                return syn::Error::new_spanned(&arg.pat, "Expected ident pattern")
                    .to_compile_error()
                    .into();
            }
        } else {
            return syn::Error::new_spanned(input, "Unexpected receiver (self) in component function")
                .to_compile_error()
                .into();
        }
    }

    let output = quote! {
        #[derive(Default)]
        #vis struct #props_struct_name {
            #(#props_fields),*
        }

        #[allow(non_snake_case)]
        #vis fn #fn_name(ctx: StateContext, props: #props_struct_name) -> impl Component {
            let #props_struct_name { #(#param_idents),* } = props;

            #block
        }
    };

    output.into()
}
