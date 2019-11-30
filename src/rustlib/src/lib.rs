extern crate rustr;
extern crate tls_read_hancock_bin;

use tls_read_hancock_bin::HancockReader;
pub mod export;
pub use rustr::*;

// #[rustr_export]
//#' Name of Function
pub fn read_hancock_bin(input: String) -> RResult<SEXP> {
	let mut reader: HancockReader = HancockReader::new(input).unwrap();
	r_message(&format!("Number of beams: {}", reader.n_beams));

	let mut xoff = reader.xoff;
	let mut yoff = reader.yoff;
	let mut zoff = reader.zoff;

	let mut offsets = vec![xoff, yoff, zoff];
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

		let mut offsets_df = rlist!(
			offsets ~ offsets
		);
		offsets_df.as_data_frame()?;


		let mut beams_df = rlist!(
			zen ~ zen,
			az ~ az,
			x ~ x,
			y ~ y,
			z ~ z,
			shot_n ~ shot_n,
			n_hits ~ n_hits
		);
		beams_df.as_data_frame()?;

		// Name columns for reflectance df
		let mut returns_df = rlist!(
			shot_n ~ shot_n_ret,
			r ~ r,
			refl ~ refl
		);
		returns_df.as_data_frame()?;

		let list_all_m = rlist!(
			offsets ~ offsets_df,
			beams ~ beams_df, 
			returns ~ returns_df
		);

		list_all_m.intor()
	}
}
