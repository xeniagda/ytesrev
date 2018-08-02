extern crate sdl2;

use sdl2::pixels::Color;

use std::sync::Mutex;
use std::process::{Command, exit};
use std::fs::{File, create_dir, remove_dir_all};
use std::path::{Path, PathBuf};
use std::io::{Result as IResult, Error, ErrorKind, Write};
use std::time::Instant;

use image::PngImage;

const LATEX_PRELUDE:  &str = include_str!("latex_prelude.tex");
const LATEX_POSTLUDE: &str = "\\end{document}";

#[derive(Debug)]
pub enum LatexError {
    NotExist,
    NotLoaded,
}

pub struct LatexIdx(usize);

impl LatexIdx {
    pub fn get_id(&self) -> usize { self.0 }
}

lazy_static! {
    static ref EQUATIONS: Mutex<Vec<(&'static str, Option<PngImage>)>> = Mutex::new(Vec::new());
}

pub fn register_equation(equation: &'static str) -> LatexIdx {
    if let Ok(ref mut eqs) = EQUATIONS.lock() {
        let idx = eqs.len();
        eqs.push((equation, None));
        LatexIdx(idx)
    } else {
        panic!("Can't eqs");
    }
}

pub fn read_image(idx: LatexIdx) -> Result<PngImage, LatexError> {
    if let Ok(ref mut eqs) = EQUATIONS.lock() {
        if let Some(ref mut x) = eqs.get_mut(idx.0) {
            if x.1.is_some() {
                Ok(x.1.take().unwrap())
            } else {
                Err(LatexError::NotLoaded)
            }
        } else {
            Err(LatexError::NotExist)
        }
    } else {
        Err(LatexError::NotLoaded)
    }
}

pub fn render_all_eqations() -> IResult<()> {
    let mut path = PathBuf::new();
    path.push("/tmp/ytesrev");

    if path.exists() {
        remove_dir_all(path.clone())?;
    }
    create_dir(path.clone())?;

    let mut tex_path = path.clone();
    tex_path.push("tmp.tex");

    let mut pdf_path = path.clone();
    pdf_path.push("tmp.pdf");

    let mut crop_path = path.clone();
    crop_path.push("tmp-crop.pdf");

    let mut raw_path = path.clone();
    raw_path.push("tmp-crop");

    let start = Instant::now();

    create_tex(&tex_path)?;

    render_tex(&tex_path, &pdf_path, &crop_path, &raw_path)?;

    read_pngs(&path)?;

    let diff = Instant::now() - start;
    eprintln!("Rendering took {:.2?}", diff);

    Ok(())
}

fn create_tex(tex_path: &Path) -> IResult<()> {

    let mut tex_file = File::create(tex_path)?;
    writeln!(tex_file, "{}", LATEX_PRELUDE)?;


    if let Ok(eqs) = EQUATIONS.lock() {
        for ref equation in eqs.iter() {
            writeln!(tex_file, "\\begin{{equation*}}")?;
            writeln!(tex_file, "{}", equation.0)?;
            writeln!(tex_file, "\\end{{equation*}}")?;
        }
    }

    writeln!(tex_file, "{}", LATEX_POSTLUDE)?;

    Ok(())
}

fn render_tex(tex_path: &Path, pdf_path: &Path, crop_path: &Path, raw_path: &Path) -> IResult<()> {
    let out = Command::new("pdflatex")
            .current_dir(tex_path.parent().unwrap())
            .arg(tex_path.file_name().unwrap())
            .output()
            .expect("Can't make command");

    if !out.status.success() {
        eprintln!("Latex compile error:");
        eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        exit(1);
    }

    let out = Command::new("pdfcrop")
            .arg(pdf_path.as_os_str())
            .arg(crop_path.as_os_str())
            .output()
            .expect("Can't make command");

    if !out.status.success() {
        eprintln!("pdfcrop error:");
        eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        exit(1);
    }

    let out = Command::new("pdftoppm")
            .arg(crop_path.as_os_str())
            .arg(raw_path.as_os_str())
            .arg("-r")
            .arg("300")
            .arg("-png")
            .output()
            .expect("Can't make command");

    if !out.status.success() {
        eprintln!("pdftoppm error");
        eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        exit(1);
    }

    Ok(())
}

fn read_pngs(path: &Path) -> IResult<()> {
    if let Ok(ref mut eqs) = EQUATIONS.lock() {
        for (i, (_, ref mut im)) in eqs.iter_mut().enumerate() {
            let mut img_path = path.to_path_buf();
            img_path.push(format!("tmp-crop-{}.png", i + 1));

            *im = Some(
                PngImage::load_from_path_transform(
                    File::open(img_path)?,
                    white_transparent)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))
                ?);
        }
    }
    Ok(())
}

fn white_transparent(col: Color) -> Color {
    let max_channel = col.r.min(col.g).min(col.b);
    Color { r: col.r, g: col.g, b: col.b, a: 255 - max_channel }
}
