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
- [Get Rust](www.rust-lang.org).
- Clone _this_ repo.
- `cd diskordbot`.
- Check the paths are appropriate to wherever you installed GFPGAN to (currently hardcoded, sorry).
- make sure you have a token in your `PATH`, google this for your system.
- `cargo build --release`.
- `./target/release/diskortbot`.
- make sure it's invited to the appropriate server.
- make sure the appropriate user and channel IDs you can set these yourself in the `main.rs` file.

## TODO:

- [] the `impl` for `EventHandler` is starting to get a lil wild and sizey.
- [] set it up on audrey (USA Server)
- [] can the `build.rs` install GFPGAN for them?
- [] can this be dockerised? (to avoid having to do the above)
- [] what other superres/photo editing tooling can be added?
- [] currently two files sharing a filename will prevent the bot from downloading... edgecasey but shouldhandle.
- [] no progress feedback is given... edgecasey as the smaller, shittier, older images which this GFPGAN is actually good at working on take less than a second on my rig...
- [] need to implement a queue, currently spamming the bot will make it crash #cozpythoncozcuda
