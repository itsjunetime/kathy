#![feature(unsized_const_params)]

use core::{
	marker::PhantomData,
	ops::{Index, IndexMut}
};

pub use kathy_macros::Keyable;

impl<T1, T2, I> RefKeyPathIndexable<(T1, T2)> for I
where
	// theoretically it would be nice to add `T1: ?Sized` as well but that violates rust's rule
	// about 'only the last element of a tuple may be unsized' and realistically this will only be
	// used with KeyPath anyways, which is always zero-sized, so whatever.
	T2: ?Sized,
	I: RefKeyPathIndexable<T2> + ?Sized,
	<I as RefKeyPathIndexable<T2>>::Type: RefKeyPathIndexable<T1> + 'static
{
	type Type = <<I as RefKeyPathIndexable<T2>>::Type as RefKeyPathIndexable<T1>>::Type;
	fn idx(&self) -> &Self::Type {
		<<I as RefKeyPathIndexable<T2>>::Type as RefKeyPathIndexable<T1>>::idx(
			<I as RefKeyPathIndexable<T2>>::idx(self)
		)
	}
}

impl<T1, T2, I> MutKeyPathIndexable<(T1, T2)> for I
where
	T2: ?Sized,
	I: MutKeyPathIndexable<T2> + ?Sized,
	<I as RefKeyPathIndexable<T2>>::Type:
		RefKeyPathIndexable<T1> + MutKeyPathIndexable<T1> + 'static
{
	fn idx_mut(&mut self) -> &mut Self::Type {
		<<I as RefKeyPathIndexable<T2>>::Type as MutKeyPathIndexable<T1>>::idx_mut(
			<I as MutKeyPathIndexable<T2>>::idx_mut(self)
		)
	}
}

impl<T1, T2, I> MovingKeyPathIndexable<(T1, T2)> for I
where
	I: MovingKeyPathIndexable<T2>,
	<I as RefKeyPathIndexable<T2>>::Type: MovingKeyPathIndexable<T1> + Sized + 'static,
	<<I as RefKeyPathIndexable<T2>>::Type as RefKeyPathIndexable<T1>>::Type: Sized
{
	fn idx_move(self) -> Self::Type {
		<<I as RefKeyPathIndexable<T2>>::Type as MovingKeyPathIndexable<T1>>::idx_move(
			<I as MovingKeyPathIndexable<T2>>::idx_move(self)
		)
	}
}

impl<T, I> RefKeyPathIndexable<Aggregator<T>> for I
where
	I: RefKeyPathIndexable<T> + ?Sized
{
	type Type = <I as RefKeyPathIndexable<T>>::Type;
	fn idx(&self) -> &Self::Type {
		<I as RefKeyPathIndexable<T>>::idx(self)
	}
}

impl<T, I> MutKeyPathIndexable<Aggregator<T>> for I
where
	I: MutKeyPathIndexable<T> + ?Sized
{
	fn idx_mut(&mut self) -> &mut Self::Type {
		<I as MutKeyPathIndexable<T>>::idx_mut(self)
	}
}

impl<T, I> MovingKeyPathIndexable<Aggregator<T>> for I
where
	I: MovingKeyPathIndexable<T>,
	<I as RefKeyPathIndexable<T>>::Type: Sized
{
	fn idx_move(self) -> Self::Type {
		<I as MovingKeyPathIndexable<T>>::idx_move(self)
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

pub trait RefKeyPathIndexable<T>
where
	T: ?Sized
{
	type Type: ?Sized;
	fn idx(&self) -> &Self::Type;
}

pub trait MutKeyPathIndexable<T>: RefKeyPathIndexable<T>
where
	T: ?Sized
{
	fn idx_mut(&mut self) -> &mut Self::Type;
}

pub trait MovingKeyPathIndexable<T>: MutKeyPathIndexable<T>
where
	T: ?Sized,
	Self::Type: Sized
{
	fn idx_move(self) -> Self::Type;
}

// hmmmm. how do we do this without running into the weird infinitely recursive trait resolution
// stuff.
/*impl<I, T> MovingKeyPathIndexable<I> for &mut T
where
	T: MutKeyPathIndexable<I>
{
	fn idx_move(self) -> Self::Type {
		self.idx_mut()
	}
}

impl<I, T> MovingKeyPathIndexable<I> for &T
where
	T: RefKeyPathIndexable<I>
{
	fn idx_move(self) -> Self::Type {
		self.idx()
	}
}*/

impl<const N: usize, T> RefKeyPathIndexable<UsizeKeyPath<N>> for T
where
	T: Index<usize>
{
	type Type = <T as Index<usize>>::Output;
	fn idx(&self) -> &Self::Type {
		&self[N]
	}
}

impl<const N: usize, T> MutKeyPathIndexable<UsizeKeyPath<N>> for T
where
	T: IndexMut<usize>
{
	fn idx_mut(&mut self) -> &mut Self::Type {
		&mut self[N]
	}
}

impl<const N: usize, T> MovingKeyPathIndexable<UsizeKeyPath<N>> for Vec<T> {
	fn idx_move(mut self) -> Self::Type {
		self.remove(N)
	}
}

pub trait MapKeyPath: Iterator {
	fn map_kp<KP>(
		self,
		_kp: KP
	) -> core::iter::Map<
		Self,
		impl FnMut(Self::Item) -> <Self::Item as RefKeyPathIndexable<KP>>::Type
	>
	where
		Self::Item: MovingKeyPathIndexable<KP>,
		<Self::Item as RefKeyPathIndexable<KP>>::Type: Sized,
		Self: Sized
	{
		self.map(|item| <Self::Item as MovingKeyPathIndexable<KP>>::idx_move(item))
	}
}

impl<T> MapKeyPath for T where T: Iterator {}

pub trait RefMapKeyPath<'item, T>: Iterator<Item = &'item T>
where
	T: 'item
{
	fn map_kp_ref<KP>(
		self,
		_kp: KP
	) -> core::iter::Map<Self, impl FnMut(&'item T) -> &<T as RefKeyPathIndexable<KP>>::Type>
	where
		T: RefKeyPathIndexable<KP>,
		<T as RefKeyPathIndexable<KP>>::Type: 'item,
		Self: Sized
	{
		self.map(|item| <T as RefKeyPathIndexable<KP>>::idx(item))
	}
}

impl<'item, I, T> RefMapKeyPath<'item, I> for T
where
	T: Iterator<Item = &'item I>,
	I: 'item
{
}

pub trait MutMapKeyPath<'item, T>: Iterator<Item = &'item mut T>
where
	T: 'item
{
	fn map_kp_mut<KP>(
		self,
		_kp: KP
	) -> core::iter::Map<Self, impl FnMut(&'item mut T) -> &mut <T as RefKeyPathIndexable<KP>>::Type>
	where
		T: MutKeyPathIndexable<KP>,
		<T as RefKeyPathIndexable<KP>>::Type: 'item,
		Self: Sized
	{
		self.map(|item| <T as MutKeyPathIndexable<KP>>::idx_mut(item))
	}
}

impl<'item, I, T> MutMapKeyPath<'item, I> for T
where
	T: Iterator<Item = &'item mut I>,
	I: 'item
{
}
