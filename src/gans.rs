//
// A place for all the image processing gans to live..
//
#![allow(non_snake_case)]
use chrono::Utc;
use std::process::Command;

use crate::GFPGAN_PATH;

const UPSCALE_FACTOR: u64 = 4;

// ----------------------GFPGAN--------------------------
pub fn run_gfpgan() -> anyhow::Result<()> {
    // runs GFPGAN on the images in path GFPGANPATH/inputs/whole_images which is where the bot will automatically download them to
    /*  Usage: python inference_gfpgan.py -i inputs/whole_imgs -o results -v 1.3 -s 2 [options]...
    -h                   show this help
    -i input             Input image or folder. Default: inputs/whole_imgs
    -o output            Output folder. Default: results
    -v version           GFPGAN model version. Option: 1 | 1.2 | 1.3. Default: 1.3
    -s upscale           The final upsampling scale of the image. Default: 2
    -bg_upsampler        background upsampler. Default: realesrgan
    -bg_tile             Tile size for background sampler, 0 for no tile during testing. Default: 400
    -suffix              Suffix of the restored faces
    -only_center_face    Only restore the center face
    -aligned             Input are aligned faces
    -ext                 Image extension. Options: auto | jpg | png, auto means using the same extension as inputs. Default: auto
    */
    println!("{} QUEUED a restore.", Utc::now());
    let python = Command::new("python")
        .current_dir(GFPGAN_PATH)
        .arg("inference_gfpgan.py")
        .arg("-i") // input:
        .arg("inputs/whole_imgs")
        .arg("-o") // output:
        .arg("results")
        .arg("-v") // version:
        .arg("1.3")
        .arg("-s") // Upscale:
        .arg(UPSCALE_FACTOR.to_string())
        .status()?; // This lets us block until the Command is done .output() may be another more... information rich option
    assert!(python.success()); // NOTE: this won't crash the app
    Ok(())
}
