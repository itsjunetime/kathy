#![feature(unsized_const_params)]

use core::marker::PhantomData;

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
	pub const fn kp<const NAME: &'static str, T2>(self) -> Aggregator<(KeyPath<NAME, T2>, T)>
	where
		T2: ?Sized
	{
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

pub struct KeyPath<const NAME: &'static str, T>
where
	T: ?Sized
{
	_phantom: PhantomData<T>
}

impl<const NAME: &'static str, T> KeyPath<NAME, T>
where
	T: ?Sized
{
	pub const fn new() -> Self {
		Self {
			_phantom: PhantomData
		}
	}
	pub const fn kp<const N2: &'static str, T2>(self) -> Aggregator<(KeyPath<N2, T2>, Self)>
	where
		T2: ?Sized
	{
		Aggregator::new()
	}
}

impl<const NAME: &'static str, T> Default for KeyPath<NAME, T>
where
	T: ?Sized
{
	fn default() -> Self {
		Self {
			_phantom: PhantomData
		}
	}
}

impl<const NAME: &'static str, T> Clone for KeyPath<NAME, T>
where
	T: ?Sized
{
	fn clone(&self) -> Self {
		*self
	}
}

impl<const NAME: &'static str, T> Copy for KeyPath<NAME, T> where T: ?Sized {}

pub trait KeyPathIndexable<T>
where
	T: ?Sized
{
	type Type: ?Sized;
	fn idx(&self) -> &Self::Type;
	fn idx_mut(&mut self) -> &mut Self::Type;
}
