use std::io::Write;

extern crate bindgen;
extern crate shlex;


/// Path to the kernel bindings source file to generate
const FILEPATH_CODE:   &'static str = "src/os/kernel.rs";

/// Name of the file to store include directives for the C preprocessor in
const FILENAME_HEADER: &'static str = "kernel-include.h";

/// List of headers required by linux-std itself
const CLANG_HEADER_REQUIRED: [&'static str; 3] = [
	"linux/printk.h",  // For the `KernelDebugWriter` (used by the `println!()` macro)
	"linux/slab.h",    // For converting Rust strings to C strings at runtime
	"linux/hrtimer.h", //WORKAROUND: "linux/timer.h" misses an include to this file 😔
];

/// List of parameters not ever to pass to the clang parser of rust-bindgen
const CLANG_ARGS_BLACKLIST: [&'static str; 10] = [
	"-mno-80387", "-mno-fp-ret-in-387", "-mskip-rax-setup", "-maccumulate-outgoing-args",
	"-mpreferred-stack-boundary=3", "-mfentry",
	"-fno-var-tracking-assignments", "-fconserve-stack", "-DCC_HAVE_ASM_GOTO",
	"-fno-delete-null-pointer-checks"
];


/// Logging printer for output from rust-bindgen
struct Logger;
impl bindgen::Logger for Logger {
	fn error(&self, msg: &str) {
		writeln!(&mut std::io::stderr(), "error: {}", msg).unwrap();
	}
	
	fn warn(&self, msg: &str) {
		writeln!(&mut std::io::stderr(), "warning: {}", msg).unwrap();
	}
}

fn main() {
	// Read and parse required environment variables
	let clang_args = match std::env::var("STD_CLANG_ARGS") {
		Ok(string) =>
			match shlex::split(string.as_str()) {
				Some(mut args) => {
					// Find positions of arguments to remove
					let mut remove_indices = Vec::with_capacity(CLANG_ARGS_BLACKLIST.len());
					for (index, clang_arg) in args.iter().enumerate() {
						if CLANG_ARGS_BLACKLIST.contains(&clang_arg.as_str()) {
							remove_indices.push(index);
						}
					}
					
					// Remove the found positions from the argument list
					for index in remove_indices.iter().rev() {
						args.swap_remove(*index);
					}
					
					args
				},
				None => {
					panic!("Malformed environment variable STD_CLANG_ARGS");
				}
			},
		Err(error) => {
			panic!("Missing environment variable STD_CLANG_ARGS: {:?}", error);
		}
	};
	let clang_files = match std::env::var("STD_CLANG_FILES") {
		Ok(string) =>
			match shlex::split(string.as_str()) {
				Some(args) => args,
				None => {
					panic!("Malformed environment variable STD_CLANG_FILES");
				}
			},
		Err(error) => {
			panic!("Missing environment variable STD_CLANG_FILES: {:?}", error);
		}
	};
	let kernel_path = match std::env::var("STD_KERNEL_PATH") {
		Ok(string) => string,
		Err(error) => {
			panic!("Missing environment variable STD_KERNEL_PATH: {:?}", error);
		}
	};
	let out_dir = match std::env::var("OUT_DIR") {
		Ok(string) => string,
		Err(error) => {
			panic!("Missing environment variable OUT_DIR: {:?}", error);
		}
	};
	
	
	
	let filepath_header = format!("{}/{}", out_dir, FILENAME_HEADER);
	
	// Assemble parsing options
	let mut options = bindgen::BindgenOptions {
		builtins:     true,
		// We need the original `clang_args` for writing the build parameters later on
		clang_args:   clang_args.clone(),
		derive_debug: false,
		emit_ast:     false,
		.. Default::default()
	};
	
	// Prevent the kernel from declaring datatypes that are rust internal datatypes
	options.clang_args.push(String::from("-Dfalse=__false"));
	options.clang_args.push(String::from("-Dtrue=__true"));
	options.clang_args.push(String::from("-Du64=__u64"));
	
	// Tell clang to process the generated header file
	options.clang_args.push(filepath_header.clone());
	
	// Push supplied header file paths (relative to the kernel directory)
	match std::fs::File::create(filepath_header.clone()) {
		Ok(mut file) => {
			// Generate include lines for all requested headers
			for clang_file in clang_files.iter() {
				writeln!(file, "#include <{}>", clang_file).unwrap();
			}
			
			// Generate include lines for headers required by linux-std itself
			for clang_file in CLANG_HEADER_REQUIRED.iter() {
				writeln!(file, "#include <{}>", clang_file).unwrap();
			}
		},
		Err(error) => {
			panic!("Failed to open file \"{}\": {}", filepath_header, error);
		}
	}
	
	
	
	{
		// Open output file
		let mut file = match std::fs::File::create(FILEPATH_CODE) {
			Ok(file)   => file,
			Err(error) => {
				panic!("Failed to open file \"{}\": {}", FILEPATH_CODE, error);
			}
		};
		
		let build_path  = std::env::current_dir().unwrap().as_path().to_owned();
		let kernel_path = std::path::Path::new(kernel_path.as_str());
		assert!(std::env::set_current_dir(&kernel_path).is_ok());
		
		// Debugging information (displayed in case of problems)
		println!("Working directory: {}", std::env::current_dir().unwrap().to_str().unwrap());
		println!("LLVM arguments:    {}", options.clang_args.join(" "));
		
		let logger = Logger {};
		match bindgen::Bindings::generate(&options, Some(&logger), None) {
			Ok(result) => {
				file.write_all(result.to_string().as_bytes()).unwrap();
			},
			_ => {
				// Logger hopefully has already printed all relevant information
				panic!("Error generating bindings!");
			}
		}
		
		assert!(std::env::set_current_dir(&build_path).is_ok());
	};
}
