use std::{env, fs::File, io::Write};

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
    show_info();

    // Parse command line arguments
    // First argument is input .wav file, second argument is output text file.
    let args: Vec<String> = env::args().collect();
    // TODO: your code here

    if args.len() != 3 {
        eprintln!("Usage: {} <input_wav_file> <output_txt_file>", args[0]);
        std::process::exit(1);
    }

    let input_wav_file = &args[1];
    let output_txt_file = &args[2];

    // Open the input wave file and determine number of channels
    // TODO: your code here; see `hound::WavReader::open`.

    let mut reader = hound::WavReader::open(input_wav_file).expect("Failed to open input wave file");
    // println!("Sample format: {:?}", reader.spec().sample_format);

    let num_channels = reader.spec().channels;

    // Read audio data and write it to the output text file (one column per channel)
    // TODO: your code here; we suggest using `hound::WavReader::samples`, `File::create`, and `write!`.
    //       Remember to convert the samples to floating point values and respect the number of channels!

    let mut output_file = File::create(output_txt_file).expect("Failed to create output text file");

    for sample in reader.samples::<i32>() {
        let sample = sample.expect("Failed to read sample from input wave file") as f64;
        write!(output_file, "{:.6}", sample).expect("Failed to write to output text file");

        for _ in 1..num_channels {
            write!(output_file, " {:.6}", sample).expect("Failed to write to output text file");
        }

        writeln!(output_file, "").expect("Failed to write to output text file");
    }
}
