use parse_format::{ParseMode, Parser, Piece, Position};
use proc_macro2::{Ident, Span};
use proc_macro::TokenStream;
use quote::quote;
use std::rc::Rc;
use syn::{parse::Parse, parse_macro_input, ExprPath, LitStr, Token};
use unindent::unindent;

struct MacroInput {
    name: Ident,
    prompt: String,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: ExprPath = input.parse()?;
        input.parse::<Token![,]>()?;
        let prompt: LitStr = input.parse()?;

        let segments = name.path.segments;
        assert!(segments.len() == 1, "Function name is required");

        Ok(Self {
            name: segments.first().unwrap().ident.clone(),
            prompt: unindent(prompt.value().trim()),
        })
    }
}

#[proc_macro]
pub fn prompt(input: TokenStream) -> TokenStream {
    let input = TokenStream::from(input);
    let MacroInput { name, prompt } = parse_macro_input!(input as MacroInput);

    let parser = Parser::new(&prompt, None, None, false, ParseMode::Format);
    let args = parser
        .filter_map(|piece| match piece {
            Piece::NextArgument(arg) => Some(arg),
            Piece::String(_) => None,
        })
        .collect::<Rc<_>>();

    if args
        .iter()
        .any(|arg| !matches!(arg.position, Position::ArgumentNamed(_)))
    {
        panic!("Only named arguments are supported, e.g. {{name}}");
    }

    let args = args
        .iter()
        .map(|arg| {
            let Position::ArgumentNamed(name) = arg.position else {
                unreachable!()
            };

            Ident::new(name, Span::call_site())
        })
        .collect::<Vec<_>>();

    let tokens = quote! {
        pub fn #name(#(#args: &str),*) -> String {
            ::std::format!(#prompt, #(#args = #args),*)
        }
    };

    tokens.into()
} 