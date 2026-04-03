use std::{fmt, time::Duration};

use chrono::Local;
use iced::Font;
use iced::{
    Color, Element,
    Length::{self, Fill},
    Subscription, time,
    widget::{
        button, column, container,
        image::{Handle, viewer},
        row, text,
    },
};
use iced_aw::{ICED_AW_FONT_BYTES, selection_list::SelectionList, style::selection_list::primary};
use image::{GenericImageView, ImageFormat, ImageReader};
use rfd::FileDialog;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .font(ICED_AW_FONT_BYTES)
        .title("ImgForge")
        .run()
}

// --- Formats ---

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ExportImageFormats {
    Jpg,
    Png,
    Tiff,
    Bmp,
    WebP,
}

impl ExportImageFormats {
    fn all() -> Vec<Self> {
        vec![Self::Jpg, Self::Png, Self::Tiff, Self::Bmp, Self::WebP]
    }

    fn extension(&self) -> &str {
        match self {
            Self::Jpg => "jpg",
            Self::Png => "png",
            Self::Tiff => "tif",
            Self::Bmp => "bmp",
            Self::WebP => "webp",
        }
    }

    fn to_image_format(&self) -> ImageFormat {
        match self {
            Self::Jpg => ImageFormat::Jpeg,
            Self::Png => ImageFormat::Png,
            Self::Tiff => ImageFormat::Tiff,
            Self::Bmp => ImageFormat::Bmp,
            Self::WebP => ImageFormat::WebP,
        }
    }

    fn from_image_format(fmt: &ImageFormat) -> Option<Self> {
        match fmt {
            ImageFormat::Jpeg => Some(Self::Jpg),
            ImageFormat::Png => Some(Self::Png),
            ImageFormat::Tiff => Some(Self::Tiff),
            ImageFormat::Bmp => Some(Self::Bmp),
            ImageFormat::WebP => Some(Self::WebP),
            _ => None,
        }
    }
}

impl fmt::Display for ExportImageFormats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jpg => write!(f, "JPG"),
            Self::Png => write!(f, "PNG"),
            Self::Tiff => write!(f, "TIFF"),
            Self::Bmp => write!(f, "BMP"),
            Self::WebP => write!(f, "WebP"),
        }
    }
}

// --- App State ---

#[derive(Debug)]
struct App {
    image_path: Option<String>,
    image_format: Option<ImageFormat>,
    dimensions: Option<(u32, u32)>,
    time: String,
    available_formats: Vec<ExportImageFormats>,
    selected: Option<ExportImageFormats>,
    status: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            image_path: None,
            image_format: None,
            dimensions: None,
            time: String::new(),
            available_formats: vec![],
            selected: None,
            status: "Open an image to get started.".to_string(),
        }
    }
}

// --- Messages ---

#[derive(Clone, Debug)]
enum Operations {
    FilePick,
    Tick,
    ClearFilePick,
    FormatSelected(usize, ExportImageFormats),
    Export,
}

// --- App Logic ---

impl App {
    fn update_available_formats(&mut self) {
        let current = self
            .image_format
            .as_ref()
            .and_then(ExportImageFormats::from_image_format);

        self.available_formats = ExportImageFormats::all()
            .into_iter()
            .filter(|f| Some(f) != current.as_ref())
            .collect();
    }

    fn view(&self) -> Element<'_, Operations> {
        // Footer
        let footer_content = row![
            match &self.image_path {
                Some(_) => container(
                    text(format!(
                        "Format: {:?}{}",
                        self.image_format.unwrap(),
                        self.dimensions
                            .map(|(w, h)| format!("  |  {}x{}", w, h))
                            .unwrap_or_default()
                    ))
                    .width(Fill)
                ),
                None => container(text(&self.status).width(Fill)),
            },
            text(&self.time)
        ];

        // Left panel
        let left_panel = container(match &self.image_path {
            Some(img) => {
                let handle = Handle::from_path(img);
                container(
                    viewer(handle)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(iced::ContentFit::Contain)
                        .min_scale(1.0),
                )
                .padding(10.0)
            }
            None => container(
                column![
                    text("⚒").font(Font::MONOSPACE).size(80),
                    text("ImgForge").size(28),
                    text("Convert images between JPG, PNG, TIFF, BMP and WebP.").size(13),
                    text("").size(8),
                    button("Select an Image")
                        .padding(8)
                        .on_press(Operations::FilePick),
                ]
                .spacing(10)
                .align_x(iced::Alignment::Center),
            ),
        })
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .center(Length::FillPortion(2));

