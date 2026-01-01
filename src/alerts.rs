use rust_embed::Embed;
use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::io::Cursor;

#[derive(Embed)]
#[folder = "File\path\to\your.mp3"] 
struct Asset;

pub fn trigger_timer_end(_ctx: &egui::Context) {
    // 1. Initialize the output stream using the new Builder API
    let stream_handle = OutputStreamBuilder::open_default_stream()
        .expect("Failed to open default output stream");
    
    // 2. Initialize the sink using the mixer from the stream handle
    let sink = Sink::connect_new(stream_handle.mixer());

    // 3. Load embedded file
    let embedded_file = Asset::get("File\path\to\your.mp3")
        .expect("rickrolll.mp3 not found in assets/");

    // 4. Wrap bytes in Cursor for decoding
    let cursor = Cursor::new(embedded_file.data);
    let source = Decoder::new(cursor)
        .expect("Failed to decode MP3");

    // 5. Play audio
    sink.append(source);
    
    // Note: The stream_handle must remain in scope for playback to continue
    sink.sleep_until_end();


}
