use core::panic;
use reqwest::{redirect::Policy, StatusCode};
use std::{fs::File, io::Write};

fn main() {
    // Make an HTTP client that treats 301 and 302 results as errors.
    // Those redirect back to https://forecast.uoa.gr/en/forecast-maps/dust/europe instead of returning an image.
    let client = reqwest::blocking::Client::builder()
        .redirect(Policy::none())
        .build()
        .unwrap();

    for prefix in generate_image_url_prefixes(12, 204, 6) {
        let filename = format!("{}.zoomdconc.png", prefix);
        let url = format!(
            "https://forecast.uoa.gr/maps/0day/DUST/GRID1/zoomdconc/{}",
            filename
        );

        print!("Downloading {} ... ", url);
        {
            let rq = client.get(url).build().unwrap();
            let response = client.execute(rq).unwrap();
            match response {
                r if r.status() == StatusCode::OK => {
                    let image_bytes = r.bytes().unwrap();
                    let mut file = File::create(filename).unwrap();
                    file.write_all(&image_bytes).unwrap();
                }
                r => {
                    println!("FAILED");
                    panic!("{:?}", r);
                }
            }
        }
        println!("OK");
    }
}

fn generate_image_url_prefixes(start: u8, end_inclusive: u8, step: u8) -> Vec<String> {
    (start..end_inclusive + 1)
        .step_by(step as usize)
        .map(|i| format!("{:03}", i))
        .collect()
}
