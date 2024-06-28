#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate why_core;
extern crate why_p2p;

use why_core::ser;
use why_p2p::msg::Ping;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<Ping, ser::Error> = ser::deserialize(&mut d);
});
