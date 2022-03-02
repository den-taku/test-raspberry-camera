use rascam::*;
use std::fs::File;
use std::io::Write;
use std::{thread, time};

fn main() {
    let info = info().unwrap();
    if info.cameras.len() < 1 {
        println!("Found 0 cameras. Exiting");
        std::process::exit(1);
    }

    simple_sync(&info.cameras[0]);
}

fn simple_sync(info: &CameraInfo) {
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    let settings = CameraSettings {
        // width: 800,
        // height: 600,
        ..CameraSettings::default()
    };
    camera.configure(settings);
    camera.activate().unwrap();

    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    let take_duration = time::Duration::from_millis(1000);
    let count = 30;
    for i in 0..count {
        let b = camera.take_one().unwrap();
        File::create(&format!("target/images/image{i}.jpg"))
            .unwrap()
            .write_all(&b)
            .unwrap();
        println!("Saved image as image{i}.jpg");
        thread::sleep(take_duration);
    }
}
