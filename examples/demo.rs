use kathy::{KeyPathIndexable, Keyable};

#[derive(Debug, Keyable)]
struct Family {
	mom: Person
}

#[derive(Debug, Keyable)]
struct Person {
	age: usize,
	name: String,
	dimensions: Vec2
}

#[derive(Debug, Keyable)]
struct Vec2 {
	height: u16,
	width: u16
}

fn main() {
	let person = Person {
		age: 10,
		name: "Joe".to_string(),
		dimensions: Vec2 {
			height: 20,
			width: 4
		}
	};
	let family = Family { mom: person };
	real_main(family);
}

#[inline(never)]
fn real_main(mut family: Family) {
	let height_agg = Vec2::height;
	println!("height: {}", family.mom.dimensions[height_agg]);

	let height_agg = Person::dimensions.kp::<"height", _>();
	println!("height: {}", family.mom[height_agg]);

	let height_agg = Family::mom.kp::<"dimensions", _>().kp::<"height", _>();
	println!("height: {}", family[height_agg]);

	modify(&mut family.mom, Person::dimensions.kp::<"height", _>(), 5);
	println!("family: {family:?}");
}

fn modify<T, KP, I>(thing: &mut T, _path: KP, new_val: I)
where
	T: KeyPathIndexable<KP, Type = I>
{
	*thing.idx_mut() = new_val;
}
