use crate::{
    bpcs::{
        bit_plane::{BitPlane, PLANE_SIZE},
        dynamic_prefix::{
            data_bits_from_prefixed_planes, get_prefixed_planes, num_of_prefixed_planes_for_n_bits,
            prefix_length,
        },
    },
    errors::{SteganographyError, check_plane_number},
    utils::bit_operations::{bits_to_u32, unsigned_int_to_bits},
};

pub(crate) const MESSAGE_LENGTH_IV_BIT_NUMBER: usize = 32;
pub(crate) const MESSAGE_REMNANT_IV_BIT_NUMBER: usize = 32;

pub(crate) fn calculate_iv_plane_number(min_alpha: f64) -> usize {
    num_of_prefixed_planes_for_n_bits(MESSAGE_LENGTH_IV_BIT_NUMBER, prefix_length(min_alpha))
        + num_of_prefixed_planes_for_n_bits(MESSAGE_REMNANT_IV_BIT_NUMBER, prefix_length(min_alpha))
}

pub(crate) fn build_message_length_iv(message_plane_length: u32) -> Vec<bool> {
    unsigned_int_to_bits(message_plane_length)
}

pub(crate) fn build_message_remnant_iv(remnant_bit_number: u32) -> Vec<bool> {
    assert!(remnant_bit_number <= (PLANE_SIZE * PLANE_SIZE));
    unsigned_int_to_bits(remnant_bit_number)
}

pub(crate) fn build_iv_planes(
    min_alpha: f64,
    message_plane_length: u32,
    remnant_bit_number: u32,
) -> Vec<BitPlane> {
    let mut iv_planes: Vec<BitPlane> = Vec::new();
    iv_planes.extend(get_prefixed_planes(
        build_message_length_iv(message_plane_length),
        min_alpha,
    ));
    iv_planes.extend(get_prefixed_planes(
        build_message_remnant_iv(remnant_bit_number),
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

pub(crate) fn drain_iv_data_from_accepted_planes(
    planes: &mut Vec<BitPlane>,
    min_alpha: f64,
) -> Result<(u32, u32), SteganographyError> {
    let minimum_plane_num = calculate_iv_plane_number(min_alpha);
    check_plane_number(minimum_plane_num, planes.len())?;

    let prefix_length = prefix_length(min_alpha);
    let msg_len_iv_plane_length = num_of_prefixed_planes_for_n_bits(32, prefix_length);
    let msg_rem_iv_plane_length = num_of_prefixed_planes_for_n_bits(32, prefix_length);

    let length_iv_bits = data_bits_from_prefixed_planes(
        planes.drain(0..msg_len_iv_plane_length).collect(),
        min_alpha,
    )
    .drain(0..32) // get only the bits of iv data, without the filling
    .collect::<Vec<bool>>();

    let message_plane_length = bits_to_u32(length_iv_bits.try_into().unwrap());

    let remnant_iv_bits = data_bits_from_prefixed_planes(
        planes.drain(0..msg_rem_iv_plane_length).collect(),
        min_alpha,
    )
    .drain(0..32)
    .collect::<Vec<bool>>();

    let message_remnant_length = bits_to_u32(remnant_iv_bits.try_into().unwrap());

    Ok((message_plane_length, message_remnant_length))
}

pub(crate) fn drain_conjugation_map_from_accepted_planes(
    planes: &mut Vec<BitPlane>,
    min_alpha: f64,
    message_plane_length: u32,
) -> Result<Vec<bool>, SteganographyError> {
    let conjugation_map_plane_length =
        num_of_prefixed_planes_for_n_bits(message_plane_length as usize, prefix_length(min_alpha));

    check_plane_number(conjugation_map_plane_length, planes.len())?;

    let conjugation_map_data = data_bits_from_prefixed_planes(
        planes.drain(0..conjugation_map_plane_length).collect(),
        min_alpha,
    )
    .drain(0..message_plane_length as usize) // get only the bits of iv data, without the filling
    .collect::<Vec<bool>>();
    Ok(conjugation_map_data)
}

#[cfg(test)]
mod tests {
    use crate::bpcs::dynamic_prefix::get_n_random_bools;

    use super::*;

    #[test]
    fn test_circular_iv_generation_and_data_extraction() -> Result<(), Box<dyn std::error::Error>> {
        let (min_alpha, message_plane_length, remnant_bit_number) = (0.3, 65832, 4);
        let mut iv_planes = build_iv_planes(min_alpha, message_plane_length, remnant_bit_number);

        assert_eq!(
            drain_iv_data_from_accepted_planes(&mut iv_planes, min_alpha)?,
            (message_plane_length, remnant_bit_number)
        );

        Ok(())
    }

    #[test]
    fn test_circular_conjugation_map_generation_and_data_extraction()
    -> Result<(), Box<dyn std::error::Error>> {
        let message_plane_length = 47u32;
        let min_alpha = 0.3f64;
        let conjugation_map = get_n_random_bools(message_plane_length as usize);
        let mut conjugation_map_planes =
            build_conjugation_map_planes(conjugation_map.clone(), min_alpha);
        assert_eq!(
            drain_conjugation_map_from_accepted_planes(
                &mut conjugation_map_planes,
                min_alpha,
                message_plane_length
            )?,
            conjugation_map
        );

        Ok(())
    }
}
