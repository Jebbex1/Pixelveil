use std::{
    io::{self, Read},
    os::unix::fs::MetadataExt,
};

use pixelveil::{
    image::lossless::bpcs::{embed_data, extract_data},
    utils::image_handling::open_lossless_image_from_path,
};

#[test]
fn test_circular_bpcs() -> Result<(), Box<dyn std::error::Error>> {
    let mut source_image =
        open_lossless_image_from_path("tests/assets/test_circular_bpcs_vessel.png").unwrap();

    let data_file = std::fs::File::open("tests/assets/test_circular_bpcs_data.png").unwrap();
    let data_len = data_file.metadata().unwrap().size();
    let data = io::BufReader::new(data_file).bytes();
    let mut data = data.map(|res| res.unwrap());

    let min_alpha = 0.3f64;
    let rng_key = [42u8; 32];

    embed_data(
        &mut source_image,
        &mut data,
        data_len.try_into().unwrap(),
        min_alpha,
        rng_key,
    )?;

    let extracted = extract_data(source_image, min_alpha, rng_key)?;

    assert_eq!(
        extracted,
        std::fs::read("tests/assets/test_circular_bpcs_data.png").unwrap()
    );

    Ok(())
}
