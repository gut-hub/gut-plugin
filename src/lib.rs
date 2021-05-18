use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, ExprArray, Lit, Result, Token};

struct PluginInput {
    names: ExprArray,
    descriptions: ExprArray,
}

impl Parse for PluginInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let names: ExprArray = input.parse()?;
        input.parse::<Token![,]>()?;
        let descriptions: ExprArray = input.parse()?;

        let out = Self {
            names,
            descriptions,
        };
        Ok(out)
    }
}

#[proc_macro]
pub fn gut_export(input: TokenStream) -> TokenStream {
    // parse macro input
    let data = parse_macro_input!(input as PluginInput);

    // convert input into vector of strings
    let names: Vec<String> = data
        .names
        .elems
        .into_iter()
        .map(|x| match x {
            Expr::Lit(expr_lit) => match expr_lit.lit {
                Lit::Str(lit_str) => lit_str.value(),
                _ => "".to_string(),
            },
            _ => "".to_string(),
        })
        .collect();
    let descriptions: Vec<String> = data
        .descriptions
        .elems
        .into_iter()
        .map(|x| match x {
            Expr::Lit(expr_lit) => match expr_lit.lit {
                Lit::Str(lit_str) => lit_str.value(),
                _ => "".to_string(),
            },
            _ => "".to_string(),
        })
        .collect();

    // convert to JSON string
    let json_names = serde_json::to_string(&names).expect("Failed to convert to JSON string");
    let json_descriptions =
        serde_json::to_string(&descriptions).expect("Failed to convert to JSON string");

    // create rust code with qoute
    let exports = quote! {
        use std::ffi::CString;
        use std::os::raw::c_char;

        #[no_mangle]
        pub extern "C" fn gut_export_functions() -> *mut c_char {
            let c_string = CString::new(#json_names).expect("Failed to create c_string");
            c_string.into_raw()
        }

        #[no_mangle]
        pub extern "C" fn gut_export_descriptions() -> *mut c_char {
            let c_string = CString::new(#json_descriptions).expect("Failed to create c_string");
            c_string.into_raw()
        }
    };

    exports.into()
}
