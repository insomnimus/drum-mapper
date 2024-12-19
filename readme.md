# Drum-Mapper
This plugin and command line tool helps you remap MIDI drum tracks across popular drum sample libraries.

Simply insert it in your FX chain, tweak the parameters and you're done.

## Included Libraries
Trademark and copyright notice: These are 3rd party libraries. This project and it's authors are not associated with the respective companies nor are endorsed by them.

- Addictive Drums 2
- EzDrummer 2
- EzDrummer 3
- SSD 5
- Superior Drummer 3
- Ugritone Drums
- Getgood Drums - One Kit Wonder: Architects

You can embed your own mappings into the plugin/tool; instructions are given below.

## Installation
1. Download the archive per your operating system from [the releases page](https://github.com/insomnimus/drum-mapper/releases).
2. Extract the archive to some directory. It will contain 3 files/folders:
	- `drum-mapper` (`drum-mapper.exe` on windows): This is the command line tool. You can move it anywhere you'd like.
	- `Drum Mapper.vst3`: This is a directory and it's meant to be treated as a whole per VST3 specification. Move this directory into somewhere in your VST plugin path.
	- `Drum Mapper.clap`: This is a File on Linux and Windows, a directory on MacOS. Move it whole into your Clap plugin path.
3. Rescan your plugins on your DAW. The plugins will appear under the vendor "Insomnia".

## Building From Source
Requirements:
- An up to date rust toolchain.

### Building the Plugin (VST3 and Clap)
To build, you can run the build script on Linux/MacOS:

`./build.sh --release`

Or manually type:

`cargo run -q --bin bundler -- bundle drum-mapper --release`

The plugins will be written to `target/bundled`.

#### Tips
1. If you don't care about the plugin's GUI, you can simply disable it, reducing the executable size down to 30%. To disable the GUI:
	- `./build.sh --release --no-default-features`
	- Or, manually: `cargo run -q --bin bundler -- bundle drum-mapper --no-default-features`
2. If you don't intend to distribute the produced plugins to other machines, you can possibly get a little performance boost by specifying the target cpu as `native`, which can be done by running
	- (Bash, Zsh and similar shells): `RUSTFLAGS=-Ctarget-cpu=native ./build.sh --release`
	- Powershell: `$env:RUSTFLAGS = "-Ctarget-cpu=native"; cargo run -q --bin bundler -- bundle drum-mapper --release`

### Building the CLI Tool
Run:
`cargo build --release --bin drum-mapper`

The tool will be built into `target/release/drum-mapper`; on Windows, it will have a `.exe` extension as well.

## Plugin Usage
1. Insert it into a track with MIDI input. The plugin's author is `Insomnia` and the name is `Drum Mapper`.
2. Select the library to map from (this is the source). The parameter is called `from`.
3. Select a library to map to. The parameter is called `to`.
4. By default, only channel 10 (counting from 1) will be mapped (this is the drum channel on General MIDI). You can select a different channel or `all` to map all channels. The parameter is called `channel`.
5. The notes the plugin receives will be remapped. Every event except for Note On, note Off and Aftertouch are passed-through.

## Adding Custom Mappings
You can add your own mappings or remove any default mapping!
The files in the `drums` folder in the project's root with the `.txt` extension will be parsed as mappings.
The file name without the extension is used as the name of the library in the plugin and the CLI tool.

The format of the files is simple:
- Each line contains a mapping.
- Each mapping consists of `gm_note -> library_note`. Where `gm_note` is the note in General MIDI and `library_note` is the mapping of that note to the library.
- Empty Lines and lines starting with `#` are ignored.
- Lines not matching above are errors.

### Generating Mapping Files From Already Mapped MIDI Files
Do you use an external converter and want to integrate it into the plugin? Follow these steps.
1. First build the CLI tool as described above.
2. Generate a dummy MIDI file:
	`drum-mapper template -o dummy.mid`
3. Convert the generated `dummy.mid` with your external converter of choice.
4. Assuming the converted file is called `converted.mid`, run:
	`drum-mapper generate converted.mid -o "Foo Drums.txt"`
5. If the converter didn't add noise to the file, the above command should finish without any message.
6. Now, copy the generated "Foo Drums.txt" into the project's `drums` directory and follow the steps to build the plugin as described above. Your custom mapping will be integrated into the plugin!

## Remapping From The Command Line
The CLI tool has a command that lets you remap MIDI files from the command line.

The usage is:

`drum-mapper remap --from <library> --to <library> --out remapped.mid ./foo.mid`

It has more options, for example using a mapping file instead of embedded libraries for from/to. Run `drum-mapper remap --help` to see them all.
