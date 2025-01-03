mod args;
mod gif_combine;

use crate::args::Args;
use crate::gif_combine::GifCombine;
use clap::Parser;
use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use std::fs::File;
use std::io::{Error, Write};

macro_rules! generate_urls {
    ($file_template: expr, $url_template: expr) => {{
        let start = 12;
        let end_inclusive = 204;
        let step = 6;
        (start..=end_inclusive)
            .step_by(step as usize)
            .map(|i| {
                let file = format!("{:03}.{}", i, $file_template);
                let url = format!($url_template, file);
                (file, url)
            })
            .collect::<Vec<_>>()
    }};
}

fn main() {
    let args = Args::parse();

    // Make an HTTP client that treats 301 and 302 results as errors.
    // Those redirect back to https://forecast.uoa.gr/en/forecast-maps/dust/europe instead of returning an image.
    let client = reqwest::blocking::Client::builder()
        .redirect(Policy::none())
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    if let Some(output) = args.dust_load {
        download(
            &client,
            generate_urls!(
                "zoomdload.png",
                "https://forecast.uoa.gr/maps/0day/DUST/GRID1/zoomdload/{}"
            ),
            &output,
            args.fps,
            args.save_intermediate,
        );
    }

    if let Some(output) = args.dust_concentration {
        download(
            &client,
            generate_urls!(
                "zoomdconc.png",
                "https://forecast.uoa.gr/maps/0day/DUST/GRID1/zoomdconc/{}"
            ),
            &output,
            args.fps,
            args.save_intermediate,
        );
    }
}

fn download(
    client: &Client,
    urls: Vec<(String, String)>,
    output_path: &str,
    fps: u8,
    save_intermediate: bool,
) {
    let output_file = File::create(&output_path).expect("Unable to create output GIF file");

    let mut combiner = GifCombine::new(fps, output_file);

    for (file, url) in urls {
        println!("Downloading {} ...", url);
        let bytes = download_bytes(&client, &url).expect("Unable to download frame");

        if save_intermediate {
            save_frame(&file, &bytes).expect("Unable to write to frame output file");
        }
        println!("Inserting frame into {}", output_path);
        combiner.add_frame(&bytes).expect("Unable to add frame");
    }
}

fn save_frame(file: &str, bytes: &bytes::Bytes) -> Result<(), Error> {
    println!("Saving intermediate frame to {}", file);
    let mut output_file = File::create(file)?;
    output_file.write_all(&bytes)
}

fn download_bytes(client: &Client, url: &str) -> Result<bytes::Bytes, reqwest::Error> {
    let request = client.get(url).build()?;
    client.execute(request)?.error_for_status()?.bytes()
}
