use reqwest;
use serde::Deserialize;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use clap::Parser;
use bytes::Bytes;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the board PNG to save
    #[clap(short, long, value_parser, default_value = "canvas.png")]
    path: String,
    #[clap(short = 'c', long, value_parser, default_value_t = false)]
    use_canvas_code: bool,
}


#[derive(Deserialize)]
struct PxlsInfoColor {
    value: String,
}

#[derive(Deserialize)]
struct PartialPxlsInfo {
    #[serde(rename = "canvasCode")]
    canvas_code: String,
    width: u32,
    height: u32,
    palette: Vec<PxlsInfoColor>,
}

async fn fetch_info(url: &String) -> Result<PartialPxlsInfo, reqwest::Error> {
    let info = reqwest::get(url)
        .await.expect("Unable to reqwest info")
        .json()
        .await.expect("Unable to deserialize info JSON");
    Ok(info)
}

async fn fetch_board_data(url: &String) -> Result<Bytes, reqwest::Error> {
    let board_data = reqwest::get(url)
        .await.expect("Unable to reqwest board data")
        .bytes()
        .await.expect("Unable to deserialize board data bytes");
    Ok(board_data)
}

fn palette_value_to_rgb(value: &String) -> (u8, u8, u8) {
    let trimmed: String = if value.starts_with("#") {
        value.strip_prefix("#").unwrap().to_owned()
    } else {
        value.to_string()
    };
    let color: u32 = u32::from_str_radix(trimmed.as_str(), 16).expect(format!("Unable to parse color hex: {}", trimmed).as_str());
    let [.., r, g, b] = u32::to_be_bytes(color);
    (r, g, b)
}

async fn map_board_data_palette(board_data: &Bytes, info: &PartialPxlsInfo) -> Vec<u8> {
    let mut mapped_board_data = vec![255; (info.width * info.height * 4).try_into().unwrap()];

    for (i, &color_index) in board_data.iter().enumerate() {
        if color_index == 255 {
            mapped_board_data[i * 4] = 0;
            mapped_board_data[i * 4 + 1] = 0;
            mapped_board_data[i * 4 + 2] = 0;
            mapped_board_data[i * 4 + 3] = 0;
            continue;
        }
        let hex_code = &info.palette.get(color_index as usize).unwrap().value;
        let (red, green, blue) = palette_value_to_rgb(&hex_code);
        mapped_board_data[i * 4] = red;
        mapped_board_data[i * 4 + 1] = green;
        mapped_board_data[i * 4 + 2] = blue;
    }

    mapped_board_data
}

fn make_canvas_png(mapped_board_data: &Vec<u8>, path: &str, info: &PartialPxlsInfo, args: &Args) {
    let mut path_to_use: &str = path;
    let path_replaced: &str = &path_to_use.replace(".png", format!("-{}.png", info.canvas_code).as_str());
    if args.use_canvas_code {
        path_to_use = path_replaced;
    }
    let file_path = Path::new(path_to_use);
    let file = File::create(file_path).expect(format!("Unable to create file at {}", path.to_owned()).as_str());
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&mapped_board_data).expect("Unable to write canvas PNG data");
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let info = fetch_info(&"https://pxls.space/info".to_owned())
        .await.expect("Unable to fetch info");

    let board_data = fetch_board_data(&"https://pxls.space/boarddata".to_owned())
        .await.expect("Unable to fetch board data");

    let mapped_board_data_palette = map_board_data_palette(&board_data, &info)
        .await;
    
    make_canvas_png(&mapped_board_data_palette, &args.path, &info, &args);
}
