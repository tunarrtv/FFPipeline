extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Expr, Token};
use syn::parse::Parse;

struct MacroArgs {
    exprs: Punctuated<Expr, Token![,]>
}

// We implement the `syn::parse::Parse` trait for our custom `MacroArgs` struct.
impl Parse for MacroArgs {
    // The `parse` function is where we define how to handle the incoming token stream.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // We use `Punctuated::parse_separated_nonempty` to parse one or more
        // expressions separated by commas.
        let exprs = Punctuated::parse_separated_nonempty(input)?;
        Ok(MacroArgs { exprs })
    }
}

#[proc_macro]
pub fn make_pixel_format(input: TokenStream) -> TokenStream {
    let exprs = parse_macro_input!(input as MacroArgs);

    if exprs.exprs.len() != 2 {
        // Diagnostic::span
        // Return an empty TokenStream to prevent further compilation errors from malformed output
        return TokenStream::new();
    }

    let name = exprs.exprs.get(0);
    let bit_depth = exprs.exprs.get(1);

    quote! {
        StaticPixelFormat {
            name: #name.to_string(),
            ffmpeg_name: #name.to_string(),
            bit_depth: #bit_depth,
        }
    }.into()
}

#[proc_macro_derive(PixelFormat)]
pub fn pix_fmt_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let output = quote! {
        impl PixelFormat for #name  {
            fn name(&self) -> &str {
                return self.name.as_str();
            }
            fn ffmpeg_name(&self) -> &str {
                return self.ffmpeg_name.as_str();
            }

            fn bit_depth(&self) -> u8 {
                return 0
            }
        }
    };

    output.into()
}
