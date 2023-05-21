use orderless::impl_orderless;

struct Args {}

#[impl_orderless]
impl Args {
	#[make_orderless(defs(a = false, b = false))]
	pub fn two(a: bool, b: bool) -> (bool, bool) {
		(a, b)
	}
}

#[test]
fn none() {
	assert_eq!(Args__two!(), (false, false));
}

#[test]
fn just_a() {
	assert_eq!(Args__two!(a = true), (true, false));
}

#[test]
fn just_b() {
	let b: bool = true;
	assert_eq!(Args__two!(b), (false, true));
}

#[test]
fn a_and_b() {
	let b: bool = true;
	assert_eq!(Args__two!(b, a = true), (true, true));
}
