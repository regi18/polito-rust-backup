use std::{fs::File, io::BufReader};
use rodio::{Sink, Decoder, OutputStream};

// use rodio::source::SineWave;
// fn get_audio_sin(freq) {
//     return SineWave::new(freq).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
// }

pub fn play_audio(file_name: &str) {
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(format!("assets/{}", file_name)).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
