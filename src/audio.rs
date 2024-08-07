use std::{fs::File, io::BufReader, thread, time::Duration};
use rodio::{cpal::FromSample, source::SineWave, Decoder, OutputStream, Sample, Sink, Source};


fn play_audio<S>(source: S)  
    where
        S: Source + Send + 'static,
        f32: FromSample<S::Item>,
        S::Item: Sample + Send,
{
    // Play the sound on a separate thread so that it is non-blocking
    thread::spawn(move || {
        // Get an output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    });
}

pub fn play_audio_sin(freq: f32, ampl: f32) {
    let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(0.5)).amplify(ampl);
    play_audio(source);
}

pub fn play_audio_file(file_name: &str) {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(format!("assets/{}", file_name)).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    play_audio(source);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_play_audio_sin() {
        play_audio_sin(300.0, 0.5);
        thread::sleep(Duration::from_secs(1));
    }
}
