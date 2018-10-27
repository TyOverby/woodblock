extern crate hist;
extern crate image;
extern crate imageproc;
use image::*;

/*
    let mut low = 0u8;
    let mut high = 255u8;

    for x in 0..image.width() {
        for y in 0..image.height() {
            low = min(low, image.get_pixel(x, y).data[0]);
            high = max(high, image.get_pixel(x, y).data[0]);
        }
    }
    let low = low as f64;
    let high = high as f64;
    let dist = high - low;
 */

fn find_peaks(v: &[u32]) -> Vec<(usize, u32)> {
    if v.len() == 0 {
        return vec![];
    }
    if v.len() == 1 {
        return vec![(0, v[0])];
    }

    let mut out = vec![];

    if v[0] > v[1] {
        out.push((0, v[0]));
    }

    for i in 1..(v.len() - 1) {
        if v[i] > v[i - 1] && v[i] > v[i + 1] {
            out.push((i, v[i]));
        }
    }

    if v[v.len() - 1] > v[v.len() - 2] {
        out.push((v.len() - 1, v[v.len() - 1]));
    }

    out.sort_by_key(|&(_, c)| c);
    out
}

fn extremize(image: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    use std::cmp::{max, min};
    let image = image.to_luma();

    let histo = imageproc::stats::histogram(&image);
    println!("{:?}", histo.iter().cloned().collect::<Vec<_>>());
    hist::Hist::new(
        256,
        30,
        &(0..256).collect(),
        &histo.iter().map(|&x| x as i32).collect(),
    ).display();

    let mut peaks = find_peaks(&histo);
    let a = peaks.pop().unwrap().0;
    let b = peaks.pop().unwrap().0;
    let (mut low, mut high) = if a < b { (a, b) } else { (b, a) };
    let d = high - low;
    low += d / 4;
    high -= d / 4;

    let low = low as f64;
    let high = high as f64;
    println!("lh : {:?}", (low, high));
    let dist = high - low;

    let mut img = ImageBuffer::new(image.width(), image.height());
    for x in 0..image.width() {
        for y in 0..image.height() {
            let mut v = image.get_pixel(x, y).data[0] as f64;
            if v < low {
                v = low
            }
            if v > high {
                v = high
            }
            let lowered = v - low;
            let downscaled = lowered / dist;
            let upscaled = downscaled * 255.0;
            img.put_pixel(
                x,
                y,
                Luma {
                    data: [upscaled as u8],
                },
            );
        }
    }

    let r = 3i64;
    for x in 0..(image.width() as i64) {
        for y in 0..(image.height() as i64) {
            let mut keep = false;
            'f: for xm in -r..r {
                for ym in -r..r {
                    let x = x + xm;
                    let y = y + ym;
                    if x < 0 || x > (image.width() as i64) || y < 0 || y > (image.height() as i64) {
                        continue;
                    }
                    if img.get_pixel(x as u32, y as u32).data[0] == 0 {
                        keep = true;
                        break 'f;
                    }
                }
            }
            if !keep {
                img.put_pixel(x as u32, y as u32, Luma { data: [255] });
            }
        }
    }

    return img;
}

fn main() {
    let img = open("/Users/tyoverby/Downloads/durer.jpg").unwrap();
    println!("w/h: {}/{}", img.width(), img.height());
    let ext = extremize(img);
    ext.save("/Users/tyoverby/Downloads/durer-ext.jpg").unwrap();
}
