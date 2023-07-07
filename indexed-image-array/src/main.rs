use std::collections::HashMap;

fn main() {
	let img = image::open("input.png").unwrap();

	assert!(img.width() == 320);
	assert!(img.height() == 240);

	let bytes = img.to_rgb8();

	let mut palette: HashMap<u32, u32> = HashMap::new();
	let mut index_map: Vec<u32> = Vec::new();

	let mut idx: u32 = 0;
	for pixel in bytes.pixels() {
		let color_idx = palette
			.entry(u32::from_be_bytes([pixel.0[2], pixel.0[1], pixel.0[0], 0x0]))
			.or_insert_with(|| {
				let tmp = idx;
				idx += 1;
				tmp
			});
		index_map.push(*color_idx);
	}

	const PALETTE_TMPL: &str = "const uint32_t GUI_COLOR_Colors4[%0] __attribute((aligned (4))) = { %1 };";
	const INDICES_TMPL: &str = "const unsigned char _ac[76800]  __attribute((aligned (4))) = { %0 };";

	let mut palette = palette.iter().map(|(a, b)| (*a, *b)).collect::<Vec<_>>();
	palette.sort_by(|a, b| a.1.cmp(&b.1));
	let palette = palette
		.into_iter()
		.map(|x| x.0)
		.map(|x| format!("{x:#X}")[0..6 + 2].to_owned())
		.collect::<Vec<_>>();

	println!("// Palette");
	println!(
		"{}",
		PALETTE_TMPL
			.replace("%0", &palette.len().to_string())
			.replace("%1", &palette.join(", "))
	);
	println!();
	println!("// Indices");
	println!(
		"{}",
		INDICES_TMPL.replace("%0", &index_map.iter().map(u32::to_string).collect::<Vec<_>>().join(", "))
	);
}
