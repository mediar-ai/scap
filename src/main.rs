// This program is just a testing application
// Refer to `lib.rs` for the library source code

use scap::{
    capturer::{Capturer, Options},
    frame::Frame,
    Target,
};
use image::{ImageBuffer, Rgba};
use std::path::Path;

fn main() {
    // Check if the platform is supported
    if !scap::is_supported() {
        println!("❌ Platform not supported");
        return;
    }

    // Check if we have permission to capture screen
    // If we don't, request it.
    if !scap::has_permission() {
        println!("❌ Permission not granted. Requesting permission...");
        if !scap::request_permission() {
            println!("❌ Permission denied");
            return;
        }
    }

    // Get recording targets
    let targets = scap::get_all_targets();
    println!("Available targets:");
    for (i, target) in targets.iter().enumerate() {
        println!("{}: {:?}", i, target);
    }

    // Find the specific window with id 76 and title "mod.rs — screenpipe"
    let target_window = targets.iter().find_map(|target| {
        if let Target::Window(window) = target {
            if window.id == 76 {
                Some(window.clone())
            } else {
                None
            }
        } else {
            None
        }
    });

    if target_window.is_none() {
        println!("❌ Target window not found");
        return;
    }

    println!("Selected target window: {:?}", target_window);

    // Create Options
    let options = Options {
        fps: 60,
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_720p,
        target: target_window.map(Target::Window),
        crop_area: None, // Remove the crop area for now
        ..Default::default()
    };

    println!("Capture options: {:?}", options);

    // Create Recorder with options
    let mut recorder = Capturer::new(options).expect("Failed to create capturer");

    // Start Capture
    recorder.start_capture();
    println!("Capture started successfully");

    // Capture 1 frame
    let frame = recorder.get_next_frame().expect("Error getting frame");

    match frame {
        Frame::BGRA(frame) => {
            println!(
                "Received BGRA frame of width {} and height {} and time {}",
                frame.width,
                frame.height,
                frame.display_time
            );

            // Check if frame data is empty
            if frame.data.is_empty() {
                println!("❌ Frame data is empty");
                return;
            }

            // Save the frame as a PNG file
            let img = ImageBuffer::<Rgba<u8>, _>::from_raw(
                frame.width as u32,
                frame.height as u32,
                frame.data,
            ).expect("Failed to create image buffer");

            let file_name = "captured_frame.png";
            img.save(Path::new(file_name)).expect("Failed to save image");

            println!("Saved frame to {}", file_name);
        }
        _ => {
            println!("Received unsupported frame type");
        }
    }

    // Stop Capture
    recorder.stop_capture();
}