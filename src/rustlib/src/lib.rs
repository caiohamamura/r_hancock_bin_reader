#[macro_use]
extern crate rustr;
extern crate tls_read_hancock_bin;

use tls_read_hancock_bin::{HancockReader};
pub mod export;
pub use rustr::*;

const NUM_COLS: usize = 7;

// #[rustr_export]
//#' Name of Function
pub fn read_hancock_bin(input: String) -> RResult<SEXP> {
	let mut reader: HancockReader = HancockReader::new(input).unwrap();
	r_message(&format!("Number of beams: {}",reader.n_beams));
	let mut zen = Vec::with_capacity(reader.n_beams);
	let mut az = Vec::with_capacity(reader.n_beams);
	let mut x = Vec::with_capacity(reader.n_beams);
	let mut y = Vec::with_capacity(reader.n_beams);
	let mut z = Vec::with_capacity(reader.n_beams);
	let mut shot_n = Vec::with_capacity(reader.n_beams);
	let mut n_hits = Vec::with_capacity(reader.n_beams);
	let mut shot_n_ret = Vec::with_capacity(reader.n_beams);
	let mut r = Vec::with_capacity(reader.n_beams);
	let mut refl = Vec::with_capacity(reader.n_beams);
	// let r = Vec::with_capacity(reader.n_beams);
	// let refl = Vec::with_capacity(reader.n_beams);
	while let Some(data) = reader.next() {
		zen.push(data.zen);
		az.push(data.az);
		x.push(data.x);
		y.push(data.y);
		z.push(data.z);
		shot_n.push(data.shot_n);
		n_hits.push(data.n_hits);
		for i in 0..data.n_hits as usize {
			shot_n_ret.push(data.shot_n);
			r.push(data.r.try_borrow().unwrap()[i]);
			refl.push(data.refl.try_borrow().unwrap()[i]);
		}
	}

	unsafe {
		let list_all = Shield::new(Rf_allocVector(VECSXP, 2 as R_xlen_t)); 
		let vec_all = Shield::new(Rf_allocVector(VECSXP, NUM_COLS as R_xlen_t));
		let vec2_returns = Shield::new(Rf_allocVector(VECSXP, 3 as R_xlen_t)); 
		let vecs = vec_all.s();
		

		let mut list_all_m: RListM<Preserve> = RList::new(list_all.s()).unwrap(); 
		let mut list : RListM<Preserve> = RList::new(vecs).unwrap();
		let mut df_returns = RList::new(vec2_returns.s()).unwrap();
		list.uset(0, zen.uintor());
		list.uset(1, az.uintor());
		list.uset(2, x.uintor());
		list.uset(3, y.uintor());
		list.uset(4, z.uintor());
		list.uset(5, shot_n.uintor());
		list.uset(6, n_hits.uintor());
		df_returns.uset(0, shot_n_ret.uintor());
		df_returns.uset(1, r.uintor());
		df_returns.uset(2, refl.uintor());


		list.as_data_frame()?;
		
		// Name columns for beams df
		let mut colnames = CharVec::alloc(NUM_COLS);
		colnames.set(0,"zen")?;
		colnames.set(1,"az")?;
		colnames.set(2,"x")?;
		colnames.set(3,"y")?;
		colnames.set(4,"z")?;
		colnames.set(5,"shot_n")?;
		colnames.set(6,"n_hits")?;
		list.set_name(&colnames)?;

		// Name columns for reflectance df
		df_returns.as_data_frame()?;
		let mut colnames2 = CharVec::alloc(3);
		colnames2.set(0,"shot_n")?;
		colnames2.set(1,"r")?;
		colnames2.set(2,"refl")?;
		df_returns.set_name(&colnames2)?;
		

		list_all_m.uset(0, list);
		list_all_m.uset(1, df_returns);
		

		list_all_m.intor()
	}
}