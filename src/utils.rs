use crate::gans;
use chrono::Utc;
use serenity::client::Context;
use serenity::model::channel::Attachment;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

// TODO: mutex this bad boy and set it to on when we're running a gan, if the .output() call on the gan returns a 0 we succeded, if the file has been returned we can flip this back over
pub struct WorkHandle {
    pub(crate) available: Arc<Mutex<bool>>,
    pub(crate) worklist: Arc<Mutex<Vec<String>>>,
}

impl WorkHandle {
    /// Initialises the work handle, with an empty worklist and availability set to true
    pub fn init() -> WorkHandle {
        let mut available = Arc::new(Mutex::new(true));
        let mut worklist = Arc::new(Mutex::new(Vec::new()));
        WorkHandle {
            available,
            worklist,
        }
    }

    fn check_all_work_completed(&mut self) -> bool {
        let worklist = self.worklist.lock().unwrap();
        let availability = *self.available.lock().unwrap();
        if worklist.len() == 0 && availability {
            return true;
        }
        println!("Worklist len: {:?}", worklist.len());
        println!("Availability: {:?}", availability);
        false
    }
}

#[allow(non_snake_case)]
pub async fn remote_kill_triggered(message: &Message, GFPGAN_BOT_ID: &u64, context: &Context) {
    if message.content.contains(" !terminate")
        && message.content.contains(&GFPGAN_BOT_ID.to_string())
    {
        println!(
            "{} REMOTE PANIC TRIGGERED BY: {} WITH COMMAND {}: ",
            Utc::now(),
            message.author,
            message.content
        );
        let _ = message.reply_mention(&context, " You killed me...").await;
        panic!();
    }
}
#[allow(non_snake_case)] // Coz it complains about the const variables -- which by convention are uppercase?
pub async fn run(
    message: &Message,
    context: &Context,
    GFPGAN_BOT_ID: &u64,
    MAXIMUM_INPUT_RESOLUTION: u64,
    GFPGAN_PATH: &str,
    ESRGAN_PATH: &str,
    workhandle: &mut WorkHandle,
) {
    for attachment in &message.attachments {
        let filename = attachment.filename.clone();

        // Confirm we're working somewhere we're supposed to be
        if check_permissions(message, *GFPGAN_BOT_ID).unwrap_or(false) {
            // Confirm we're dealing with and image, and of a suitably small size
            let ismedia = attachment_is_image(&*attachment).unwrap_or(false); // or false because .txt files aren't media amongst others
            assert_eq!(ismedia, true);

            if attachment.width > Some(MAXIMUM_INPUT_RESOLUTION)
                || attachment.height > Some(MAXIMUM_INPUT_RESOLUTION)
            {
                println!("{} Image is already pretty high res, skipping.", Utc::now());
                let _ = message.reply_mention(
                    &context,
                    format!(
                        "`{}` is already pretty big, there's probably nothing I can do to make it better.\nTry a worse quality image.",
                        filename
                    ),
                ).await;
                return;
            };

            // Download files n get to work
            let _ = match attachment.download().await {
                Ok(content) => {
                    let photo = format!("{}inputs/whole_imgs/{}", GFPGAN_PATH, filename);

                    // Actual download
                    if check_attachment_and_download(message, &photo, &filename, &content)
                        .unwrap_or(false)
                    {
                        // Acknowledge download
                        let _ = message
                            .reply_mention(
                                &context,
                                format!(
                                    "I have your file `{}` and will return it in about 10 seconds.\n
                                    Please note, for 'superres' calls that if you don't have Nitro I'll be unable to send your file back!",
                                    filename
                                ),
                            )
                            .await;
                        // NOTE: explicitly creating a scope here to allow the compiler to handle unlock for us
                        {
                            workhandle.worklist.lock().unwrap().push(filename.clone());
                            *workhandle.available.lock().unwrap() = false;
                            //DEBUG:
                            println!(
                                "{} Jobs in Queue: {}",
                                Utc::now(),
                                workhandle.worklist.lock().unwrap().len()
                            );
                        }
                        //
                        //NOTE CLEAN THIS UP! into a match with pretty calls?
                        if message.content.contains("superres") {
                            let top_job = workhandle.worklist.lock().unwrap().pop().unwrap();
                            if gans::run_esrgan(top_job).expect("Failed to run ESRGAN") {
                                *workhandle.available.lock().unwrap() = true;
                                workhandle.worklist.lock().unwrap().pop();
                            }
                        }

                        if message.content.contains("restore") {
                            let top_job = workhandle.worklist.lock().unwrap().pop().unwrap();
                            if gans::run_gfpgan(top_job).expect("Failed to run GFPGAN") {
                                *workhandle.available.lock().unwrap() = true;
                                workhandle.worklist.lock().unwrap().pop();
                            }
                        } else {
                            return;
                        };
                        if *workhandle.available.lock().unwrap() {
                            let restored_imgs =
                                format!("{}results/restored_imgs/{}", GFPGAN_PATH, filename);

                            // Get it back to them
                            return_file(
                                vec![&restored_imgs[..]], // NOTE: this is pretty nasty ...
                                message.channel_id,
                                &filename,
                                context,
                                message,
                                GFPGAN_PATH,
                                ESRGAN_PATH,
                                workhandle,
                            )
                            .await;
                        }
                    }
                    if message.content.contains("restore") {
                        gans::run_gfpgan(filename.clone()).expect("Failed to run GFPGAN");
                    } else {
                        return;
                    };

                    let restored_imgs =
                        format!("{}results/restored_imgs/{}", GFPGAN_PATH, filename);

                    // Get it back to them
                    return_file(
                        vec![&restored_imgs[..]],
                        message.channel_id,
                        &filename,
                        context,
                        message,
                        GFPGAN_PATH,
                        ESRGAN_PATH,
                        workhandle,
                    )
                    .await;
                }

                // Communicate failure to download files
                Err(why) => {
                    println!("Error downloading attachment: {:?}", why);
                    let _ = message.channel_id.say(&context, "Error downloading").await;
                }
            };
        }
    }
}

