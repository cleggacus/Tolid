extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result}, parse_macro_input, ItemFn,
};

struct Attribute {
    name: syn::Ident,
    value: AttributeValue,
}

struct Element {
    name: syn::Ident,
    attrs: Vec<Attribute>,
    children: Vec<Element>,
}

enum AttributeValue {
    Literal(syn::Lit),
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
        let name = input.parse::<syn::Ident>()?;
        input.parse::<syn::token::Eq>()?;

        let value: AttributeValue = input.parse()?;

        Ok(Attribute {
            name,
            value,
        })
    }
}


impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::token::Lt>()?;
        let name: syn::Ident = input.parse()?;

        //attributes
        
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
                children: vec![]
            });
        }

        input.parse::<syn::token::Gt>()?;

        // chilren

        let mut children: Vec<Element> = vec![];

        while !(input.peek(syn::token::Lt) && input.peek2(syn::token::Slash)) {
            let child: Element = input.parse()?;
            children.push(child);
        }

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

        // Generate tokens for attributes
        let mut fields: Vec<proc_macro2::TokenStream> = self.attrs.iter().map(|attr| {
            let key = &attr.name;
            match &attr.value {
                AttributeValue::Literal(lit) => {
                    if self.name == "Text" && key == "value" {
                        quote! { value: tolid::component::ComponentValue::Static(#lit) }
                    } else {
                        quote! { #key: #lit }
                    }
                },
                AttributeValue::Expr(expr) => {
                    // Only auto-wrap if component is Text and field is on_click
                    if self.name == "Text" && key == "on_click" {
                        quote! { on_click: Some(Box::new(#expr)) }
                    } else if self.name == "Text" && key == "value" {
                        match expr {
                            syn::Expr::Closure(_) => {
                                quote! { value: tolid::component::ComponentValue::Dynamic(Box::new(#expr)) }
                            }
                            _ => {
                                quote! { value: tolid::component::ComponentValue::Static(#expr) }
                            }
                        }
                    } else {
                        quote! { #key: (#expr) }
                    }
                }
            }
        }).collect();

        // Generate tokens for children
        let child_exprs = self.children.iter().map(|child| {
            let child_tokens = child.generate_tokens();
            quote! {
                Box::new(#child_tokens)
            }
        });

        if !self.children.is_empty() {
            fields.push(
                quote! {
                    children: vec![
                        #(#child_exprs),*
                    ]
                }
            );
        }

        quote! {
            #name(#props_name {
                #(#fields,)*
                ..Default::default()
            })
        }
    }
}

#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as Element);
    let output = parsed.generate_tokens();
    eprintln!("{}", output);
    TokenStream::from(output)
}




#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.sig.ident.clone();
    let block = input_fn.block;
    let vis = input_fn.vis.clone();

    let inputs = &input_fn.sig.inputs;

    // Otherwise generate props struct and rewrite fn to take props param

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
        struct #props_struct_name {
            #(#props_fields),*
        }

        #vis fn #fn_name(props: #props_struct_name) -> impl Component {
            let #props_struct_name { #(#param_idents),* } = props;

            #block
        }
    };

    output.into()
}
