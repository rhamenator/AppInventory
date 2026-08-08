#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Application {
    pub name: String,
    pub version: String,
    pub publisher: String,
    pub size: String,
    pub installed_on: String,
}

pub fn decode(bytes: &[u8]) -> Result<String, String> {
    if bytes.starts_with(&[0xff, 0xfe]) {
        if !(bytes.len() - 2).is_multiple_of(2) {
            return Err("truncated UTF-16LE input".into());
        }
        let units = bytes[2..]
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>();
        String::from_utf16(&units).map_err(|error| error.to_string())
    } else if bytes.starts_with(&[0xfe, 0xff]) {
        if !(bytes.len() - 2).is_multiple_of(2) {
            return Err("truncated UTF-16BE input".into());
        }
        let units = bytes[2..]
            .chunks_exact(2)
            .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>();
        String::from_utf16(&units).map_err(|error| error.to_string())
    } else {
        String::from_utf8(bytes.to_vec()).map_err(|error| error.to_string())
    }
}

pub fn parse(input: &str) -> Vec<Application> {
    let mut applications = Vec::new();
    let mut current = Application::default();
    for line in input.lines().chain(std::iter::once(
        "----------------------------------------------",
    )) {
        let line = line.trim();
        if line.starts_with('-') && line.len() >= 8 {
            if !current.name.is_empty() {
                applications.push(std::mem::take(&mut current));
            }
            continue;
        }
        let Some((label, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim().to_owned();
        match label.trim().to_ascii_lowercase().as_str() {
            "software name" | "name" => current.name = value,
            "version" => current.version = value,
            "publisher" => current.publisher = value,
            "size" => current.size = value,
            "install time" | "installed on" => current.installed_on = value,
            _ => {}
        }
    }
    applications.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
    });
    applications
}

pub fn to_csv(applications: &[Application]) -> String {
    let mut output = "Software Name,Version,Publisher,Size,Install Time\n".to_owned();
    for app in applications {
        output.push_str(
            &[
                &app.name,
                &app.version,
                &app.publisher,
                &app.size,
                &app.installed_on,
            ]
            .map(|value| csv_cell(value))
            .join(","),
        );
        output.push('\n');
    }
    output
}

fn csv_cell(value: &str) -> String {
    if value
        .chars()
        .any(|character| matches!(character, ',' | '"' | '\n' | '\r'))
    {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_handles_missing_fields_and_csv_escaping() {
        let apps = parse(
            "Software Name: Tool, Pro\nPublisher: Example\n--------\nSoftware Name: Alpha\nVersion: 1\n",
        );
        assert_eq!(apps.len(), 2);
        let csv = to_csv(&apps);
        assert!(csv.contains("\"Tool, Pro\""));
        assert!(csv.lines().nth(1).unwrap().starts_with("Alpha,"));
    }

    #[test]
    fn decoder_accepts_utf16le_bom() {
        let mut bytes = vec![0xff, 0xfe];
        for unit in "Software Name: Demo".encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        assert!(decode(&bytes).unwrap().contains("Demo"));
    }
}
