use std::{fs::File, io::Write};

use reqwest::{blocking::Client, StatusCode};

pub struct PageInfo {
    pub filename_template: fn(&str) -> String,
    pub url_template: fn(&str) -> String,
    pub prefix_start: u8,
    pub prefix_end_inclusive: u8,
    pub step: u8,
}

pub struct Downloader<'a> {
    client: &'a Client,
}

impl<'a> Downloader<'_> {
    pub fn new(client: &'a Client) -> Downloader<'a> {
        Downloader { client }
    }

    pub fn download(self, info: &PageInfo) {
        for prefix in
            generate_image_url_prefixes(info.prefix_start, info.prefix_end_inclusive, info.step)
        {
            let filename = (info.filename_template)(&prefix);
            let url = (info.url_template)(&filename);

            print!("Downloading {} ... ", url);
            {
                let rq = self.client.get(url).build().unwrap();
                let response = self.client.execute(rq).unwrap();
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
        }
    }
}

fn generate_image_url_prefixes(start: u8, end_inclusive: u8, step: u8) -> Vec<String> {
    (start..end_inclusive + 1)
        .step_by(step as usize)
        .map(|i| format!("{:03}", i))
        .collect()
}
