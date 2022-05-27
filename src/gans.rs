//
// A place for all the image processing gans to live..
//
#![allow(non_snake_case)]
use chrono::Utc;
use std::process::Command;

//TODO: move these into the set current dir python calls
use crate::ESRGAN_PATH;
use crate::GFPGAN_PATH;

const UPSCALE_FACTOR: u64 = 4;

//
// ----------------------ESRGAN--------------------------
// pub enum ESRGANModel {
//     // Model options:
//     //     ESRGAN_SRx4_DF2KOST_official-ff704c30.pth
//     //     RealESRGAN_x2plus.pth  NOTE: Best for larger than 1000x1000 sizes
//     //     RealESRGAN_x2plus_netD.pth
//     //     RealESRGAN_x4plus.pth  NOTE: Best for smaller than 1000x1000 sizes
//     //     RealESRGAN_x4plus_anime_6B.pth NOTE: Best for line drawings
//     //     RealESRGAN_x4plus_netD.pth
//     //     RealESRNet_x4plus.pth
//     X2plus,
//     X4plus,
//     X4plusAnime,
// }

//TODO: ESRGAN has a new precompiled high-performance Vulkan version, update to that.
//https://github.com/xinntao/Real-ESRGAN-ncnn-vulkan/releases/tag/v0.2.0
// pub fn run_esrgan(model: ESRGANModel) -> anyhow::Result<()> {
//     // runs ESRGAN on the images in its path ../inputs/
//     /*Usage: realesrgan-ncnn-vulkan.exe -i infile -o outfile [options]...
//     -h                   show this help
//     -i input-path        input image path (jpg/png/webp) or directory
//     -o output-path       output image path (jpg/png/webp) or directory
//     -s scale             upscale ratio (can be 2, 3, 4. default=4)
//     -t tile-size         tile size (>=32/0=auto, default=0) can be 0,0,0 for multi-gpu
//     -m model-path        folder path to the pre-trained models. default=models
//     -n model-name        model name (default=realesr-animevideov3, can be realesr-animevideov3 | realesrgan-x4plus | realesrgan-x4plus-anime | realesrnet-x4plus)
//     -g gpu-id            gpu device to use (default=auto) can be 0,1,2 for multi-gpu
//     -j load:proc:save    thread count for load/proc/save (default=1:2:2) can be 1:2,2,2:2 for multi-gpu
//     -x                   enable tta mode"
//     -f format            output image format (jpg/png/webp, default=ext/png)
//     -v                   verbose output */
//     println!("{} QUEUED a superres.", Utc::now());
//     // ppython inference_realesrgan.py --model_path experiments/pretrained_models/RealESRGAN_x4plus.pth --input inputs --face_enhance

//     #[allow(unused_must_use)]
//     // as above, most of the models are not use, so we're silencing the compiler
//     let model = match model {
//         _X2plus => "RealESRGAN_x2plus.pth",
//         _X4plus => "RealESRGAN_x4plus.pth",
//         _X4plusAnime => "RealESRGAN_x4plus_anime_6B.pth",
//         _ => "RealESRNet_x4plus .pth",
//     };
//     let python = Command::new("python")
//         .current_dir(ESRGAN_PATH)
//         .arg("inference_realesrgan.py")
//         .arg("--model_path")
//         .arg(format!("experiments/pretrained_models/{}", model))
//         .arg("--input")
//         .arg("inputs")
//         .arg("--face_enhance") // TODO: work out whether or not a face is detected then run or disable this flag?
//         .status()?;
//     assert!(python.success());

//     Ok(())
// }

//
// ----------------------GFPGAN--------------------------
pub fn run_gfpgan(job: String) -> anyhow::Result<bool> {
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

    let job = format!("inputs/whole_imgs/{}", job);
    println!("{} QUEUED a restore.", Utc::now());
    let python = Command::new("python")
        .current_dir(GFPGAN_PATH)
        .arg("inference_gfpgan.py")
        .arg("-i") // input:
        .arg(job)
        .arg("-o") // output:
        .arg("results")
        .arg("-v") // version:
        .arg("1.3")
        .arg("-s") // Upscale:
        .arg(UPSCALE_FACTOR.to_string())
        .status()?;

    Ok(python.success())
}
