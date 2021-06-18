
use std::process;

pub fn show_result( arg_weight : &Vec<f64>, arg_title : &Vec<String> )
{
	println!( "\nResult:\n" );

	if ( *arg_weight ).len() != ( *arg_title ).len() {
		println!( "\nERROR !!!\n" );
		println!( "Length of (*arg_weight) != Length of (*arg_title)" );
		println!( "\nProgram halted !!!\n" );

		process::exit(1);
	}

	let num_seq : usize = ( *arg_weight ).len();

	println!( "num\tweight\ttitle" );
	for i in 0 .. num_seq {
		if ( *arg_title )[i].len() > 80 {
			print!( "{}\t{:.3}\t{}", i + 1, (*arg_weight)[i], (*arg_title)[i].chars().take(80).collect::<String>() );
			println!( " ..." );
		} else {
			println!( "{}\t{:.3}\t{}", i + 1, (*arg_weight)[i], (*arg_title)[i] );
		}
	}

}
