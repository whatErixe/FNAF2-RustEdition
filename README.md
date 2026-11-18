# FNAF 2: Rust Edition — Android Prototype 0.1

Fan-made prototype inspired by the mechanics of Five Nights at Freddy's 2.
This is an original implementation; it does not include copyrighted game art,
audio, characters' assets, or source code.

## What's implemented
- Bevy 0.15 project structure
- Stylized placeholder office scene
- HUD with night clock, flashlight charge and music-box meter
- Monitor toggle and five placeholder camera labels
- Mask and flashlight toggles
- Wind music-box control
- Basic timed animatronic activity messages
- Keyboard controls for desktop testing
- Touch-screen buttons and basic touch camera switching

## Requirements
- Rust toolchain (stable)
- For Android: Android Studio / Android SDK and NDK, plus the Rust Android target

## Run on desktop first
```bash
cargo run
```

## Android preparation
Install Android Studio and its SDK/NDK, then add the target matching your device:
```bash
rustup target add aarch64-linux-android
```

Install cargo-ndk:
```bash
cargo install cargo-ndk
```

Android packaging and Bevy's Android entry-point configuration can vary by
Bevy/toolchain version. Treat this scaffold as the gameplay prototype; the
next step is to add an Android Gradle wrapper/template and verify a device
build with the installed SDK/NDK.

## Controls
- Space: raise/lower monitor
- M: toggle Freddy mask
- F: flashlight
- Left / Right arrows: switch cameras
- W: wind music box
- Touch buttons: same core actions

## Project layout
```text
src/
  main.rs          platform entry point
  lib.rs           Bevy app setup
  game.rs          prototype gameplay state and systems
assets/
  images/
  sounds/
```

## Next milestones
1. Replace placeholder office shapes with original artwork.
2. Add camera-room visual overlays.
3. Implement distinct animatronic state machines.
4. Add audio cues and music-box danger states.
5. Add Android-specific build template and test on a real device.
