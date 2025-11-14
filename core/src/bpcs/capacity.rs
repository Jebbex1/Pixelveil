use crate::{
    bpcs::{
        dynamic_prefix::{num_of_prefixed_planes_for_n_bits, prefix_length},
        initialization_vector::calculate_iv_plane_number,
    },
    errors::SteganographyError,
};

pub(crate) fn calculate_num_of_embedding_planes(
    min_alpha: f64,
    message_plane_length: u32,
) -> usize {
    let iv_plane_num = calculate_iv_plane_number(min_alpha);
    let conj_map_plane_num =
        num_of_prefixed_planes_for_n_bits(message_plane_length as usize, prefix_length(min_alpha));

    iv_plane_num + conj_map_plane_num + message_plane_length as usize
}

pub(crate) fn check_capacity(
    min_alpha: f64,
    message_plane_length: u32,
    accepted_planes_num: usize,
) -> Result<(), SteganographyError> {
    let required_min_accepted_num =
        calculate_num_of_embedding_planes(min_alpha, message_plane_length);
    if required_min_accepted_num <= accepted_planes_num {
        Ok(())
    } else {
        Err(SteganographyError::InsufficientPlaneNumber(
            required_min_accepted_num,
            accepted_planes_num,
        ))
    }
}
