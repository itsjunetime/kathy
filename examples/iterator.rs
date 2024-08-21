use kathy::{Keyable, MapKeyPath};

#[derive(Keyable)]
struct Vec2 {
	height: u16,
	width: u16
}

#[derive(Keyable)]
struct Person {
	name: &'static str,
	dimensions: Vec2
}

fn main() {
	let mut people = [
		Person {
			name: "Kathy",
			dimensions: Vec2 {
				height: 100,
				width: 20
			}
		},
		Person {
			name: "Karen",
			dimensions: Vec2 {
				height: 120,
				width: 40
			}
		},
		Person {
			name: "Kaley",
			dimensions: Vec2 {
				height: 140,
				width: 60
			}
		}
	];

	people
		.iter()
		.map_kp(Person::dimensions.kp::<"height">())
		.for_each(|height| println!("height: {height}"));

	people
		.iter_mut()
		.map_kp(Person::dimensions.kp::<"width">())
		.map(|width| *width * 2)
		.for_each(|width| println!("width: {width}"));

	people
		.into_iter()
		.map_kp(Person::name)
		.for_each(|name| println!("name: {name}"));
}
