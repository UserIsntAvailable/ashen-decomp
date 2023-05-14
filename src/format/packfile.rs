use std::str;

use super::packfile_entry::PackFileEntry;
use super::traits::*;

#[derive(Debug)]
pub struct PackFile {
    pub copyright: String,
    pub entries: Vec<PackFileEntry>,
}

impl AssetLoad for PackFile {
    type Data = ();

    fn load(bytes: &[u8], _: Self::Data) -> Result<(Self, usize), DataError>
    where
        Self: Sized,
    {
        let mut offset = 0usize;

        // PMAN signature
        let pman = match read_part::<4>(bytes, &mut offset) {
            Ok(pman) => match str::from_utf8(pman.0) {
                Ok(pman) => pman.to_string(),
                Err(error) => {
                    return Err(DataError {
                        file_type: Some(Self::file_type()),
                        section: Some("PMAN signature".to_string()),
                        offset: Some(offset + error.valid_up_to()),
                        actual: Box::new(format!("{:?}", pman.1)),
                        expected: ExpectedData::Other {
                            description: "To be a valid UTF-8 string".to_string(),
                        },
                    })
                }
            },
            Err(error) => {
                let mut error = DataError::from(error);
                error.file_type = Some(Self::file_type());
                error.section = Some("PMAN signature".to_string());
                return Err(error);
            }
        };
        if pman != "PMAN" {
            return Err(DataError {
                file_type: Some(Self::file_type()),
                section: Some("PMAN signature".to_string()),
                offset: Some(offset),
                actual: Box::new(pman),
                expected: ExpectedData::Equal { value: Box::new(0) },
            });
        }

        // Number of entries
        let entries = match read_part::<4>(bytes, &mut offset) {
            Ok(entries) => u32::from_le_bytes(*entries.0),
            Err(error) => {
                let mut error = DataError::from(error);
                error.file_type = Some(Self::file_type());
                error.section = Some("Offset".to_string());
                return Err(error);
            }
        };

        // Copyright
        let copyright = match read_part::<56>(bytes, &mut offset) {
            Ok(copyright) => match str::from_utf8(copyright.0) {
                Ok(copyright) => copyright.to_string(),
                Err(error) => {
                    return Err(DataError {
                        file_type: Some(Self::file_type()),
                        section: Some("PMAN signature".to_string()),
                        offset: Some(offset + error.valid_up_to()),
                        actual: Box::new(format!("{:?}", copyright.1)),
                        expected: ExpectedData::Other {
                            description: "To be a valid UTF-8 string".to_string(),
                        },
                    })
                }
            },
            Err(error) => {
                let mut error = DataError::from(error);
                error.file_type = Some(Self::file_type());
                error.section = Some("PMAN signature".to_string());
                return Err(error);
            }
        };

        // Entries information
        let entries: Vec<_> = (0..entries)
            .map(|_| match PackFileEntry::load(&bytes[offset..], ()) {
                Ok(entry) => {
                    offset += entry.1;
                    Ok(entry.0)
                }
                Err(mut error) => {
                    if let Some(offset_error) = &mut error.offset {
                        *offset_error += offset;
                    }
                    Err(error)
                }
            })
            .collect::<Result<_, DataError>>()?;

        Ok((Self { copyright, entries }, offset))
    }

    fn file_type() -> String {
        "PackFile".to_string()
    }
}
