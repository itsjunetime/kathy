#![feature(unsized_const_params)]
#![doc = include_str!("../README.md")]

use core::{
	marker::PhantomData,
	ops::{Index, IndexMut}
};

pub use kathy_macros::Keyable;

impl<T1, T2, I> KeyPathIndexable<(T1, T2)> for I
where
	// theoretically it would be nice to add `T1: ?Sized` as well but that violates rust's rule
	// about 'only the last element of a tuple may be unsized' and realistically this will only be
	// used with KeyPath anyways, which is always zero-sized, so whatever.
	T2: ?Sized,
	I: KeyPathIndexable<T2>,
	<I as KeyPathIndexable<T2>>::Output: KeyPathIndexable<T1>
{
	type Output = <<I as KeyPathIndexable<T2>>::Output as KeyPathIndexable<T1>>::Output;
	#[inline(always)]
	fn idx(self) -> Self::Output {
		<<I as KeyPathIndexable<T2>>::Output as KeyPathIndexable<T1>>::idx(
			<I as KeyPathIndexable<T2>>::idx(self)
		)
	}
}

impl<T, I> KeyPathIndexable<Aggregator<T>> for I
where
	I: KeyPathIndexable<T>,
	<I as KeyPathIndexable<T>>::Output: Sized
{
	type Output = <I as KeyPathIndexable<T>>::Output;
	#[inline(always)]
	fn idx(self) -> Self::Output {
		<I as KeyPathIndexable<T>>::idx(self)
	}
}

/// A convenience struct that is used to store nested keypaths, normally as increasingly nested
/// tuples. For example, a triply-nested keypath would be represented as an `Aggregator<(T1, (T2,
/// T3))>`, where `T3` is evaluated on the top-most item, then `T2`, then `T1.
pub struct Aggregator<T>
where
	T: ?Sized
{
	_phantom: PhantomData<T>
}

impl<T> Aggregator<T>
where
	T: ?Sized
{
	/// Convenience function to create a new instance of [`Self`]. This is used instead of
	/// [`Default::default`] because it must be const.
	#[inline(always)]
	pub const fn new() -> Self {
		Self {
			_phantom: PhantomData
		}
	}

	/// Extend the KeyPath represented by [`self`] one layer deeper to index a field named `NAME`
	#[inline(always)]
	pub const fn kp<const NAME: &'static str>(self) -> Aggregator<(KeyPath<NAME>, T)> {
		Aggregator::new()
	}

	/// Extend the KeyPath represented by [`self`] one layer deeper to index an item at index `N`
	#[inline(always)]
	pub const fn idx<const N: usize>(self) -> Aggregator<(UsizeKeyPath<N>, T)> {
		Aggregator::new()
	}
}

// unfortunately, we need to write these impls ourselves since they should be implemented
// regardless of what T is.
impl<T> Default for Aggregator<T>
where
	T: ?Sized
{
	#[inline(always)]
	fn default() -> Self {
		Self::new()
	}
}

impl<T> Clone for Aggregator<T>
where
	T: ?Sized
{
	#[inline(always)]
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Copy for Aggregator<T> where T: ?Sized {}

/// A struct which stores a single field-based keypath. When nesting keypaths, many of these are
/// stored in an [`Aggregator`], in the order and structure defined by that struct's docs.
#[derive(Copy, Clone, Default)]
pub struct KeyPath<const NAME: &'static str>;

impl<const NAME: &'static str> KeyPath<NAME> {
	#[inline(always)]
	pub const fn kp<const N2: &'static str>(self) -> Aggregator<(KeyPath<N2>, Self)> {
		Aggregator::new()
	}

	#[inline(always)]
	pub const fn idx<const N: usize>(self) -> Aggregator<(UsizeKeyPath<N>, Self)> {
		Aggregator::new()
	}
}

/// The same as [`KeyPath`], but for index-based keypaths as opposed to field-based.
#[derive(Copy, Clone, Default)]
pub struct UsizeKeyPath<const N: usize>;

/// The trait that powers indexing via key-path. There are not separate traits for move-by-index,
/// mut-ref-by-index, and ref-by-index. Instead, `KeyPathIndexable` is implemented for `T`, `&T`,
/// and `&mut T` separately.
pub trait KeyPathIndexable<T>
where
	T: ?Sized
{
	type Output;
	fn idx(self) -> Self::Output;
}

impl<'t, const N: usize, T> KeyPathIndexable<UsizeKeyPath<N>> for &'t T
where
	T: Index<usize>
{
	type Output = &'t <T as Index<usize>>::Output;
	#[inline(always)]
	fn idx(self) -> Self::Output {
		&self[N]
	}
}

impl<'t, const N: usize, T> KeyPathIndexable<UsizeKeyPath<N>> for &'t mut T
where
	T: IndexMut<usize>
{
	type Output = &'t mut <T as Index<usize>>::Output;
	#[inline(always)]
	fn idx(self) -> Self::Output {
		&mut self[N]
	}
}

/// A special-cased implementation for move-by-indexing with a Vec, as there is no standard trait
/// to implement such a thing (since [`Index`] and [`IndexMut`] only allow ref-by-index and
/// mut-ref-by-index, respectively.
#[cfg(feature = "alloc")]
impl<const N: usize, T> KeyPathIndexable<UsizeKeyPath<N>> for Vec<T> {
	type Output = T;
	#[inline(always)]
	fn idx(mut self) -> Self::Output {
		self.remove(N)
	}
}

/// A convenience trait for Iterators
pub trait MapKeyPath: Iterator {
	/// A convenience method for mapping an iterator, transforming each object inside it into the
	/// object referenced by `_kp`. This allows more elegant API creation, allowing field-based
	/// indexing via 0-cost abstractions, instead of by passing closures around (which closures then
	/// normally access and return a nested object).
	fn map_kp<KP>(
		self,
		_kp: KP
	) -> core::iter::Map<Self, impl FnMut(Self::Item) -> <Self::Item as KeyPathIndexable<KP>>::Output>
	where
		Self::Item: KeyPathIndexable<KP>,
		<Self::Item as KeyPathIndexable<KP>>::Output: Sized,
		Self: Sized
	{
		self.map(|item| <Self::Item as KeyPathIndexable<KP>>::idx(item))
	}
}

impl<T> MapKeyPath for T where T: Iterator {}

/// This was stolen from the [`type_equals` crate](https://crates.io/crates/type-equals) and then
/// extended as described in [my article on const generic matrices](https://itsjuneti.me/post/1).
/// It is used in the type signature for `Index` and `IndexMut` as generated by the `Keyable`
/// macro.
pub trait TypeEquals<T> {
	fn to_type(self) -> T;
}

impl<T> TypeEquals<T> for T {
	fn to_type(self) -> T {
		self
	}
}
