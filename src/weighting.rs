
use std::collections::HashMap;
use std::f64;

static mut SYMBOL : Vec<char> = Vec::new();

pub fn weight_henikoff( site_list : &Vec<String>, arg_t : &String ) -> Vec<f64>
{
	unsafe {
		if *arg_t == "no" { SYMBOL = "ARNDCQEGHILKMFPSTWYV-".chars().collect(); }
		else              { SYMBOL = "ARNDCQEGHILKMFPSTWYVBZXU-".chars().collect(); }
	}

	//Number of the sequences and sites.
	let num_seq  : usize = ( *site_list )[0].len();
	let num_site : usize = ( *site_list ).len();

	let mut weight_list : Vec<f64> = vec![ 0.0; num_seq ];

	for site in site_list.iter() {
		/*
		Calculate weighting factor using position based method (Henikoff, 1994).
		r = Number of the AA types in a site.
		s = Frequency of the AA in a site.
		weight_factor = 1 / (r * s).
		*/
		//println!( "{}", *site );
		let r : usize = count_types( site );
		for i in 0 .. ( *site ).len() {
			let aa_vec : Vec<char> = ( *site ).chars().collect();
			let aa : char = aa_vec[i];
			//println!( "{}", aa );
			let s : usize = count_freq( aa, site );
			let weight_factor : f64 = 1.0 / ( (r as f64) * (s as f64) );
			//println!( "weight_factor : {}", weight_factor );
			weight_list[i] += weight_factor;
		}
	}

	let mut sum_weight : f64 = 0.0;

	for i in 0 .. weight_list.len() {
		/*
		Get sequence weight by calculating mean of weighting factors in each sites.
		num_site = Denominator of mean.
		*/
		weight_list[i] = weight_list[i] / ( num_site as f64 );
		//println!( "Weight of Sequence {} : {:.3}", i + 1, weight_list[i] );
		sum_weight += weight_list[i];
	}

	println!( "\nSum of sequence weighting : {:.3}", sum_weight );

	weight_list
}

fn count_types( arg_site : &String ) -> usize
{
	let mut count : HashMap<char, usize> = HashMap::new();

	unsafe {
		for aa in SYMBOL.iter() { count.insert(*aa, 0); }
	}

	for aa in ( *arg_site ).chars() {
		let inc : usize = count[ &aa ] + 1;
		count.insert( aa, inc );
	}

	let mut r : usize = 0;
	unsafe {
		for aa in SYMBOL.iter() {
			if count[ aa ] != 0 {
				r += 1;
			}
		}
	}

	//println!( "Number of AA types in {} : {}", *arg_site, r);

	r
}

fn count_freq( arg_aa : char, arg_site : &String ) -> usize
{
	let aa_list : Vec<char> = ( *arg_site ).chars().collect();
	let mut freq : usize = 0;

	for i in 0 .. aa_list.len() {
		if arg_aa == aa_list[i] {
			freq += 1;
		}
	}

	//println!( "Frequency of {} in {} : {}", arg_aa, *arg_site, freq );

	freq
}
