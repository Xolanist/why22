#![no_main]
use libfuzzer_sys::fuzz_target;

extern crate why_core;

use why_core::core::UntrustedCompactBlock;
use why_core::ser::{self, DeserializationMode};

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<UntrustedCompactBlock, ser::Error> =
		ser::deserialize(&mut d, ser::ProtocolVersion(1), DeserializationMode::Full);
});
