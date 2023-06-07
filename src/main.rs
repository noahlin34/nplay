use std::os::windows;
use std::thread;
use std::{env, process::Output};
use std::error::Error;
use std::path;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, dynamic_mixer, Source};
use clap::Parser;
use audiotags::Tag;
use indicatif;



#[derive(Parser)]

struct cliArgs {

    path: std::path::PathBuf,

    vol: String,

}


fn main() {
    
    let args = cliArgs::parse();

    let tag = Tag::new().read_from_path(&args.path).unwrap();


     let strVol = &args.vol;

     let volume: f32 = strVol.parse().unwrap();

    let title = tag.title().unwrap();

    
    let artist = tag.artist().unwrap();
    let time = tag.duration();

    println!("Title: {:#?}", title);
    println!("Artist: {:#?}", artist);
    println!("{:#?}", time);
    let file = &args.path.display().to_string();


        play(&file, volume);


   
}


fn play(filename: &String, volume: f32) -> Result<(), Box<dyn std::error::Error>> {

    

    let file = File::open(filename)?;

    let source = Decoder::new(BufReader::new(file))?;



    let (_stream, stream_handle) = OutputStream::try_default()?;

    let sink = Sink::try_new(&stream_handle)?;

    thread::spawn(|| {


    });

    sink.set_volume(volume);


    sink.append(source);



    sink.sleep_until_end();



    Ok(())
}


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