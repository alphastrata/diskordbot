//
// A place for all the image processing gans to live..
//
#![allow(non_snake_case)]
use chrono::Utc;
use std::process::Command;

const UPSCALE_FACTOR: u64 = 4;
#[allow(dead_code)] // supressing compiler warnings from model's we're not currently using
pub enum Model {
    // Model options:
    //     ESRGAN_SRx4_DF2KOST_official-ff704c30.pth
    //     RealESRGAN_x2plus.pth  NOTE: Best for larger than 1000x1000 sizes
    //     RealESRGAN_x2plus_netD.pth
    //     RealESRGAN_x4plus.pth  NOTE: Best for smaller than 1000x1000 sizes
    //     RealESRGAN_x4plus_anime_6B.pth NOTE: Best for line drawings
    //     RealESRGAN_x4plus_netD.pth
    //     RealESRNet_x4plus.pth
    X2plus,
    X4plus,
    X4plusAnime,
}

pub fn run_esrgan(model: Model) -> anyhow::Result<()> {
    // runs ESRGAN on the images in its path ../inputs/
    println!("{} QUEUED a superres.", Utc::now());
    // should run equivalent to this:
    // ppython inference_realesrgan.py --model_path experiments/pretrained_models/RealESRGAN_x4plus.pth --input inputs --face_enhance

    #[allow(unused_must_use)]
    // as above, most of the models are not use, so we're silencing the compiler
    let model = match model {
        X2plus => "RealESRGAN_x2plus.pth",
        X4plus => "RealESRGAN_x4plus.pth",
        X4plusAnime => "RealESRGAN_x4plus_anime_6B.pth",
        _ => "RealESRNet_x4plus .pth",
    };
    let python = Command::new("python3.9")
        .current_dir("/home/jer/Documents/ESRGAN")
        .arg("inference_realesrgan.py")
        .arg("--model_path")
        .arg(format!("experiments/pretrained_models/{}", model))
        .arg("--input")
        .arg("inputs")
        .arg("--face_enhance") // TODO: work out whether or not a face is detected then run or disable this flag?
        .status()?;
    assert!(python.success());

    Ok(())
}

pub fn run_gfpgan() -> anyhow::Result<()> {
    // runs GFPGAN on the images in its path .../inputs/whole_images/
    println!("{} QUEUED a restore.", Utc::now());
    //python inference_gfpgan.py --upscale 2 --test_path inputs/whole_imgs --save_root results
    let python = Command::new("python3.9")
        .current_dir("/home/jer/Documents/GFPGAN")
        .arg("inference_gfpgan.py")
        .arg("--upscale")
        .arg(UPSCALE_FACTOR.to_string())
        .arg("--test_path")
        .arg("inputs/whole_imgs")
        .arg("--save_root")
        .arg("results")
        .status()?; // This lets us block until the Command is done .output() may be another more... information rich option

    assert!(python.success()); // NOTE: this won't crash the app
    Ok(())
}
