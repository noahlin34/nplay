use audiotags::Tag;
use clap::Parser;
use indicatif;
use rodio::{dynamic_mixer, Decoder, OutputStream, Sink, Source};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::os::windows;
use std::path;
use std::time::Duration;
use std::{env, process::Output};
use std::{io, thread};

#[derive(Parser)]

struct cliArgs {
    path: std::path::PathBuf,

    vol: String,
}


//TODO: IMPLEMENT https://docs.rs/tui/latest/tui/ 

fn main() {
    //get the arguments parsed
    let args = cliArgs::parse();

    let tag = Tag::new().read_from_path(&args.path).unwrap();

    //cast volume argument to a string
    let strVol = &args.vol;

    //cast the volume string to f32 float
    let volume: f32 = strVol.parse().unwrap();

    //extract metadata
    let title = tag.title().unwrap();
    let artist = tag.artist().unwrap();
    let time = tag.duration();

    //print the information
    println!("Title: {:#?}", title);
    println!("Artist: {:#?}", artist);
    println!("{:#?}", time);

    //cast the std::PathBuf to a string to pass into play()
    let file = &args.path.display().to_string();

    //call the play function
    play(&file, volume);
}

fn play(filename: &String, volume: f32) -> Result<(), Box<dyn std::error::Error>> {
    //read file from passed filename
    let file = File::open(filename)?;

    let source = Decoder::new(BufReader::new(file))?;

    let (_stream, stream_handle) = OutputStream::try_default()?;

    let sink = Sink::try_new(&stream_handle)?;

    thread::spawn(|| {});

    sink.set_volume(volume);

    sink.append(source);

    //input handling thread is spawned in here for some reason
    thread::spawn(|| {
        println!("Playing!");

        //get the user's input
        let mut string = String::new();
        io::stdin().read_line(&mut string).expect("error");

        //need to cast the string to a char to extract first character
        let char = string.chars().next().unwrap();

        //kill code
        if char == 'q' {
            println!("goodbye");

            std::process::exit(0);
        }
    });

    sink.sleep_until_end();

    Ok(())
}

//TODO: FIX This
fn drawBar() {
    let args = cliArgs::parse();

    let path = &args.path;

    let tag = Tag::new().read_from_path(&args.path).unwrap();

    let duration = tag.duration();

    let pb = indicatif::ProgressBar::new(100);

    for i in 0..100 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
}
