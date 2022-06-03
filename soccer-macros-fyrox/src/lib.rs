use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self,
    parse::{self, Parser},
    parse_macro_input, Data, DataStruct, DeriveInput, Fields,
};

// To be updated with the suggestions from the question.
//
#[proc_macro_attribute]
pub fn my_actor_based(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = syn::parse(input).unwrap();
    let _ = parse_macro_input!(args as parse::Nothing);

    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &mut ast.data
    {
        let fields_tokens = vec![
            quote! { img_base: &'static str },
            quote! { img_indexes: Vec<u8> },
            quote! { pub vpos: Vector2<i16> },
            quote! { anchor: Anchor },
        ];

        for field_tokens in fields_tokens {
            fields
                .named
                .push(syn::Field::parse_named.parse2(field_tokens).unwrap());
        }
    } else {
        panic!("Unexpected input (missing curly braces?)");
    }

    let name = &ast.ident;

    let gen = quote! {
        #ast

        impl crate::my_actor::MyActor for #name {
            fn vpos(&self) -> Vector2<i16> {
                self.vpos
            }

            fn img_base(&self) -> &'static str {
                self.img_base
            }

            fn img_indexes(&self) -> &[u8] {
                &self.img_indexes
            }

            fn anchor(&self) -> Anchor {
                self.anchor
            }
        }
    };

    gen.into()
}
