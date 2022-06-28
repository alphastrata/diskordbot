//
//╭━━━┳━━━┳━━━┳━━━┳━━━┳━╮╱╭^ ┳╮╱╱╱╱╭╮
//┃╭━╮┃╭━━┫╭━╮┃╭━╮┃╭━╮┃┃╰╮┃| ┃┃╱╱╱╭╯╰╮
//┃┃╱╰┫╰━━┫╰━╯┃┃╱╰┫┃╱┃┃╭╮╰╯|/┃╰━┳━┻╮╭╯
//┃┃╭━┫╭━━┫╭━━┫┃╭━┫╰━╯┃┃╰╮┃|/┃╭╮┃╭╮┃┃
//┃╰┻━┃┃╱╱┃┃╱╱┃╰┻━┃╭━╮┃┃╱┃┃|/┃╰╯┃╰╯┃╰╮
//╰━━━┻╯╱╱╰╯╱╱╰━━━┻╯╱╰┻╯╱╰━|/┻━━┻━━┻━╯
//
A discord bot for GFPGAN

## USAGE:

- Install GFPGAN, [LMGTFY](https://lmgtfy.app/?q=gfpgan).
- [Get Rust](www.rust-lang.org).
- Clone this repo.
- `cd diskordbot`.
- Check the paths are appropriate to wherever you installed GFPGAN to (currently hardcoded, sorry).
- `cargo build --release`.
- `./target/release/diskortbot`.
- make sure it's invited to the appropriate server.
- make sure the appropriate user and channel IDs are set (currently hardcoded, sorry).
- make sure you have a token in your `PATH`... not even going to post a LMGTFY this time.

## TODO:

- [] the `impl` for `EventHandler` is starting to get a lil wild and sizey.
- [] set it up on audrey (USA Server)
- [] can the `build.rs` install GFPGAN for them?
- [] can this be dockerised? (to avoid having to do the above)
- [] what other superres/photo editing tooling can be added?
- [] currently two files sharing a filename will prevent the bot from downloading... edgecasey but shouldhandle.
- [] no progress feedback is given... edgecasey as the smaller, shittier, older images which this GFPGAN is actually good at working on take less than a second on my rig...
- [] need to implement a queue, currently spamming the bot will make it crash #cozpythoncozcuda
- [] error handling is... non-existant.
- [] can this be broken out into something like app logic -> match (all the capablilities...)
