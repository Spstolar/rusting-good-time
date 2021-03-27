use std::f32::consts::PI;
use std::fs::File;
use std::path::Path;
use itertools_num::linspace;

fn main() -> std::io::Result<()> {
    let freq: f32 = 400.0;
    let sample_rate: u32 = 44_100;
    let song_length: u32 = 8;

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
