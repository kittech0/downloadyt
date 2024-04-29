#![feature(async_closure)]

use clap::Parser;
use futures::future::join_all;
use indicatif::ProgressBar;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rustube::Video;

use crate::error::{BoxResult, NotFoundVideoError};

mod error;

/// Multi-thread YouTube download
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Options {
    /// Requires calling per every YouTube video, uses id of the video.
    #[arg(short, long)]
    video: Vec<String>,
    /// Worst quality
    #[arg(short, long)]
    worst_quality: bool,
    /// Output path
    #[arg(short, long, default_value = ".")]
    output: String,
}

#[tokio::main]
async fn main() -> BoxResult<()> {
    let args = Options::parse();

    if args.video.is_empty() {
        return Err("No videos.".into());
    }

    download_all(args.output, args.worst_quality, args.video).await
}

async fn download_all(path: String, worst_quality: bool, videos: Vec<String>) -> BoxResult<()> {
    let pb = ProgressBar::new(videos.len() as u64);
    pb.inc(0);
    let tasks = videos
        .into_par_iter()
        .map(|video_id| download(worst_quality, video_id, path.clone(), pb.clone()))
        .collect::<Vec<_>>();

    join_all(tasks).await;
    pb.finish_with_message("Downloaded all videos");
    Ok(())
}

async fn download(
    worst_quality: bool,
    video_id: String,
    dir: String,
    progress_bar: ProgressBar,
) -> BoxResult<()> {
    let video = Video::from_id(rustube::Id::from_string(video_id.clone())?).await?;
    let name = video
        .video_info()
        .player_response
        .video_details
        .title
        .clone();

    if worst_quality {
        video
            .worst_quality()
            .ok_or(NotFoundVideoError {})?
            .download_to_dir(dir)
            .await?;
    } else {
        video
            .best_quality()
            .ok_or(NotFoundVideoError {})?
            .download_to_dir(dir)
            .await?;
    }

    progress_bar.inc(1);
    progress_bar.println(format!("Downloaded video: {name}"));
    Ok(())
}
