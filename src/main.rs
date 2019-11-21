use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Duration;

use cpal::{StreamData, UnknownTypeOutputBuffer};
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use hound;

fn main() {
    let (audio_thread_tx, audio_thread_rx): (Sender<bool>, Receiver<bool>) = channel();
    
    let _audio_thread = thread::spawn(move || {
        let mut reader_1 = hound::WavReader::open("/home/pi/Downloads/Cassette808_Samples/Cassette808_CP_01-16bit.wav").unwrap();
        let sample_vec_1 = reader_1.into_samples::<i16>()
            .map(|x| x.unwrap() / 2)
            .collect::<Vec<_>>();
        let mut sample_cycle_1 = sample_vec_1.iter().cycle();
        
        let mut reader_2 = hound::WavReader::open("/home/pi/Downloads/Cassette808_Samples/Cassette808_BD01-16bit.wav").unwrap();
        let sample_vec_2 = reader_2.into_samples::<i16>()
            .map(|x| x.unwrap() / 2)
            .collect::<Vec<_>>();
        let mut sample_cycle_2 = sample_vec_2.iter().cycle();
        
        let host = cpal::default_host();
        let event_loop = host.event_loop();
        let device = host.default_output_device().expect("no output device available");
        let mut supported_formats_range = device.supported_output_formats()
            .expect("error while querying formats");
        
        //~ for i in supported_formats_range {
            //~ println!("{:?}", i);
        //~ }
        
        //~ let format = supported_formats_range.next()
            //~ .expect("no supported format?!")
            //~ .with_max_sample_rate();
            
        let format = cpal::Format{ channels: 1, sample_rate: cpal::SampleRate(44100), data_type: cpal::SampleFormat::I16 };
        
        //~ println!("{:?}", device.name().unwrap());
        //~ println!("{:?}", format);
            
        let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
        
        let mut sample_play = false;
        
        event_loop.run(move |stream_id, stream_result| {
            let stream_data = match stream_result {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                    return;
                }
                _ => return,
            };

            match stream_data {
                //~ StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                    //~ for elem in buffer.iter_mut() {
                        //~ *elem = u16::max_value() / 2;
                    //~ }
                //~ },
                StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                    for elem in buffer.iter_mut() {
                        if sample_play {
                            *elem = *sample_cycle_1.next().unwrap() + *sample_cycle_2.next().unwrap();
                        } else {
                            *elem = 0;
                        }
                    }
                },
                //~ StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                    //~ for elem in buffer.iter_mut() {
                        //~ *elem = 0.0;
                    //~ }
                //~ },
                _ => (),
            }
            
            match audio_thread_rx.try_recv() {
                Ok(val) => {
                    sample_play = val;
                },
                _ => (),
                
            }
        });
    });
    
    let mut playing = true;
    
    loop {
        thread::sleep(Duration::from_millis(1000));
        playing = !playing;
        audio_thread_tx.send(playing);
        
        //~ break;
    }
}
