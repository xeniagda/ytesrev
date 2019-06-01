//! The LaTeX renderer. This is quite low level, and you probably don't want to use this. Instead,
//! use [`LatexObj`], which contains a better, more high-level way to handle LaTeX in your
//! presentation.
//!
//! ---
//!
//! ## The rendereing process:
//!
//! 1. Collect all LaTeX expressions into a file, saved in /tmp/ytesrev/tmp.tex
//! 2. Run `pdflatex` on the file
//! 3. Run `pdfcrop` on to make all expressions the right size
//! 4. Run `pdftoppm` on the resulting `.pdf`-files to generate `.png`-files of all the expressions
//! 5. (Done for each `LatexObj`) Load the `.png`-file into a `PngImage`
//!
//! [`LatexObj`]: ../latex_obj/struct.LatexObj.html

extern crate sdl2;

use sdl2::pixels::Color;

use std::fs::{create_dir, remove_dir_all, File};
use std::io::{Error, ErrorKind, Result as IResult, Write};
use std::mem::drop;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::sync::Mutex;
use std::time::Instant;

use image::PngImage;

const LATEX_PRELUDE: &str = include_str!("latex_prelude.tex");
const LATEX_POSTLUDE: &str = "\\end{document}";

/// An error that might occur when rendering LaTeX expressions
#[derive(Debug, PartialEq)]
pub enum LatexError {
    /// The specified LaTeX expression wasn't registered. This error should be impossible to get,
    /// as to get it you need an invalid index. See [`LatexIdx`]
    ///
    /// [`LatexIdx`]: ../latex/latex_obj/struct.LatexObj.html
    NotExisting,
    /// The LaTeX document hasn't been rendered yet. Run the [`render_all_equations`]
    NotLoaded,
}

/// An index given to each [`LatexObj`], as they are all rendered in the same document
/// The only way to obtain an index is to register an equation using `register_equation`,
/// and as such, an invalid index should be impossible to obtain.
///
/// [`LatexObj`]: ../latex_obj/struct.LatexObj.html
pub struct LatexIdx(usize);

