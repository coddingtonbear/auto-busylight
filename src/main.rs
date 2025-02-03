use glob::glob;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn is_device_in_use(glob_pattern: &str) -> bool {
    let devices: Vec<_> = glob(glob_pattern).unwrap().filter_map(Result::ok).collect();

    for device in devices {
        let output = Command::new("fuser").arg(device.to_str().unwrap()).output();

        if let Ok(output) = output {
            if !output.stdout.is_empty() {
                println!("\tPID {}", String::from_utf8_lossy(&output.stdout));
                return true;
            }
        }
    }

    false
}

fn activate_busylight(activate: bool) -> bool {
    let mut command = Command::new("busylight");
    if activate {
        command.arg("on").arg("red");
    } else {
        command.arg("off");
    }

    let output = command.output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Command executed successfully!");
                if !output.stdout.is_empty() {
                    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
                }
                return true;
            } else {
                eprintln!(
                    "Command failed with error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return false;
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            return false;
        }
    }
}

fn main() {
    let check_interval = Duration::from_millis(2000);
    let mut light_on: bool = false;

    println!(
        "Ready!  Busylight will be activated automatically when webcam or microphone is accessed."
    );

    loop {
        let webcam_in_use: bool = is_device_in_use("/dev/video*");
        let microphone_in_use: bool = is_device_in_use("/dev/snd/pcmC*D*c");

        if webcam_in_use || microphone_in_use {
            if !light_on {
                println!("Meeting started");
                activate_busylight(true);
                light_on = true;
            }
        } else if light_on {
            println!("Meeting stopped");
            activate_busylight(false);
            light_on = false;
        }

        sleep(check_interval);
    }
}
