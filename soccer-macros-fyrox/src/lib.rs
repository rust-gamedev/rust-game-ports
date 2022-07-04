use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self,
    parse::{self, Parser},
    Data, DataStruct, DeriveInput, Fields,
};

type TokenStream2 = proc_macro2::TokenStream;

macro_rules! bail {
    ( $msg:expr $(,)? ) => {
        return ::syn::Result::<_>::Err(::syn::Error::new(::proc_macro2::Span::call_site(), &$msg))
    };

    ( $msg:expr => $spanned:expr $(,)? ) => {
        return ::syn::Result::<_>::Err(::syn::Error::new_spanned(&$spanned, &$msg))
    };
}

// To be updated with the suggestions from the question.
//
#[proc_macro_attribute]
pub fn my_actor_based(args: TokenStream, input: TokenStream) -> TokenStream {
    let my_actor_base_impl = impl_my_actor_based(args, input);

    my_actor_base_impl
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn impl_my_actor_based(
    args: TokenStream,
    input: TokenStream,
) -> Result<TokenStream2, ::syn::Error> {
    let mut ast: DeriveInput = ::syn::parse2(input.into())?;
    let _: parse::Nothing = ::syn::parse2(args.into())?;

    add_fields(&mut ast)?;

    let trait_impl = impl_trait(&ast)?;

    let gen = quote! {
        #ast

        #trait_impl
    };

    Ok(gen.into())
}

fn add_fields(ast: &'_ mut DeriveInput) -> Result<(), ::syn::Error> {
    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &mut ast.data
    {
        let fields_tokens = vec![
            quote! { pub img_base: &'static str },
            quote! { pub img_indexes: Vec<u8> },
            quote! { pub vpos: Vector2<f32> },
            quote! { anchor: Anchor },
            quote! { rectangle_h: Handle<Node> },
        ];

        for field_tokens in fields_tokens {
            let field = syn::Field::parse_named.parse2(field_tokens).unwrap();
            fields.named.push(field);
        }

        Ok(())
    } else {
        bail!("Unexpected input (missing curly braces?)")
    }
}

fn impl_trait(ast: &'_ DeriveInput) -> Result<TokenStream2, ::syn::Error> {
    #[allow(non_snake_case)]
    let TyName = &ast.ident;
    let (intro_generics, forward_generics, maybe_where_clause) = ast.generics.split_for_impl();

    Ok(quote!(
        impl #intro_generics
            crate::my_actor::MyActor
        for
            #TyName #forward_generics
        #maybe_where_clause
        {
            fn vpos(&self) -> Vector2<f32> {
                self.vpos
            }

            fn vpos_mut(&mut self) -> &mut Vector2<f32> {
                &mut self.vpos
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

            fn rectangle_h(&self) -> Handle<Node> {
                self.rectangle_h
            }
        }
    ))
}
