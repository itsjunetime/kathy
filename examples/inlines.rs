use kathy::Keyable;

#[derive(Keyable)]
struct Person {
	age: u16
}

fn main() {
	get_age(Person { age: 46 });
}

#[inline(never)]
fn get_age(person: Person) {
	std::hint::black_box(person[Person::age]);
}
