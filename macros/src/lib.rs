extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(PixelFormat)]
pub fn pix_fmt_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let output = quote! {
        impl PixelFormat for #name  {
            fn get_name(&self) -> &str {
                return self.name.as_str();
            }
            fn get_ffmpeg_name(&self) -> &str {
                return self.ffmpeg_name.as_str();
            }

            fn get_bit_depth(&self) -> u8 {
                return 0
            }
        }
    };

    output.into()
}