#[allow(non_snake_case)]
fn check_permissions(message: &Message, GFPGAN_BOT_ID: u64) -> anyhow::Result<bool> {
    // returns true if the bot should be posting
    if message.author.id == GFPGAN_BOT_ID {
        println!("{} Skipping! this is either a channel I'm not authorised to post to or.. I posted this :P", Utc::now(),);
        return Ok(false);
    };
    Ok(true)
}

fn attachment_is_image(attachment: &Attachment) -> anyhow::Result<bool> {
    if let Some(media_type) = &attachment.content_type {
        println!("{} MediaType: {}", Utc::now(), media_type,);
        return Ok(true);
    }
    Ok(false)
}

fn check_attachment_and_download(
    message: &Message,
    photo: &str,
    filename: &String,
    content: &Vec<u8>,
) -> anyhow::Result<bool> {
    // returns true if the file was downloaded, false if the file already existing on disk
    // we do not run if the file already exists
    if Path::new(&photo).is_file() {
        println!(
            "{} Duplicate, skipping as I already have this image {:#?}",
            Utc::now(),
            filename
        );
        return Ok(false);
    } else {
        let _ = fs::write(photo, content.clone())?;
        println!();
        println!("{} DOWNLOADED:{:#?}", Utc::now(), filename);
        println!("{} USER:{}", Utc::now(), message.author);
        println!("{} CONTENT:{}", Utc::now(), message.content);
        // Let the user know we've downloaded their file
    }
    Ok(true)
}

#[allow(non_snake_case)]
fn cleanup(model_path: &str, gan: &str, workhandle: &mut WorkHandle) -> anyhow::Result<()> {
    //NOTE: clearing out the worklist will need to happen before this cleanup is called
    //TODO:
    //check_worklist_is_empty && avaiablity is true
    // cleanup by removing all downloaded files
    if workhandle.check_all_work_completed() {
        match gan {
            "ESR" => {
                let inputs = format!("{}inputs/inputs/", model_path);
                let results = format!("{}results/results/", model_path);
                let _ = remove_all(inputs);
                let _ = remove_all(results);
            }
            "GFP" => {
                let inputs = format!("{}inputs/whole_imgs", model_path);
                let results = format!("{}results/restored_imgs", model_path);
                let _ = remove_all(inputs);
                let _ = remove_all(results);
            }
            _ => (),
        };
    }

    Ok(())
}

fn remove_all(dir: String) -> anyhow::Result<()> {
    let _ = fs::read_dir(&dir)?
        .into_iter()
        .filter_map(|e| e.ok()) // Should also check filetypes??...
        .map(|e| e.path())
        .try_for_each(|p| fs::remove_file(p));

    println!("{} CLEANED UP: {}", Utc::now(), dir);
    Ok(())
}

#[allow(non_snake_case)]
async fn return_file(
    paths: Vec<&str>,
    channel_id: ChannelId,
    filename: &str,
    context: &Context,
    message: &Message,
    GFPGAN_PATH: &str,
    ESRGAN_PATH: &str,
    workhandle: &mut WorkHandle,
) {
    // return the user's uploaded file to them restored
    let photo = format!("{}results/restored_imgs/{}", GFPGAN_PATH, filename);
    println!("{} Awaiting script to finish work...", Utc::now());

    if std::path::Path::new(&photo).is_file() {
        let _response = channel_id
            .send_files(&context, paths, |m| {
                m.content(format!("restored_{}", filename))
            })
            .await;
        println!("{} FINISHED! response sent!\n", Utc::now());
        if message.content.contains("superres") {
            let _ = cleanup(ESRGAN_PATH, "ESER", workhandle);
        } else {
            let _ = cleanup(GFPGAN_PATH, "GFP", workhandle);
        }
    } else {
        println!("{} File was not ready...", Utc::now());

        std::thread::sleep(std::time::Duration::from_secs(5)); // #BADHAX

        let _ = return_file(
            paths,
            message.channel_id,
            filename,
            context,
            message,
            GFPGAN_PATH,
            ESRGAN_PATH,
            workhandle,
        );
    }
}

pub(crate) fn read_token_txt(p: String) -> Result<String, std::io::Error> {
    let contents = match fs::read_to_string(p) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    Ok(contents)
}
