use crate::utils::bit_operations_utils::unsigned_int_to_bits;
use num::ToPrimitive;
use std::mem::transmute;
use symphonia::core::audio::{AudioBuffer, Signal};

fn check_capacity(source_audio: &AudioBuffer<f32>, data_length: usize) {
    assert!(
        source_audio.frames() * source_audio.spec().channels.count() >= data_length,
        "Provided audio data does not have enough capacity to store {data_length} bytes"
    );
}

fn set_lsb(value: f32, flag: bool) -> f32 {
    let mut u = value.to_bits();

    if flag {
        u |= 1;
    } else {
        u &= !1;
    }

    f32::from_bits(u)
}

/// This function is not for user usage
fn embed_raw_bytes(source_audio: &mut AudioBuffer<f32>, data: Vec<u8>) {
    check_capacity(source_audio, data.len());

    let mut new_data: Vec<bool> = Vec::with_capacity(data.len() * 8);
    for byte in data {
        new_data.append(&mut unsigned_int_to_bits(byte));
    }

    let mut data_index = 0; // because of the check_capacity(...) check at the start, there is no need to check the data_index before every access
    for chan_i in 0..source_audio.spec().channels.count() {
        let mut chan = source_audio.chan_mut(chan_i);
        for sample_i in 0..chan.len() {
            chan[sample_i] = set_lsb(chan[sample_i], new_data[data_index]);
            data_index += 1;
        }
    }
}
