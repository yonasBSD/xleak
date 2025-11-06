use anyhow::{Context, Result};
use calamine::{Data, Range, Reader, Sheets, open_workbook_auto};
use chrono::{NaiveDate, Duration};
use std::path::Path;

pub struct Workbook {
    sheets: Sheets<std::io::BufReader<std::fs::File>>,
}

impl Workbook {
    /// Open an Excel file (auto-detects format)
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let sheets = open_workbook_auto(path.as_ref()).context("Failed to open workbook")?;

        Ok(Self { sheets })
    }

    /// Get all sheet names
    pub fn sheet_names(&self) -> Vec<String> {
        self.sheets.sheet_names()
    }

    /// Load a specific sheet by name
    pub fn load_sheet(&mut self, name: &str) -> Result<SheetData> {
        let range = self
            .sheets
            .worksheet_range(name)
            .with_context(|| format!("Sheet '{name}' not found"))?;

        Ok(SheetData::from_range(range))
    }
}

#[derive(Debug, Clone)]
pub struct SheetData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<CellValue>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub enum CellValue {
    Empty,
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Error(String),
    DateTime(f64), // Excel datetime as float
}

impl CellValue {
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        matches!(self, CellValue::Empty)
    }

    #[allow(dead_code)]
    pub fn is_numeric(&self) -> bool {
        matches!(self, CellValue::Int(_) | CellValue::Float(_))
    }
}

impl std::fmt::Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellValue::Empty => write!(f, ""),
            CellValue::String(s) => write!(f, "{s}"),
            CellValue::Int(i) => {
                // Format integers with thousand separators
                let s = i.to_string();
                let negative = s.starts_with('-');
                let digits: String = s.trim_start_matches('-').chars().collect();
                let mut result = String::new();
                for (idx, ch) in digits.chars().rev().enumerate() {
                    if idx > 0 && idx % 3 == 0 {
                        result.push(',');
                    }
                    result.push(ch);
                }
                if negative {
                    result.push('-');
                }
                write!(f, "{}", result.chars().rev().collect::<String>())
            }
            CellValue::Float(val) => {
                // Format floats with thousand separators
                let formatted = if val.fract() == 0.0 {
                    format!("{val:.0}")
                } else {
                    format!("{val:.2}")
                };
                let parts: Vec<&str> = formatted.split('.').collect();
                let int_part = parts[0];
                let negative = int_part.starts_with('-');
                let digits: String = int_part.trim_start_matches('-').chars().collect();
                let mut result = String::new();
                for (idx, ch) in digits.chars().rev().enumerate() {
                    if idx > 0 && idx % 3 == 0 {
                        result.push(',');
                    }
                    result.push(ch);
                }
                if negative {
                    result.push('-');
                }
                let int_formatted: String = result.chars().rev().collect();
                if parts.len() > 1 {
                    write!(f, "{}.{}", int_formatted, parts[1])
                } else {
                    write!(f, "{}", int_formatted)
                }
            }
            CellValue::Bool(b) => {
                // Use lowercase for booleans
                write!(f, "{}", if *b { "true" } else { "false" })
            }
            CellValue::Error(e) => write!(f, "ERROR: {e}"),
            CellValue::DateTime(d) => {
                // Excel dates are days since December 30, 1899 (day 0)
                // Excel has a leap year bug where 1900 is incorrectly treated as a leap year
                // Days > 60 need adjustment for this bug
                let days = d.floor() as i64;

                // Excel epoch: December 30, 1899
                let excel_epoch = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap();

                // Adjust for Excel's 1900 leap year bug (day 60 = Feb 29, 1900 which didn't exist)
                let adjusted_days = if days > 60 { days - 1 } else { days };

                if let Some(date) = excel_epoch.checked_add_signed(Duration::days(adjusted_days)) {
                    // Check if there's a time component
                    let frac = d.fract();
                    if frac.abs() > 0.000001 {
                        // Has time component
                        let total_seconds = (frac * 86400.0) as u32;
                        let hours = total_seconds / 3600;
                        let minutes = (total_seconds % 3600) / 60;
                        let seconds = total_seconds % 60;
                        write!(f, "{} {:02}:{:02}:{:02}", date, hours, minutes, seconds)
                    } else {
                        // Date only
                        write!(f, "{}", date)
                    }
                } else {
                    write!(f, "Date[{days}]")
                }
            }
        }
    }
}

impl SheetData {
    pub fn from_range(range: Range<Data>) -> Self {
        let (height, width) = range.get_size();

        // Extract headers from first row if it exists
        let headers = if height > 0 {
            range
                .rows()
                .next()
                .map(|row| row.iter().map(Self::cell_to_string).collect())
                .unwrap_or_default()
        } else {
            vec![]
        };

        // Extract data rows (skip first row as headers)
        let rows: Vec<Vec<CellValue>> = range
            .rows()
            .skip(1)
            .map(|row| row.iter().map(Self::datatype_to_cellvalue).collect())
            .collect();

        Self {
            headers,
            rows,
            width,
            height: height.saturating_sub(1), // Don't count header row
        }
    }

    fn cell_to_string(cell: &Data) -> String {
        match cell {
            Data::Empty => String::new(),
            Data::String(s) => s.clone(),
            Data::Int(i) => i.to_string(),
            Data::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{f:.0}")
                } else {
                    f.to_string()
                }
            }
            Data::Bool(b) => b.to_string(),
            Data::Error(e) => format!("ERROR: {e:?}"),
            Data::DateTime(d) => format!("Date({})", d.as_f64()),
            Data::DateTimeIso(s) => s.clone(),
            Data::DurationIso(s) => s.clone(),
        }
    }

    fn datatype_to_cellvalue(cell: &Data) -> CellValue {
        match cell {
            Data::Empty => CellValue::Empty,
            Data::String(s) => CellValue::String(s.clone()),
            Data::Int(i) => CellValue::Int(*i),
            Data::Float(f) => CellValue::Float(*f),
            Data::Bool(b) => CellValue::Bool(*b),
            Data::Error(e) => CellValue::Error(format!("{e:?}")),
            Data::DateTime(d) => CellValue::DateTime(d.as_f64()),
            Data::DateTimeIso(s) => CellValue::String(s.clone()),
            Data::DurationIso(s) => CellValue::String(s.clone()),
        }
    }
}
