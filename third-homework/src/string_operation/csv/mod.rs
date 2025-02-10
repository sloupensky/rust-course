use std::error::Error;
use std::fmt;
use tabular::{Table, Row};

pub fn tabify_string_from_csv(string: String, separator: String) -> Result<String, Box<dyn Error>> {
    Ok(format!("{}", Csv::from_string(string, &separator)?))
}

struct Csv {
    header: CsvHeader,
    data_lines: Vec<CsvLine>,
}

struct CsvLine {
    columns: Vec<String>,
}

struct CsvHeader {
    columns: Vec<String>,
}

impl Csv {
    fn from_string(string: String, separator: &String) -> Result<Csv, Box<dyn Error>> {
        let mut string_collection = string.as_str().lines().enumerate();
        let header: CsvHeader;
        let mut data_lines: Vec<CsvLine> = vec![];

        if string_collection.clone().count() <= 1 {
            return Err(format!("Invalid CSV string: {}", string).into());
        }

        let (_, header_line) = &string_collection.next().unwrap();

        header = CsvHeader::from_string(header_line.to_string(), separator);

        for item in string_collection {
            let (_, line) = item;
            let csv_line = CsvLine::from_string(line.to_string(), separator);

            if header.columns.len() != csv_line.columns.len() {
                return Err(format!("Invalid column count in line columns, differs from header.").into());
            }

            data_lines.push(csv_line);
        }

        Ok(Csv{header, data_lines})
    }

    fn get_table_pattern(&self) -> String {
        let mut string: String = "".to_string();

        for _item in &self.header.columns {
            let _ = &string.push_str( "  {:<}  ");
        }

        string
    }
}

impl fmt::Display for Csv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new(&self.get_table_pattern());
        let mut row = Row::new();

        for header_column in self.header.columns.iter() {
            row = row.with_cell(header_column);
        }

        table.add_row(row);

        for line in &self.data_lines {
            let mut row = Row::new();

            for column in &line.columns {
                row = row.with_cell(column);
            }

            table.add_row(row);
        }

        write!(f, "{}", table)?;

        Ok(())
    }

}

impl CsvLine {
    fn from_string(line: String, separator: &String) -> Self {
        CsvLine{columns: line.split(separator.as_str()).map(|s| s.to_string()).collect()}
    }
}

impl CsvHeader {
    fn from_string(line: String, separator: &String) -> Self {
        CsvHeader{columns: line.split(separator.as_str()).map(|s| s.to_string()).collect()}
    }
}