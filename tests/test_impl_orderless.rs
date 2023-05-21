use orderless::impl_orderless;

struct Args {
	c: bool,
}

#[impl_orderless]
impl Args {
	#[make_orderless(defs(self = {
		Args {
			c: false
		}
	}, a = false, b = false))]
	pub fn three(self, a: bool, b: bool) -> (bool, bool, bool) {
		(a, b, self.c)
	}
}

#[test]
fn none() {
	assert_eq!(Args__three!(), (false, false, false));
}

#[test]
fn just_a() {
	assert_eq!(Args__three!(a = true), (true, false, false));
}

#[test]
fn just_b() {
	let b: bool = true;
	assert_eq!(Args__three!(b), (false, true, false));
}

#[test]
fn a_and_b() {
	let b: bool = true;
	assert_eq!(Args__three!(b, a = true), (true, true, false));
}
