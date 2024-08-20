#![feature(unsized_const_params)]

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
	fn idx(self) -> Self::Output {
		<I as KeyPathIndexable<T>>::idx(self)
	}
}

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
	pub const fn new() -> Self {
		Self {
			_phantom: PhantomData
		}
	}

	pub const fn kp<const NAME: &'static str>(self) -> Aggregator<(KeyPath<NAME>, T)> {
		Aggregator::new()
	}

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
	fn default() -> Self {
		Self::new()
	}
}

impl<T> Clone for Aggregator<T>
where
	T: ?Sized
{
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Copy for Aggregator<T> where T: ?Sized {}

#[derive(Copy, Clone, Default)]
pub struct KeyPath<const NAME: &'static str>;

impl<const NAME: &'static str> KeyPath<NAME> {
	pub const fn kp<const N2: &'static str>(self) -> Aggregator<(KeyPath<N2>, Self)> {
		Aggregator::new()
	}

	pub const fn idx<const N: usize>(self) -> Aggregator<(UsizeKeyPath<N>, Self)> {
		Aggregator::new()
	}
}

#[derive(Copy, Clone, Default)]
pub struct UsizeKeyPath<const N: usize>;

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
	fn idx(self) -> Self::Output {
		&self[N]
	}
}

impl<'t, const N: usize, T> KeyPathIndexable<UsizeKeyPath<N>> for &'t mut T
where
	T: IndexMut<usize>
{
	type Output = &'t mut <T as Index<usize>>::Output;
	fn idx(self) -> Self::Output {
		&mut self[N]
	}
}

impl<const N: usize, T> KeyPathIndexable<UsizeKeyPath<N>> for Vec<T> {
	type Output = T;
	fn idx(mut self) -> Self::Output {
		self.remove(N)
	}
}

pub trait MapKeyPath: Iterator {
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

// util thing
pub trait TypeEquals<T> {
	fn to_type(self) -> T;
}

impl<T> TypeEquals<T> for T {
	fn to_type(self) -> T {
		self
	}
}
