use std::f32::consts::PI;
use std::fs::File;
use std::path::Path;
use itertools_num::linspace;
use std::io;

fn get_song_length() -> f32 {
    let mut song_length = String::new();
    println!("How long should the song be (in seconds)?");
    io::stdin()
        .read_line(&mut song_length)
        .expect("Failed to get song length.");
    let song_length: f32 = song_length.trim().parse().expect("Please type a number.");
    song_length
}

fn get_song_frequencies() -> String {
    let mut song_frequencies = String::new();
    println!("Provide frequencies to play, separated by spaces.");
    io::stdin()
        .read_line(&mut song_frequencies)
        .expect("Failed to get frequncies.");

    let song_frequencies: String = song_frequencies
        .trim()
        .parse()
        .expect("Please provide frequencies.");
    song_frequencies
}

fn str_to_int(number_str: &&str) -> u32 {
    // why did we need &&str rather than &str?
    number_str.parse::<u32>().unwrap()
}

fn main() -> std::io::Result<()> {
    let song_length: f32 = get_song_length();
    println!("Song will be {} seconds long.", song_length);

    let song_frequencies = get_song_frequencies();
    let song_frequencies: Vec<&str> = song_frequencies.split(' ').collect();
    let song_frequencies: Vec<u32> = song_frequencies
        .iter()
        .map(str_to_int)
        .collect();

    println!("{:?}", song_frequencies);

    // next we want the user to provide space-separated frequencies
    let freq: f32 = 400.0;
    let sample_rate: u32 = 44_100;

    // let one_sec: Vec<f32> = linspace::<f32>(0., 1., sample_rate as usize).collect();
    let num_samples: f32 = song_length * sample_rate as f32;
    let mut x: Vec<f32> = linspace::<f32>(0., song_length, num_samples as usize).collect();

    // no longer needed but still an interesting learning experience
    // for _ in 1..song_length{
    //     // simply appending
    //     // x.append(&mut one_sec);
    //     // does not work. append() empties out the contents of the 
    //     // appended vector! So, after the first append, we aren't
    //     // doing anything
    //     x.extend(one_sec.iter().cloned());
    // }

    for i in &mut x {
        if i < &mut 1.0 {
            *i = (song_frequencies[0] as f32 * 2.0 * PI * *i).sin();
        } else if i < &mut 2.0 {
            *i = (song_frequencies[1] as f32 * 2.0 * PI * *i).sin();
        } else{
            *i = (song_frequencies[2] as f32 * 2.0 * PI * *i).sin();
        }
    }

    let header = wav::Header::new(wav::header::WAV_FORMAT_PCM, 1, sample_rate, 32);
    let mut out_file = File::create(Path::new("data/output.wav"))?;

    let data = wav::bit_depth::BitDepth::from(x);
    wav::write(header, &data, &mut out_file);
    Ok(())
}
