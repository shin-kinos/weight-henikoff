
use std::env;
use std::process;

pub struct Options {
	pub input    : String,
	pub tolerate : String,
}

impl Options {
	pub fn new() -> Options
	{
		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut arg_i : &String = &String::new();
		let mut arg_t : &String = &String::from( "yes" );

		if argc < 3 { show_usage(&argv[0]) };

		let mut i : usize = 1;
		while i < argc {
			match argv[i].as_str() {
				"-i" => { i += 1; arg_i = &argv[i]; },
				"-t" => { i += 1; arg_t = &argv[i]; }
				"-h" => { show_usage(&argv[0]); },
				_    => { show_usage(&argv[0]); },
			}
			i += 1;
		}

		match ( *arg_t ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[0] ),
		}

		Options {
			input    : arg_i.to_string(),
			tolerate : arg_t.to_string(),
		}
	}

	pub fn show_parameter( &self )
	{
		println!( "\nParameter set:" );
		println!( "===========================================" );
		println!( "Input filename  : {}", self.input );
		println!( "Tolerate BZXU   : {}", self.tolerate );
		println!( "===========================================" );
	}
}

fn show_usage( arg : &String )
{
	println!( "Usage: {} [Options] \n\nOptions\n\n", *arg );
	println!( "   -i\tInput filename in aligned Multi-FASTA format, REQUIRED." );
	println!( "   -t\tTolerate AA types 'B', 'Z', 'X' and 'U' in input file ('yes' or 'no', default 'yes').\n    \tIf 'no', program halts when the input file includes B, Z, X, or U." );
	println!( "   -h\tPrint this help, ignore all other arguments." );
	println!( "\n" );

	process::exit(1);
}
