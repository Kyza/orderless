use orderless::call_orderless;

fn two<T>(a: T, b: T) -> (T, T) {
	(a, b)
}

#[test]
fn none() {
	assert_eq!(
		call_orderless! {
			func = two,
			defs(a = false, b = false),
			args()
		},
		(false, false)
	);
}

#[test]
fn just_a() {
	assert_eq!(
		call_orderless! {
			func = two,
			defs(a = false, b = false),
			args(a = true)
		},
		(true, false)
	);
}

#[test]
fn just_b() {
	let b: bool = true;
	assert_eq!(
		call_orderless! {
			func = two,
			defs(a = false, b = false),
			args(b)
		},
		(false, true)
	);
}

#[test]
fn a_and_b() {
	let b: bool = true;
	assert_eq!(
		call_orderless! {
			func = two,
			defs(a = false, b = false),
			args(b, a = true)
		},
		(true, true)
	);
}
