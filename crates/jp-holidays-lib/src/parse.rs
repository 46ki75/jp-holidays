//! CSV parsing for the Cabinet Office holiday data.

use chrono::NaiveDate;
use std::collections::BTreeMap;

/// Parses a UTF-8 CSV string in the Cabinet Office format into a sorted map of
/// holidays.
///
/// The first line is treated as a header and skipped, and blank lines are
/// ignored. Each remaining line is expected to be `YYYY/M/D,name`.
pub(crate) fn parse_csv(csv: &str) -> Result<BTreeMap<NaiveDate, String>, crate::error::Error> {
    csv.lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.trim().splitn(2, ',');
            let date_str = parts
                .next()
                .ok_or_else(|| crate::error::Error::MalformedCsv("missing date column".into()))?;
            let name_str = parts
                .next()
                .ok_or_else(|| crate::error::Error::MalformedCsv("missing name column".into()))?;

            let date = NaiveDate::parse_from_str(date_str.trim(), "%Y/%m/%d")?;

            Ok((date, name_str.trim().to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_well_formed_csv() {
        let csv = "国民の祝日・休日月日,国民の祝日・休日名称\n\
            1955/1/1,元日\n\
            1955/1/15,成人の日\n\
            1955/3/21,春分の日\n";

        let map = parse_csv(csv).unwrap();

        assert_eq!(map.len(), 3);
        assert_eq!(
            map.get(&NaiveDate::from_ymd_opt(1955, 1, 1).unwrap()),
            Some(&"元日".to_string())
        );
        assert_eq!(
            map.get(&NaiveDate::from_ymd_opt(1955, 3, 21).unwrap()),
            Some(&"春分の日".to_string())
        );
    }

    #[test]
    fn tolerates_crlf_and_blank_lines() {
        let csv = "header,header\r\n1955/1/1,元日\r\n\r\n";
        let map = parse_csv(csv).unwrap();
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn rejects_bad_date() {
        let csv = "header,header\n1955/13/40,元日\n";
        assert!(parse_csv(csv).is_err());
    }
}
