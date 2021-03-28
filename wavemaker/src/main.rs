use std::f32::consts::PI;
use std::fs::File;
use std::path::Path;
use itertools_num::linspace;
use std::io;

fn get_song_length() -> u32 {
    let mut song_length = String::new();
    println!("How long should the song be (in seconds)?");
    io::stdin()
        .read_line(&mut song_length)
        .expect("Failed to get song length.");
    let song_length: u32 = song_length.trim().parse().expect("Please type a number.");
    song_length
}

fn get_song_frequencies() -> String {
    let mut song_frequencies = String::new();
    println!("Provide frequencies to play, separated by spaces.");
    io::stdin()
        .read_line(&mut song_frequencies)
        .expect("Failed to get frequncies.");

    let song_frequencies: String = song_frequencies.trim().expect("Please provide frequencies.");
    song_frequencies
}

fn main() -> std::io::Result<()> {
    let song_length: u32 = get_song_length();
    println!("Song will be {} seconds long.", song_length);

    // next we want the user to provide space-separated frequencies
    let freq: f32 = 400.0;
    let sample_rate: u32 = 44_100;

    let one_sec: Vec<f32> = linspace::<f32>(0., 1., sample_rate as usize).collect();
    let mut x: Vec<f32> = linspace::<f32>(0., 1., sample_rate as usize).collect();

    for _ in 1..song_length{
        // simply appending
        // x.append(&mut one_sec);
        // does not work. append() empties out the contents of the 
        // appended vector! So, after the first append, we aren't
        // doing anything
        x.extend(one_sec.iter().cloned());
    }

    for i in &mut x {
        *i = (freq * 2.0 * PI * *i).sin();
    }

    let header = wav::Header::new(wav::header::WAV_FORMAT_PCM, 1, sample_rate, 32);
    let mut out_file = File::create(Path::new("data/output.wav"))?;

    let data = wav::bit_depth::BitDepth::from(x);
    wav::write(header, &data, &mut out_file);
    Ok(())
}
