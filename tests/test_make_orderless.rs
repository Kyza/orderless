use orderless::make_orderless;

#[make_orderless(defs(a = false, b = false))]
fn two<T>(a: T, b: T) -> (T, T) {
	(a, b)
}

#[test]
fn none() {
	two(true, true);
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
