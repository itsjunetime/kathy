use std::ops::IndexMut;

use kathy::{KeyPathIndexable, Keyable};

#[derive(Debug, Keyable)]
struct Family {
	people: Vec<Person>
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
	let family = Family {
		people: vec![person]
	};
	real_main(family);
}

#[inline(never)]
fn real_main(mut family: Family) {
	let height_agg = Vec2::height;
	println!("height: {}", family.people[0].dimensions[height_agg]);

	let height_agg = Person::dimensions.kp::<"height">();
	println!("height: {}", family.people[0][height_agg]);

	let height_agg = Family::people
		.idx::<0>()
		.kp::<"dimensions">()
		.kp::<"height">();
	println!("height: {}", family[height_agg]);

	modify(
		&mut family.people[0],
		Person::dimensions.kp::<"height">(),
		5
	);
	println!("family: {family:?}");
}

#[inline(never)]
fn modify<KP, I>(thing: &mut Person, path: KP, new_val: I)
where
	Person: IndexMut<KP, Output = I>
{
	thing[path] = new_val;
}
