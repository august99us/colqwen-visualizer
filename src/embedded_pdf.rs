use std::{env, fs::{self, File}};

use cairo::{Context, Format, ImageSurface};
use camino::Utf8PathBuf;
use candle_core::{Tensor, DType, Device};
use image::{DynamicImage, ImageReader};
use poppler::Document;

pub struct EmbeddedPdf {
    images: Vec<DynamicImage>,
    embedding: Tensor,
}

pub fn generate_from_pdf(path: Utf8PathBuf, save_folder: &str) -> Result<EmbeddedPdf, anyhow::Error> {
    // Load the PDF file
    let pdf_name = match path.file_name() {
        Some(s) => s,
        None => return Err(anyhow::Error::msg("given path does not have a file name"))
    };
    let pdf = Document::from_file(path.as_str(), None)?;

    // Create the directory to save the converted images
    match fs::create_dir(save_folder) {
        Ok(_) => println!("Pdf image save directory {} created successfully", save_folder),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            println!("Pdf image save directory {} already exists", save_folder)
        }
        Err(e) => return Err(anyhow::Error::new(e)),
    };
    let pdf_folder = Utf8PathBuf::try_from(env::current_dir()?.join(save_folder))?;

    let mut images = Vec::new();
    for i in 0..pdf.n_pages() {
        let page = pdf.page(i).expect("Unexpected error, index out of bounds when selecting page maybe?");
        let (width, height) = page.size();
        let surface = ImageSurface::create(
            Format::ARgb32,
            width as i32,
            height as i32,
        )?;
        let context = Context::new(&surface)?;
        page.render(&context);

        let page_image_filepath = format!("{}/{}_page_{}.png", pdf_folder.as_str(), pdf_name, i);
        let mut image_file = File::create(page_image_filepath)?;
        surface.write_to_png(&mut image_file)?;
        surface.finish();

        images.push(ImageReader::open("page_image_filepath")?.decode()?);
    }

    Ok(EmbeddedPdf {
        images,
        embedding: Tensor::zeros(&[0, 1], DType::BF16, &Device::Cpu)? // Placeholder for the embedding
    })
}