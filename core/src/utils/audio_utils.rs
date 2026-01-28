use symphonia::core::audio::{AudioBuffer, Signal, SignalSpec};
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::{MediaSource, MediaSourceStream};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

fn merge_buffers(buffers: &[AudioBuffer<f32>]) -> AudioBuffer<f32> {
    assert!(!buffers.is_empty(), "provided no buffers to merge");

    // 1. Calculate total duration
    let total_duration: usize = buffers.iter().map(|b| b.frames()).sum();
    let spec = *buffers[0].spec();

    // 2. Create new buffer
    let mut merged_buffer = AudioBuffer::<f32>::new(total_duration as u64, spec);
    merged_buffer.render_reserved(None);

    // 3. Copy data channel by channel
    let mut current_offset = 0;
    for buffer in buffers {
        for channel in 0..spec.channels.count() {
            let channel_data = buffer.chan(channel);
            let target_channel = merged_buffer.chan_mut(channel);
            target_channel[current_offset..current_offset + buffer.frames()]
                .copy_from_slice(channel_data);
        }
        current_offset += buffer.frames();
    }

    merged_buffer
}

fn import_audio(src: Box<dyn MediaSource>) -> Result<AudioBuffer<f32>, Error> {
    // Create the media source stream.
    let mss = MediaSourceStream::new(src, Default::default());

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let probed = symphonia::default::get_probe()
        .format(&Hint::new(), mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    // Get the instantiated format reader.
    let mut format: Box<dyn FormatReader> = probed.format;

    // Find the first audio track with a known (decodeable) codec.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    // Use the default options for the decoder.
    let dec_opts: DecoderOptions = Default::default();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");

    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;

    let mut packet_audio_buffers: Vec<AudioBuffer<f32>> = Vec::new();
    let mut summed_capacity: u64 = 0;
    let mut spec: Option<SignalSpec> = None;

    // The decode loop.
    loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::IoError(_)) => {
                break;
            }
            Err(err) => {
                // A unrecoverable error occured, halt decoding.
                panic!("{}", err);
            }
        };

        // Consume any new metadata that has been read since the last packet.
        while !format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            format.metadata().pop();
        }

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples.
        match decoder.decode(&packet) {
            Ok(decoded) => {
                // update final audio buffer parameters

                summed_capacity += decoded.frames() as u64;
                if let Some(s) = spec {
                    assert_eq!(s, *decoded.spec())
                } else {
                    spec = Some(*decoded.spec());
                }

                let mut b: AudioBuffer<f32> =
                    AudioBuffer::new(decoded.capacity() as u64, *decoded.spec());

                decoded.convert(&mut b);

                packet_audio_buffers.push(b);
            }
            Err(Error::IoError(_)) => {
                // The packet failed to decode due to an IO error, skip the packet.
                continue;
            }
            Err(Error::DecodeError(_)) => {
                // The packet failed to decode due to invalid data, skip the packet.
                continue;
            }
            Err(err) => {
                // An unrecoverable error occurred, halt decoding.
                panic!("{}", err);
            }
        }
    }

    assert!(
        summed_capacity > 0 && packet_audio_buffers.len() > 0 && spec.is_some(),
        "The selected track is empty"
    );

    Ok(merge_buffers(&packet_audio_buffers))
}

#[cfg(test)]
mod tests {
    use symphonia::core::audio::{AudioBuffer, Channels, Signal, SignalSpec};
    use crate::utils::audio_utils::merge_buffers;

    #[test]
    fn test_merge_buffers() {
        let spec = SignalSpec {
            channels: Channels::FRONT_LEFT | Channels::FRONT_RIGHT,
            rate: 44100,
        };

        let mut ab1: AudioBuffer<f32> = AudioBuffer::new(4, spec);
        ab1.render_reserved(None);
        ab1.chan_mut(0)[2] = 0.3f32;
        ab1.chan_mut(0)[1] = -0.55221f32;
        ab1.chan_mut(1)[0] = -0.9f32;
        ab1.chan_mut(1)[3] = 0.8f32;

        let mut ab2: AudioBuffer<f32> = AudioBuffer::new(4, spec);
        ab2.render_reserved(None);
        ab2.chan_mut(0)[0] = -0.1f32;
        ab2.chan_mut(0)[3] = 0.6f32;
        ab2.chan_mut(1)[1] = 0f32;
        ab2.chan_mut(1)[2] = 1f32;

        let buffers = &[ab1, ab2];

        let merged_buffer = merge_buffers(buffers);

        assert_eq!(merged_buffer.chan(0)[2], 0.3f32);
        assert_eq!(merged_buffer.chan(0)[1], -0.55221f32);
        assert_eq!(merged_buffer.chan(1)[0], -0.9f32);
        assert_eq!(merged_buffer.chan(1)[3], 0.8f32);

        assert_eq!(merged_buffer.chan(0)[4], -0.1f32);
        assert_eq!(merged_buffer.chan(0)[7], 0.6f32);
        assert_eq!(merged_buffer.chan(1)[5], 0f32);
        assert_eq!(merged_buffer.chan(1)[6], 1f32);
    }
}
