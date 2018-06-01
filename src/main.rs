mod bool_properties;
mod general_category;

use std::{io::Write as IoWrite, fmt::Write as FmtWrite, fs::File, path::PathBuf, error::Error};

fn main() -> Result<(), Box<Error>> {
    let rules = bool_properties::BY_NAME.iter()
        .chain(general_category::BY_NAME.iter())
        .collect::<Vec<_>>();

    for (name, ranges) in rules {
        let mut buffer = String::new();
        if ranges.len() > 0 {
            writeln!(buffer, "{} =", name)?;
            writeln!(buffer, "  {{ '{}'..'{}'", ranges[0].0, ranges[0].1)?;
            for range in &ranges[1..] {
                writeln!(buffer, "  | '{}'..'{}'", range.0, range.1)?;
            }
            writeln!(buffer, "  }}")?;
        } else {
            writeln!(buffer, "{} = {{}}", name)?;
        }

        let mut path = PathBuf::new();
        path.push("pest");
        path.push(name);
        path.set_extension("pest");
        let mut file = File::create(path)?;
        writeln!(file, "{}", buffer)?;
    }

    Ok(())
}
