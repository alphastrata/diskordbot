```
     ___     __            __        __
 ___/ (_)__ / /_____  ____/ /  ___  / /_
/ _  / (_-</  '_/ _ \/ __/ _ \/ _ \/ __/
\_,_/_/___/_/\_\\___/_/ /_.__/\___/\__/

```

A discord bot for GFPGAN, and other image editing GANS.

## USAGE:

- Install [GFPGAN](https://lmgtfy.app/?q=gfpgan).
- Install Discord (if you don't have it already), you'll also need a token to get your bot into servers, you can google this.
- Install [ESRGAN](https://github.com/xinntao/Real-ESRGAN) You can download executable files for Intel/AMD/Nvidia GPU: <a href="https://github.com/xinntao/Real-ESRGAN/releases/download/v0.2.5.0/realesrgan-ncnn-vulkan-20220424-windows.zip">Windows</a>, <a href="https://github.com/xinntao/Real-ESRGAN/releases/download/v0.2.5.0/realesrgan-ncnn-vulkan-20220424-ubuntu.zip">Linux</a>, <a href="https://github.com/xinntao/Real-ESRGAN/releases/download/v0.2.5.0/realesrgan-ncnn-vulkan-20220424-macos.zip">MacOS</a>

- [Get Rust](www.rust-lang.org).
- Clone _this_ repo.
- `cd diskordbot`.
- `touch client_secret.txt` and paste your token into it.
- Check the paths are appropriate to wherever you installed GFPGAN to, if you clone them into the `diskordbot` dir everything will be done for you.
- make sure you have a token in your newly created `client_secret.txt`.
- `cargo build --release`.
- `./target/release/diskortbot`.
- make sure the bot is invited to the appropriate server(s).
- if you need to change the bot's ID to suit your deployment see the top of the `main.rs` file.

## TODO:

- [] the `impl` for `EventHandler` is starting to get a lil wild and sizey.
- [] can the `build.rs` install GFPGAN and `wget` the new ESRGAN for them?
- [] currently two files sharing a filename will prevent the bot from downloading... edgecasey but shouldhandle.
- [] no progress feedback is given... edgecasey as the smaller, shittier, older images which this GFPGAN is actually good at working on take less than a second on my rig...