        // Right panel
        let right_panel = container(match &self.image_path {
            Some(_) => {
                let export_button = if self.selected.is_some() {
                    button("Export").padding(4).on_press(Operations::Export)
                } else {
                    button("Export").padding(4)
                };

                container(
                    column![
                        row![
                            button("Select another Image")
                                .padding(4)
                                .on_press(Operations::FilePick),
                            button("Clear")
                                .padding(4)
                                .on_press(Operations::ClearFilePick),
                        ]
                        .spacing(6)
                        .padding(3),
                        text("Export as:").size(13),
                        SelectionList::new_with(
                            &self.available_formats[..],
                            Operations::FormatSelected,
                            14.0,
                            8.0,
                            primary,
                            None,
                            Font::default(),
                        )
                        .width(Length::Fixed(150.0))
                        .height(Length::Fixed((self.available_formats.len() as f32) * 35.0,)),
                        match &self.selected {
                            Some(fmt) => text(format!("Selected: {}", fmt)).size(12),
                            None => text("No format selected").size(12),
                        },
                        export_button,
                        text(&self.status).size(12),
                    ]
                    .spacing(8),
                )
                .padding(10)
                .center_x(Length::FillPortion(1))
            }
            None => container(text("")),
        })
        .width(Length::FillPortion(1))
        .height(Length::Fill);

        let layout = row![left_panel, right_panel].height(Length::Fill);

        let footer = container(footer_content)
            .width(Fill)
            .padding(9)
            .style(|_theme| container::Style {
                background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
                text_color: Some(Color::WHITE.into()),
                ..Default::default()
            });

        column![layout, footer].into()
    }

    fn update(&mut self, op: Operations) {
        match op {
            Operations::FilePick => {
                let picked = FileDialog::new()
                    .add_filter(
                        "Images",
                        &["png", "jpg", "jpeg", "webp", "bmp", "tif", "tiff"],
                    )
                    .set_directory("~/Pictures")
                    .pick_file();

                if let Some(path) = picked {
                    match ImageReader::open(&path).and_then(|r| r.with_guessed_format()) {
                        Ok(reader) => {
                            self.image_format = reader.format();
                            match reader.decode() {
                                Ok(img) => {
                                    let (w, h) = img.dimensions();
                                    self.dimensions = Some((w, h));
                                    self.status = format!("Loaded: {}x{}", w, h);
                                }
                                Err(e) => self.status = format!("Decode error: {}", e),
                            }
                        }
                        Err(e) => self.status = format!("Open error: {}", e),
                    }
                    self.image_path = Some(path.to_string_lossy().to_string());
                    self.selected = None;
                    self.available_formats.clear();
                    self.update_available_formats();
                }
            }

            Operations::Tick => {
                self.time = Local::now().format("%b %d, %Y %I:%M:%S %p").to_string();
            }

            Operations::ClearFilePick => {
                self.image_path = None;
                self.image_format = None;
                self.dimensions = None;
                self.selected = None;
                self.available_formats.clear();
                self.status = "Open an image to get started.".to_string();
            }

            Operations::FormatSelected(_index, format) => {
                self.status = format!("Will export as {}", format);
                self.selected = Some(format);
            }

            Operations::Export => match (&self.image_path.clone(), &self.selected.clone()) {
                (Some(path), Some(format)) => {
                    let ext = format.extension();
                    let save_path = FileDialog::new()
                        .add_filter("Image", &[ext])
                        .set_file_name(format!("output.{}", ext))
                        .save_file();

                    if let Some(dest) = save_path {
                        match image::open(path) {
                            Ok(img) => {
                                match img.save_with_format(&dest, format.to_image_format()) {
                                    Ok(_) => {
                                        self.status = format!(
                                            "Exported to {}",
                                            dest.file_name()
                                                .and_then(|n| n.to_str())
                                                .unwrap_or("file")
                                        )
                                    }
                                    Err(e) => self.status = format!("Export failed: {}", e),
                                }
                            }
                            Err(e) => self.status = format!("Failed to open image: {}", e),
                        }
                    }
                }
                (None, _) => self.status = "No image loaded.".to_string(),
                (_, None) => self.status = "Please select a format first.".to_string(),
            },
        }
    }

    fn subscription(&self) -> Subscription<Operations> {
        time::every(Duration::from_secs(1)).map(|_| Operations::Tick)
    }
}

