#![feature(unsized_const_params)]

use std::marker::PhantomData;

pub use kathy_macros::Keyable;

impl<T1, T2, I> KeyPathIndexable<(T1, T2)> for I
where
	I: KeyPathIndexable<T1>,
	<I as KeyPathIndexable<T1>>::Type: KeyPathIndexable<T2> + 'static
{
	type Type = <<I as KeyPathIndexable<T1>>::Type as KeyPathIndexable<T2>>::Type;
	fn idx(&self) -> &Self::Type {
		self.idx().idx()
	}
	fn idx_mut(&mut self) -> &mut Self::Type {
		self.idx_mut().idx_mut()
	}
}

impl<T, I> KeyPathIndexable<Aggregator<T>> for I
where
	I: KeyPathIndexable<T>
{
	type Type = <I as KeyPathIndexable<T>>::Type;
	fn idx(&self) -> &Self::Type {
		<I as KeyPathIndexable<T>>::idx(self)
	}
	fn idx_mut(&mut self) -> &mut Self::Type {
		<I as KeyPathIndexable<T>>::idx_mut(self)
	}
}

pub struct Aggregator<T> {
	_phantom: PhantomData<T>
}

impl<T> Aggregator<T> {
	pub const fn new() -> Self {
		Self {
			_phantom: PhantomData
		}
	}
	pub const fn kp<const NAME: &'static str, T2>(self) -> Aggregator<(T, KeyPath<NAME, T2>)> {
		Aggregator::new()
	}
}

// unfortunately, we need to write these impls ourselves since they should be implemented
// regardless of what T is.
impl<T> Default for Aggregator<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> Clone for Aggregator<T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Copy for Aggregator<T> {}

pub struct KeyPath<const NAME: &'static str, T> {
	_phantom: PhantomData<T>
}

impl<const NAME: &'static str, T> KeyPath<NAME, T> {
	pub const fn new() -> Self {
		Self {
			_phantom: PhantomData
		}
	}
	pub const fn kp<const N2: &'static str, T2>(self) -> Aggregator<(Self, KeyPath<N2, T2>)> {
		Aggregator::new()
	}
}

impl<const NAME: &'static str, T> Default for KeyPath<NAME, T> {
	fn default() -> Self {
		Self {
			_phantom: PhantomData
		}
	}
}

impl<const NAME: &'static str, T> Clone for KeyPath<NAME, T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<const NAME: &'static str, T> Copy for KeyPath<NAME, T> {}

pub trait KeyPathIndexable<T> {
	type Type;
	fn idx(&self) -> &Self::Type;
	fn idx_mut(&mut self) -> &mut Self::Type;
}
