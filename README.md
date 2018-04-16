Files for https://github.com/rust-lang/rust/issues/49995#issuecomment-381618462

The project is executed, but the documentation is executed with an error.

	#![feature(plugin)]
	#![plugin(clucstr)]

	use std::ffi::CStr;

	fn main() {
			println_str(cstr!("cluWorld!!!"));
			//CSTR "cluWorld!!!"
			//CArray [99, 108, 117, 87, 111, 114, 108, 100, 33, 33, 33, 0] 12

			println_str(cstr!(b"cluWorld!!!"));
			//CSTR "cluWorld!!!"
			//CArray [99, 108, 117, 87, 111, 114, 108, 100, 33, 33, 33, 0] 12

			println_str(cstr!(b'A'));
			//CSTR "A"
			//CArray [65, 0] 2
	}


	fn println_str(cstr: &CStr) {
			println!("CSTR {:?}", cstr);

			let cstr_array = cstr.to_bytes_with_nul();
			println!("CArray {:?} {}", cstr_array, cstr_array.len());
			println!();
	}

# cargo run

	CSTR "cluWorld!!!"
	CArray [99, 108, 117, 87, 111, 114, 108, 100, 33, 33, 33, 0] 12

	CSTR "cluWorld!!!"
	CArray [99, 108, 117, 87, 111, 114, 108, 100, 33, 33, 33, 0] 12

	CSTR "A"
	CArray [65, 0] 2

# cargo doc --open

	 Compiling clucstr v0.1.1 (file:///cluCStr)
	 Documenting clucstr v0.1.1 (file:///cluCStr)
	 Documenting testcluStr v0.1.0 (file:///testcluStr)
	error[E0457]: plugin `clucstr` only found in rlib format, but must be available in dylib format
	 --> src/main.rs:2:11
		|
	2 | #![plugin(clucstr)]
		|           ^^^^^^^

	error: cannot find macro `cstr!` in this scope
	 --> src/main.rs:7:17
		|
	7 |     println_str(cstr!("cluWorld!!!"));
		|                 ^^^^

	error: cannot find macro `cstr!` in this scope
		--> src/main.rs:11:17
		 |
	11 |     println_str(cstr!(b"cluWorld!!!"));
		 |                 ^^^^

	error: cannot find macro `cstr!` in this scope
		--> src/main.rs:15:17
		 |
	15 |     println_str(cstr!(b'A'));
		 |                 ^^^^

	error: Could not document `testcluStr`.

	Caused by:
		process didn't exit successfully: `rustdoc --crate-name testcluStr src/main.rs -o /testcluStr/target/doc -L dependency=/testcluStr/target/debug/deps --extern clucstr=/testcluStr/target/debug/deps/libclucstr-83201db073bd59f0.rmeta` (exit code: 101)

# Version

	cargo --version
	cargo 1.26.0-nightly (008c36908 2018-04-13)

	rustc --version
	rustc 1.27.0-nightly (bd40cbbe1 2018-04-14)

# Lib - compile plugin

	https://crates.io/crates/clucstr

	[dependencies]
	clucstr = "<0.1.1"
