// --8<-- [start:imports]
use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::DeriveInput;
use syn::Error;
use syn::Fields;
use syn::parse_macro_input;
// --8<-- [end:imports]

// --8<-- [start:reject]
fn reject(model: &DeriveInput, message: &str) -> TokenStream {
    Error::new(model.ident.span(), message)
        .into_compile_error()
        .into()
}
// --8<-- [end:reject]

// --8<-- [start:derive-entry]
#[proc_macro_derive(ReadingValue)]
pub fn derive_reading(input: TokenStream) -> TokenStream {
    let model = parse_macro_input!(input as DeriveInput);

    if !model.generics.params.is_empty() {
        return reject(&model, "ReadingValue 不支援泛型參數");
    }

    let data = match &model.data {
        Data::Struct(data) => data,
        _ => return reject(&model, "ReadingValue 需要 struct 宣告"),
    };

    match &data.fields {
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() != 1 {
                return reject(&model, "ReadingValue 需要恰好一個 tuple 欄位");
            }
        }
        _ => return reject(&model, "ReadingValue 需要 tuple struct"),
    }

    // --8<-- [start:generate]
    let name = &model.ident;
    let generated = quote! {
        impl ::ch14_macros::Reading for #name {
            fn reading(&self) -> i32 {
                self.0
            }
        }
    };
    generated.into()
    // --8<-- [end:generate]
}
// --8<-- [end:derive-entry]
