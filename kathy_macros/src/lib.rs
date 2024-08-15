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
			quote! {
				const #f_ident: ::kathy::KeyPath<#s> = ::kathy::KeyPath;
			}
		}).collect();

	let input_name = input.ident;

	let impl_idx: TokenStream = fields_with_strs.iter()
		.map(|(s, f)| {
			let f_ident = &f.ident;
			let f_ty = &f.ty;

			quote!{
				impl ::kathy::RefKeyPathIndexable<::kathy::KeyPath<#s>> for #input_name {
					type Type = #f_ty;
					fn idx(&self) -> &Self::Type {
						&self.#f_ident
					}
				}

				impl ::kathy::MutKeyPathIndexable<::kathy::KeyPath<#s>> for #input_name {
					fn idx_mut(&mut self) -> &mut Self::Type {
						&mut self.#f_ident
					}
				}

				impl ::kathy::MovingKeyPathIndexable<::kathy::KeyPath<#s>> for #input_name
				where
					<Self as ::kathy::RefKeyPathIndexable<::kathy::KeyPath<#s>>>::Type: ::core::marker::Sized
				{
					fn idx_move(self) -> Self::Type {
						self.#f_ident
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

		impl<T> ::core::ops::Index<T> for #input_name
		where
			#input_name: ::kathy::RefKeyPathIndexable<T>
		{
			type Output = <#input_name as ::kathy::RefKeyPathIndexable<T>>::Type;
			fn index(&self, _: T) -> &Self::Output {
				<Self as ::kathy::RefKeyPathIndexable<T>>::idx(self)
			}
		}

		impl<T> ::core::ops::IndexMut<T> for #input_name
		where
			#input_name: ::kathy::MutKeyPathIndexable<T>
		{
			fn index_mut(&mut self, _: T) -> &mut Self::Output {
				<Self as ::kathy::MutKeyPathIndexable<T>>::idx_mut(self)
			}
		}
	}.into()
}
