use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Keyable)]
pub fn derive_keyable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);
	let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) = input.data else {
		return quote! { compile_error!("The Keyable macro can only be used on structs with named fields") }.into()
	};

	let fields_with_strs = fields.named
		.into_iter()
		// we can unwrap here 'cause we already verified up above that they're named fields, not
		// unnamed
		.map(|f| (f.ident.as_ref().unwrap().to_string(), f))
		.collect::<Vec<_>>();

	let impl_consts: TokenStream = fields_with_strs.iter()
		.map(|(s, f)| {
			let f_ident = &f.ident;
			let f_ty = &f.ty;
			quote! {
				const #f_ident: ::kathy::KeyPath<#s, #f_ty> = ::kathy::KeyPath::new();
			}
		}).collect();

	let input_name = input.ident;

	let impl_idx: TokenStream = fields_with_strs.iter()
		.map(|(s, f)| {
			let f_ident = &f.ident;
			let f_ty = &f.ty;

			quote!{
				impl ::kathy::KeyPathIndexable<::kathy::KeyPath<#s, #f_ty>> for #input_name {
					type Type = #f_ty;
					fn idx(&self) -> &Self::Type {
						&self.#f_ident
					}
					fn idx_mut(&mut self) -> &mut Self::Type {
						&mut self.#f_ident
					}
				}
			}
		}).collect();


	quote!{
		#[allow(non_upper_case_globals)]
		impl #input_name {
			#impl_consts
		}

		#impl_idx

		impl<T> ::std::ops::Index<T> for #input_name
		where
			#input_name: ::kathy::KeyPathIndexable<T>
		{
			type Output = <#input_name as ::kathy::KeyPathIndexable<T>>::Type;
			fn index(&self, _: T) -> &Self::Output {
				<Self as ::kathy::KeyPathIndexable<T>>::idx(self)
			}
		}

		impl<T> ::std::ops::IndexMut<T> for #input_name
		where
			#input_name: ::kathy::KeyPathIndexable<T>
		{
			fn index_mut(&mut self, _: T) -> &mut Self::Output {
				<Self as ::kathy::KeyPathIndexable<T>>::idx_mut(self)
			}
		}
	}.into()
}
