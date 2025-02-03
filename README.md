# Auto Busylight

Do you have a busylight supported by an app like [Busylight](https://github.com/JnyJny/busylight)?  Do you also struggle to make sure your busylight is lit at exactly the right times so that your inappropriately dressed partner doesn't wander behind you while you're on camera?  This tool monitors your linux system's use of audio and video devices to automatically turn your busy light on or off depending upon whether there are any devices using your microphones or webcam.

# Building

You'll need a Rust environment and Cargo.  Then, from your clone of this git repository, run:

```bash
cargo build --release
```

If you use the out-of-the-box setup, you'll also need to have [Busylight](https://github.com/JnyJny/busylight) installed:

```bash
python3 -m pip install busylight-for-humans
```

but you can use your own busylight-controlling app if you want.  See `auto-busylight --help` for instructions.

# Running

Just run the following command from your clone of this git repository:

```
target/release/auto-busylight
```

The program will then periodically check to see whether the devices matching `/dev/video*` (for webcams) or `/dev/snd/pcmC*D*c` (for microphones).
