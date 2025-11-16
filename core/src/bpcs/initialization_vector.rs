use crate::{
    bpcs::{
        bit_plane::{BitPlane, PLANE_SIZE, USIZE_PLANE_SIZE},
        dynamic_prefix::{
            data_bits_from_prefixed_planes, get_prefixed_planes, num_of_prefixed_planes_for_n_bits,
            prefix_length,
        },
    },
    errors::{SteganographyError, check_plane_number},
    utils::bit_operations::{USIZE_BIT_DEPTH, bits_to_usize, unsigned_int_to_bits},
};

pub(crate) const MESSAGE_LENGTH_IV_BIT_NUMBER: usize = USIZE_BIT_DEPTH;
pub(crate) const MESSAGE_REMNANT_IV_BIT_NUMBER: usize = USIZE_BIT_DEPTH;

pub(crate) fn calculate_iv_plane_number(min_alpha: f64) -> usize {
    num_of_prefixed_planes_for_n_bits(MESSAGE_LENGTH_IV_BIT_NUMBER, prefix_length(min_alpha))
        + num_of_prefixed_planes_for_n_bits(MESSAGE_REMNANT_IV_BIT_NUMBER, prefix_length(min_alpha))
}

pub(crate) fn build_message_length_iv(message_plane_length: usize) -> Vec<bool> {
    unsigned_int_to_bits(message_plane_length)
}

pub(crate) fn build_message_remnant_iv(remnant_bit_number: usize) -> Vec<bool> {
    assert!(remnant_bit_number <= (USIZE_PLANE_SIZE * USIZE_PLANE_SIZE));
    unsigned_int_to_bits(remnant_bit_number)
}

pub(crate) fn build_iv_planes(
    min_alpha: f64,
    message_plane_length: usize,
    remnant_bit_number: usize,
) -> Vec<BitPlane> {
    let mut iv_planes: Vec<BitPlane> = Vec::new();
    iv_planes.extend(get_prefixed_planes(
        build_message_length_iv(message_plane_length),
        min_alpha,
    ));
    iv_planes.extend(get_prefixed_planes(
        build_message_remnant_iv(remnant_bit_number.try_into().unwrap()),
        min_alpha,
    ));

    iv_planes
}

pub(crate) fn build_conjugation_map_planes(
    conjugation_map: Vec<bool>,
    min_alpha: f64,
) -> Vec<BitPlane> {
    get_prefixed_planes(conjugation_map, min_alpha)
}

pub(crate) fn extract_iv_data_from_iv_planes(
    mut planes: Vec<BitPlane>,
    min_alpha: f64,
) -> Result<(usize, usize), SteganographyError> {
    let minimum_plane_num = calculate_iv_plane_number(min_alpha);
    check_plane_number(minimum_plane_num, planes.len())?;

    let prefix_length = prefix_length(min_alpha);
    let msg_len_iv_plane_length =
        num_of_prefixed_planes_for_n_bits(MESSAGE_LENGTH_IV_BIT_NUMBER, prefix_length);
    let msg_rem_iv_plane_length =
        num_of_prefixed_planes_for_n_bits(MESSAGE_REMNANT_IV_BIT_NUMBER, prefix_length);

    let length_iv_bits = data_bits_from_prefixed_planes(
        planes.drain(0..msg_len_iv_plane_length).collect(),
        min_alpha,
    )
    .drain(0..MESSAGE_LENGTH_IV_BIT_NUMBER) // get only the bits of iv data, without the filling
    .collect::<Vec<bool>>();

    let message_plane_length = bits_to_usize(length_iv_bits.try_into().unwrap());

    let remnant_iv_bits = data_bits_from_prefixed_planes(
        planes.drain(0..msg_rem_iv_plane_length).collect(),
        min_alpha,
    )
    .drain(0..MESSAGE_REMNANT_IV_BIT_NUMBER)
    .collect::<Vec<bool>>();

    let message_remnant_length = bits_to_usize(remnant_iv_bits.try_into().unwrap());

    if message_remnant_length > USIZE_PLANE_SIZE * USIZE_PLANE_SIZE {
        let bits_per_plane = PLANE_SIZE * PLANE_SIZE;
        return Err(SteganographyError::InvalidIVData(String::from(format!(
            "Message remnant IV can't be more than the amount of bits per plane ({message_remnant_length} > {bits_per_plane})"
        ))));
    }

    Ok((message_plane_length, message_remnant_length))
}

pub(crate) fn extract_conj_map_data_from_conj_map_planes(
    planes: Vec<BitPlane>,
    min_alpha: f64,
    message_plane_length: usize,
) -> Result<Vec<bool>, SteganographyError> {
    let conjugation_map_plane_length =
        num_of_prefixed_planes_for_n_bits(message_plane_length, prefix_length(min_alpha));

    check_plane_number(conjugation_map_plane_length, planes.len())?;

    let conjugation_map_data = data_bits_from_prefixed_planes(planes, min_alpha)
        .drain(0..message_plane_length) // get only the bits of iv data, without the filling
        .collect::<Vec<bool>>();
    Ok(conjugation_map_data)
}

#[cfg(test)]
mod tests {
    use crate::bpcs::dynamic_prefix::get_n_random_bools;

    use super::*;

    #[test]
    fn test_circular_iv_generation_and_data_extraction() -> Result<(), Box<dyn std::error::Error>> {
        let (min_alpha, message_plane_length, remnant_bit_number) = (0.3, 65832usize, 4);
        let iv_planes = build_iv_planes(min_alpha, message_plane_length, remnant_bit_number);

        assert_eq!(
            extract_iv_data_from_iv_planes(iv_planes, min_alpha)?,
            (message_plane_length, remnant_bit_number)
        );

        Ok(())
    }

    #[test]
    fn test_circular_conjugation_map_generation_and_data_extraction()
    -> Result<(), Box<dyn std::error::Error>> {
        let message_plane_length = 47usize;
        let min_alpha = 0.3f64;
        let conjugation_map = get_n_random_bools(message_plane_length as usize);
        let conjugation_map_planes =
            build_conjugation_map_planes(conjugation_map.clone(), min_alpha);
        assert_eq!(
            extract_conj_map_data_from_conj_map_planes(
                conjugation_map_planes,
                min_alpha,
                message_plane_length
            )?,
            conjugation_map
        );

        Ok(())
    }
}
