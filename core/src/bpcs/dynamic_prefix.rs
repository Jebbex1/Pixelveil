use crate::bpcs::bit_plane::PLANE_SIZE;

pub(crate) fn prefix_length(min_alpha: f64) -> u64 {
    ((PLANE_SIZE * PLANE_SIZE) as f64 * (1.4 * min_alpha + 0.05)).ceil() as u64
}
