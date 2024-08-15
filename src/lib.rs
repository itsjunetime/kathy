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
	I: KeyPathIndexable<T2> + ?Sized,
	<I as KeyPathIndexable<T2>>::Type: KeyPathIndexable<T1> + 'static
{
	type Type = <<I as KeyPathIndexable<T2>>::Type as KeyPathIndexable<T1>>::Type;
	fn idx(&self) -> &Self::Type {
		self.idx().idx()
	}
	fn idx_mut(&mut self) -> &mut Self::Type {
		self.idx_mut().idx_mut()
	}
}

impl<T, I> KeyPathIndexable<Aggregator<T>> for I
where
	I: KeyPathIndexable<T> + ?Sized
{
	type Type = <I as KeyPathIndexable<T>>::Type;
	fn idx(&self) -> &Self::Type {
		<I as KeyPathIndexable<T>>::idx(self)
	}
	fn idx_mut(&mut self) -> &mut Self::Type {
		<I as KeyPathIndexable<T>>::idx_mut(self)
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
	type Type: ?Sized;
	fn idx(&self) -> &Self::Type;
	fn idx_mut(&mut self) -> &mut Self::Type;
}

impl<const N: usize, T> KeyPathIndexable<UsizeKeyPath<N>> for T
where
	T: Index<usize> + IndexMut<usize>
{
	type Type = <T as Index<usize>>::Output;
	fn idx(&self) -> &Self::Type {
		&self[N]
	}
	fn idx_mut(&mut self) -> &mut Self::Type {
		&mut self[N]
	}
}
