use orderless::create_orderless;

fn two<T>(a: T, b: T) -> (T, T) {
	(a, b)
}

create_orderless! {
	func = two,
	order(a, b),
	defs(a = false, b = false)
}

#[test]
fn none() {
	assert_eq!(two!(), (false, false));
}

#[test]
fn just_a() {
	assert_eq!(two!(a = true), (true, false));
}

#[test]
fn just_b() {
	let b: bool = true;
	assert_eq!(two!(b), (false, true));
}

#[test]
fn a_and_b() {
	let b: bool = true;
	assert_eq!(two!(b, a = true), (true, true));
}
