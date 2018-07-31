extern crate cmake;

use std::env;
use std::fs;

use cmake::Config;

fn main() {
	let src = env::current_dir().unwrap().join("snappy");

	let out = Config::new("snappy")
		.define("CMAKE_VERBOSE_MAKEFILE", "ON")
		.build_target("snappy")
		.build();

	let mut build = out.join("build");

	if cfg!(target_os = "windows") {
		let stub = build.join("snappy-stubs-public.h");

		let profile = match &*env::var("PROFILE").unwrap_or("debug".to_owned()) {
			"bench" | "release" => "Release",
			_ => "Debug",
		};
		build = build.join(profile);

		fs::copy(stub, build.join("snappy-stubs-public.h")).unwrap();
	}

	fs::copy(src.join("snappy.h"), build.join("snappy.h")).unwrap();

	println!("cargo:rustc-link-search=native={}", build.display());
	println!("cargo:rustc-link-lib=static=snappy");
	println!("cargo:include={}", build.display());
}
