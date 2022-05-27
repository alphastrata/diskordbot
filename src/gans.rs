//
// A place for all the image processing gans to live..
//
#![allow(non_snake_case)]
use chrono::Utc;
use std::process::Command;

use crate::ESRGAN_EXECUTABLE;
use crate::ESRGAN_PATH;
use crate::GFPGAN_PATH;

const UPSCALE_FACTOR: u64 = 4;

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

    let output = format!("results");
    let job = format!("inputs/whole_imgs/{}", job);
    println!("{} QUEUED a restore.", Utc::now());
    let python = Command::new("python")
        .current_dir(GFPGAN_PATH)
        .arg("inference_gfpgan.py")
        .arg("-i") // input:
        .arg(job)
        .arg("-o") // output:
        .arg(output)
        .arg("-v") // version:
        .arg("1.3")
        .arg("-s") // Upscale:
        .arg(UPSCALE_FACTOR.to_string())
        .status()?;

    Ok(python.success())
}
pub fn run_esrgan(job: String) -> anyhow::Result<bool> {
    /*Usage: realesrgan-ncnn-vulkan.exe -i infile -o outfile [options]...

      -h                   show this help
      -i input-path        input image path (jpg/png/webp) or directory
      -o output-path       output image path (jpg/png/webp) or directory
      -s scale             upscale ratio (can be 2, 3, 4. default=4)
      -t tile-size         tile size (>=32/0=auto, default=0) can be 0,0,0 for multi-gpu
      -m model-path        folder path to the pre-trained models. default=models
      -n model-name        model name (default=realesr-animevideov3, can be realesr-animevideov3 | realesrgan-x4plus | realesrgan-x4plus-anime | realesrnet-x4plus)
      -g gpu-id            gpu device to use (default=auto) can be 0,1,2 for multi-gpu
      -j load:proc:save    thread count for load/proc/save (default=1:2:2) can be 1:2,2,2:2 for multi-gpu
      -x                   enable tta mode"
      -f format            output image format (jpg/png/webp, default=ext/png)
      -v                   verbose output
    */

    //NOTE: This is using the GFPGAN's input folder, which is why the end up needing the extra . at the front
    let output = format!(".{}results/restored_imgs/{}", GFPGAN_PATH, job);
    let job = format!(".{}inputs/whole_imgs/{}", GFPGAN_PATH, job);

    let esrgan_vulkan = Command::new(ESRGAN_EXECUTABLE)
        .current_dir(ESRGAN_PATH)
        .arg("-i")
        .arg(job)
        .arg("-o")
        .arg(output)
        .arg("-s")
        .arg(UPSCALE_FACTOR.to_string())
        .arg("-v")
        .status()?;

    Ok(esrgan_vulkan.success())
}
