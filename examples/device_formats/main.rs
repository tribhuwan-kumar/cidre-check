use cidre::{av, av::capture::device};

fn main() {
    let device_type = device::Type::built_in_wide_angle_camera();
    let media_type = av::MediaType::video();
    let position = device::Position::Front;
    let device = device::Device::with_device_type_media_and_position(
        device_type,
        Some(media_type),
        position,
    )
    .expect("device");

    let device_id = device.unique_id().to_string();
    println!("device id: {:?}", device_id);
    let formats = device.formats();
    for f in formats.iter() {
        let format_description = f.format_description();
        let resolution = format_description.dimensions();
        println!("resolution: {:?}", resolution);

        let ranges = f.video_supported_frame_rate_ranges();
        println!("autofocus {:?}", f.auto_focus_system());
        for r in ranges.iter() {
            println!(
                "  frame_rate {:?}-{:?}",
                r.min_frame_rate(),
                r.max_frame_rate()
            );
            println!(
                "  frame_duration {:?}-{:?}",
                r.min_frame_duration(),
                r.max_frame_duration()
            );
        }
    }
}
