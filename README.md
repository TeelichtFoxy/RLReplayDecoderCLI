-----

# rlrd: Rocket League Replay Decoder

**rlrd** (Rocket League Replay Decoder) is a powerful and easy-to-use command-line tool for Windows. It allows you to quickly decode all the data from your Rocket League `.replay` files and output it as a JSON.

-----

## Features

  * **Decode Replays:** Convert any `.replay` file into a detailed JSON output.
  * **Latest Replay Finder:** Easily locate the path to your most recent Rocket League replay. No more digging through folders\!
  * **Windows Native:** Built specifically for Windows users, leveraging standard Rocket League replay locations.

-----

## Installation

You can install `rlrd` directly from `github.com` using `git clone`:

```bash
git clone https://github.com/TeelichtFoxy/RLReplayDecoderCLI.git
```

Or just click [HERE](https://github.com/TeelichtFoxy/RLReplayDecoderCLI/releases/tag/rlrd) and download the latest `.exe`.

-----

## Usage

`rlrd` offers a straightforward command-line interface.

### Decoding a Specific Replay File

To decode a `.replay` file, use the `-r` or `--replay-file` flag, providing the path to your file. You can optionally specify an output path for the JSON using `-o` or `--output`.

```bash
./rlrd.exe -r "C:\Users\YourUser\Documents\My Games\Rocket League\TAGame\Demos\SomeAwesomeReplay.replay" -o "output.json"
```

### Finding Your Latest Replay

Want to quickly find your most recently played match? Use the `-g` or `--get-latest` flag:

```bash
./rlrd.exe -g
```

This will print the full path to your latest `.replay` file.

-----

## Examples

  * **Decode a replay and print to console:**

    ```bash
    ./rlrd.exe -r "C:\Users\Teelicht Foxy\Documents\My Games\Rocket League\TAGame\Demos\MyEpicSave.replay"
    ```

  * **Find the latest replay and then decode it (chained commands):**

    ```bash
    ./rlrd.exe -g
    # Copy the path from the output, then use it:
    ./rlrd.exe -r "C:\Users\Teelicht Foxy\Documents\My Games\Rocket League\TAGame\Demos\2025-05-30_22-00-00.replay" -o "latest_match_data.json"
    ```

-----

## Contributing

Feel free to open issues or pull requests if you have ideas for improvements, find bugs, or want to contribute to `rlrd`\!

-----

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](https://github.com/teelichtfoxy/rlreplaydecodercli/LICENSE) file for details.

-----
