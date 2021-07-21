# vlc-rs [![Build Status](https://travis-ci.org/garkimasera/vlc-rs.svg?branch=master)](https://travis-ci.org/garkimasera/vlc-rs)

Rust bindings for libVLC media framework.

## Status

Many missing functions and wrappers.

## Use

Please add the following dependencies to your Cargo.toml.

```Toml
[dependencies]
vlc-rs = "0.3"
```

Or:

```Toml
[dependencies.vlc-rs]
git = "https://github.com/garkimasera/vlc-rs.git"
```

## Example

Play for 10 seconds from a media file.

```Rust
extern crate vlc;
use vlc::{Instance, Media, MediaPlayer};
use std::thread;

fn main() {
    // Create an instance
    let instance = Instance::new().unwrap();
    // Create a media from a file
    let md = Media::new_path(&instance, "path_to_a_media_file.ogg").unwrap();
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();

    // Wait for 10 seconds
    thread::sleep(::std::time::Duration::from_secs(10));
}
```

Other examples are in the examples directory.

## Building

### Windows

To build `vlc-rs`, you must either build VLC from source or grab one of the pre-built packages from [videolan.org](https://www.videolan.org/vlc/download-windows.html).

If you're building for `x86_64`, then you should grab the download labelled "Installer for 64bit version".
That installer is actually a self-extracting ZIP archive, so we can extract the contents without installing VLC itself.

If you're building for `x86`, then you should either download labelled "7zip package" or the one labelled "Zip package".

Once you've downloaded your chosen package, you should extract it some place such that its path contains no spaces.
To point `vlc-rs` at your VLC package, you should set an appropriate environment variable:

- `VLC_LIB_DIR`: Directory of the VLC pacakge, any architecture
- `VLC_LIB_DIR_X86` : Directory of the VLC pacakge, `x86`-only
- `VLC_LIB_DIR_X86_64` : Directory of the VLC pacakge, `x86_64`-only

You should also add the package to your `PATH` variable if you intend to run the program.
For distribution of an executable program, you should probably copy over the neccessary DLLs, as well as the `plugins` directory.

## License

MIT (Examples are licensed under CC0)