lazy_static! {
    static ref EQUATIONS: Mutex<Vec<(&'static str, bool, Option<PngImage>)>> =
        Mutex::new(Vec::new());
    static ref PRELUDE: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
}

/// Register an equation to be rendered. To render, use the [`render_all_equations`] method.
///
/// ```
/// use ytesrev::latex::render::*;
/// # fn make_invalid_idx() -> LatexIdx {
/// #   use std::mem::transmute;
/// #   unsafe { transmute::<usize, LatexIdx>(0) }
/// # }
/// let invalid_idx = make_invalid_idx(); // This is impossible to do, this is only for demonstration
/// assert_eq!(read_image(invalid_idx).err(), Some(LatexError::NotExisting));
///
/// let valid_idx = register_equation("a^2 + b^2 = c+2", false);
/// assert_eq!(read_image(valid_idx).err(), Some(LatexError::NotLoaded));
/// ```
pub fn register_equation(equation: &'static str, is_text: bool) -> LatexIdx {
    if let Ok(ref mut eqs) = EQUATIONS.lock() {
        let idx = eqs.len();
        eqs.push((equation, is_text, None));
        LatexIdx(idx)
    } else {
        panic!("Can't eqs");
    }
}

/// Add prelude to the LaTeX render.
///
/// ```
/// use ytesrev::latex::render::add_prelude;
///
/// add_prelude("\\usepackage{skull}");
/// ```
///
/// By default, amsmath is loaded, but nothing else.
///
pub fn add_prelude(prelude: &'static str) {
    if let Ok(ref mut preludes) = PRELUDE.lock() {
        preludes.push(prelude);
    }
    // TODO: Handle Mutex lock fail
}

/// Reads an image from an LatexIdx.
pub fn read_image(idx: LatexIdx) -> Result<PngImage, LatexError> {
    let res = if let Ok(ref mut eqs) = EQUATIONS.lock() {
        if let Some(ref mut x) = eqs.get_mut(idx.0) {
            if x.2.is_some() {
                Ok(x.2.take().unwrap())
            } else {
                Err(LatexError::NotLoaded)
            }
        } else {
            Err(LatexError::NotExisting)
        }
    } else {
        Err(LatexError::NotLoaded)
    };
    drop(idx);
    res
}

/// Run the rendering process. This takes a few seconds.
///
/// As with everything in this module, you probably don't want to do this yourself as this is
/// automatically handled by the [`WindowManager`].
///
/// [`WindowManager`]: ../../window/struct.WindowManager.html
pub fn render_all_equations() -> IResult<()> {
    if let Ok(eqs) = EQUATIONS.lock() {
        if eqs.len() == 0 {
            return Ok(());
        }
    }
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

    let mut raw_path = path.clone();
    raw_path.push("tmp-res");

    let start = Instant::now();

    create_tex(&tex_path)?;

    render_tex(&tex_path, &pdf_path, &raw_path)?;

    read_pngs(&path)?;

    let diff = Instant::now() - start;
    eprintln!("Rendering took {:.2?}", diff);

    Ok(())
}

fn create_tex(tex_path: &Path) -> IResult<()> {
    let mut tex_file = File::create(tex_path)?;
    let mut added_prelude = String::new();
    if let Ok(prelude) = PRELUDE.lock() {
        prelude.iter().for_each(|prelude| {
            added_prelude.push_str(prelude);
            added_prelude.push('\n');
        });
    }

    writeln!(
        tex_file,
        "{}",
        LATEX_PRELUDE.replace("$PRELUDE", &added_prelude)
    )?;

    if let Ok(eqs) = EQUATIONS.lock() {
        for equation in eqs.iter() {
            for col in &["red", "blue"] {
                writeln!(tex_file, "\\begin{{equation*}}")?;
                writeln!(tex_file, "\\colorbox{{{}}}{{\\makebox[\\linewidth]{{", col)?;
                if equation.1 {
                    writeln!(tex_file, "{}", equation.0)?;
                } else {
                    writeln!(tex_file, "$ {} $", equation.0)?;
                }
                writeln!(tex_file, "}} }}")?;
                writeln!(tex_file, "\\end{{equation*}}")?;
            }
        }
    }

    writeln!(tex_file, "{}", LATEX_POSTLUDE)?;

    Ok(())
}

fn render_tex(tex_path: &Path, pdf_path: &Path, raw_path: &Path) -> IResult<()> {
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

    let out = Command::new("pdftoppm")
        .arg(pdf_path.as_os_str())
        .arg(raw_path.as_os_str())
        .arg("-r")
        .arg("250")
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
        let digits_max = format!("{}", eqs.len()).len();

        for (i, (_, _, ref mut im)) in eqs.iter_mut().enumerate() {
            let num_red = zero_pad(format!("{}", 2 * i + 1), digits_max);
            let num_blue = zero_pad(format!("{}", 2 * i + 2), digits_max);

            let mut img_path_red = path.to_path_buf();
            img_path_red.push(format!("tmp-res-{}.png", num_red));

            let mut img_path_blue = path.to_path_buf();
            img_path_blue.push(format!("tmp-res-{}.png", num_blue));

            let mut im_red_res = PngImage::load_from_path(File::open(img_path_red)?)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

            let im_blue = PngImage::load_from_path(File::open(img_path_blue)?)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

            let mut maxx = 0;
            let mut maxy = 0;
            let mut minx = im_red_res.width;
            let mut miny = im_red_res.height;

            for i in 0..im_red_res.width * im_red_res.height {
                let x = i % im_red_res.width;
                let y = i / im_red_res.width;

                let rr = im_red_res.data[4 * i];
                let rg = im_red_res.data[4 * i];
                let rb = im_red_res.data[4 * i + 2];

                let br = im_blue.data[4 * i];
                let bb = im_blue.data[4 * i + 2];

                let rdiff = rr as i16 - br as i16;
                let bdiff = bb as i16 - rb as i16;


                let alpha = 255 - (rdiff + bdiff) / 2;
                let alpha = alpha.min(255).max(0) as u8;


                let alpha_prec = (rdiff + bdiff) as f64 / 512.;

                im_red_res.data[4 * i] = br;
                im_red_res.data[4 * i + 2] = rb;
                im_red_res.data[4 * i + 3] = alpha;

                if (br < 250 || rg < 250 || rb < 250) && alpha > 250 {
                    maxx = maxx.max(x + 1);
                    maxy = maxy.max(y + 1);

                    minx = minx.min(x);
                    miny = miny.min(y);
                }
            }
            // Margins
            maxx = (maxx + 3).min(im_red_res.width - 1);
            maxy = (maxy + 3).min(im_red_res.height - 1);
            minx = minx.saturating_sub(3);
            miny = miny.saturating_sub(3);

            let width = maxx - minx;
            let height = maxy - miny;
            let mut resdata = vec![0; 4 * width * height];

            for x in 0..width {
                for y in 0..height {
                    let i_r = y * width + x;
                    let i_l = (y + miny) * im_red_res.width + x + minx;

                    resdata[4 * i_r] = im_red_res.data[4 * i_l];
                    resdata[4 * i_r + 1] = im_red_res.data[4 * i_l + 1];
                    resdata[4 * i_r + 2] = im_red_res.data[4 * i_l + 2];
                    resdata[4 * i_r + 3] = im_red_res.data[4 * i_l + 3];
                }
            }

            *im = Some(PngImage {
                data: resdata,
                width,
                height,
            });
        }
    }
    Ok(())
}

fn zero_pad(n: String, len: usize) -> String {
    let needed = len.saturating_sub(n.len());
    let mut res = (0..needed).map(|_| '0').collect::<String>();
    res.push_str(&n);
    res
}
