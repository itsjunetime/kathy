use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

/// Please see the main page of `kathy`'s docs to learn how to use this.
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
				impl<'a> ::kathy::KeyPathIndexable<::kathy::KeyPath<#s>> for &'a #input_name {
					type Output = &'a #f_ty;
					fn idx(self) -> Self::Output {
						&self.#f_ident
					}
				}

				impl<'a> ::kathy::KeyPathIndexable<::kathy::KeyPath<#s>> for &'a mut #input_name {
					type Output = &'a mut #f_ty;
					fn idx(self) -> Self::Output {
						&mut self.#f_ident
					}
				}

				impl ::kathy::KeyPathIndexable<::kathy::KeyPath<#s>> for #input_name {
					type Output = #f_ty;
					fn idx(self) -> Self::Output {
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
			for<'a> &'a #input_name: ::kathy::KeyPathIndexable<T>,
			#input_name: ::kathy::KeyPathIndexable<T>,
			for<'a> <&'a #input_name as ::kathy::KeyPathIndexable<T>>::Output:
				::kathy::TypeEquals<&'a <#input_name as ::kathy::KeyPathIndexable<T>>::Output>,
		{
			type Output = <#input_name as ::kathy::KeyPathIndexable<T>>::Output;
			fn index(&self, _: T) -> &Self::Output {
				use ::kathy::TypeEquals as _;
				<&Self as ::kathy::KeyPathIndexable<T>>::idx(self)
					.to_type()
			}
		}

		impl<T> ::core::ops::IndexMut<T> for #input_name
		where
			// need to include the requirements from Index
			for<'a> &'a #input_name: ::kathy::KeyPathIndexable<T>,
			#input_name: ::kathy::KeyPathIndexable<T>,
			for<'a> <&'a #input_name as ::kathy::KeyPathIndexable<T>>::Output:
				::kathy::TypeEquals<&'a <#input_name as ::kathy::KeyPathIndexable<T>>::Output>,
			// and then here are the requirements specific to IndexMut
			for<'a> &'a  mut #input_name: ::kathy::KeyPathIndexable<T>,
			for<'a> <&'a mut #input_name as ::kathy::KeyPathIndexable<T>>::Output:
				::kathy::TypeEquals<&'a mut <#input_name as ::kathy::KeyPathIndexable<T>>::Output>,
		{
			fn index_mut(&mut self, _: T) -> &mut Self::Output {
				use ::kathy::TypeEquals as _;
				<&mut Self as ::kathy::KeyPathIndexable<T>>::idx(self)
					.to_type()
			}
		}
	}.into()
}
