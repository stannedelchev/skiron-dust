mod downloader;

use crate::downloader::Downloader;
use reqwest::redirect::Policy;

fn main() {
    // Make an HTTP client that treats 301 and 302 results as errors.
    // Those redirect back to https://forecast.uoa.gr/en/forecast-maps/dust/europe instead of returning an image.
    let client = reqwest::blocking::Client::builder()
        .redirect(Policy::none())
        .build()
        .unwrap();

    let dust_concentration = Downloader::new(&client);
    let dust_load = Downloader::new(&client);

    dust_concentration.download(&downloader::PageInfo {
        filename_template: |prefix| format!("{}.zoomdconc.png", prefix),
        url_template: |filename| {
            format!(
                "https://forecast.uoa.gr/maps/0day/DUST/GRID1/zoomdconc/{}",
                filename
            )
        },
        prefix_start: 12,
        prefix_end_inclusive: 204,
        step: 6,
    });

    dust_load.download(&downloader::PageInfo {
        filename_template: |prefix| format!("{}.zoomdload.png", prefix),
        url_template: |filename| {
            format!(
                "https://forecast.uoa.gr/maps/0day/DUST/GRID1/zoomdload/{}",
                filename
            )
        },
        prefix_start: 12,
        prefix_end_inclusive: 204,
        step: 6,
    });
}
