use pixelveil::{
    image::lossless::bpcs::{embed_data, extract_data},
    utils::image_handling::open_lossless_image_from_path,
};

#[test]
fn test_circular_bpcs() -> Result<(), Box<dyn std::error::Error>> {
    let mut source_image =
        open_lossless_image_from_path("tests/assets/test_circular_bpcs_vessel.png").unwrap();
    let data = std::fs::read("tests/assets/test_circular_bpcs_data.png").unwrap();
    let min_alpha = 0.3f64;
    let rng_key = [42u8; 32];

    embed_data(&mut source_image, &data, min_alpha, rng_key)?;

    let extracted = extract_data(source_image, min_alpha, rng_key)?;

    assert_eq!(extracted, data);

    Ok(())
}
