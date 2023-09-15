# EmoteWheel
Emote Wheel POC made with rust + egui for NieR: Automata


This is based off of sy1ntexx's egui-d3d11 repo (https://github.com/sy1ntexx/egui-d3d11/) using their template.
It has no functionality, it only exists as an example of egui in d3d11.

## Installation
Install as a `LATE` load plugin for nier-mod-loader as per its instructions: https://github.com/xxk-i/Nier-Mod-Loader.

## Assets
The assets used by this mod can be found here: https://web.archive.org/web/20230915173344/https://cdn.discordapp.com/attachments/897325582583476265/1152296033624268892/assets.zip

## Building
Unfortunately, it seems the egui-d3d11 repo and respective crates have been taken down by the owner, so this cannot be built from source anymore. This repo does not contain egui-d3d11 files aside from a modified lib.rs from the example-wnd directory.

If you do have access to the egui-d3d11 repo (and its dependencies) you can download the assets from the link above, and save them with the name of their image description (with a .png extension) into the `assets/` directory. Then just `cargo build --release`
