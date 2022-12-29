// Std
use std::env;
use std::error::Error;
use std::fs::File;
use std::process;

// External
use chrono::NaiveDate;

// Some common enum vals
const STATE_OR_REGION: &'static [&'static str] = &[
    "AL", "AK", "AS", "AZ", "AR", "CA", "CO", "CT", "DE", "DC", "FL", "GA", "GU", "HI", "ID", "IL",
    "IN", "IA", "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH",
    "NJ", "NM", "NY", "NC", "ND", "MP", "OH", "OK", "OR", "PA", "PR", "RI", "SC", "SD", "TN", "TX",
    "UT", "VT", "VI", "VA", "WA", "WV", "WI", "WY",
];

const ACCEPTED_FILE_TYPES: &'static [&'static str] = &[
    "csv", "json", "xml", "xlsx", "zip/csv", "zip/json", "zip/xml", "zip/xlsx", "other",
];

// Allowable column types
enum ColType {
    Enum(&'static [&'static str]),
    NaiveDate,
    Other, // Generic/unspecified case
}

// Column metadata
struct DataCol {
    name: &'static str,
    required: bool,
    col_type: ColType,
}

fn get_schema(schema_name: &str) -> Result<&[DataCol], Box<dyn Error>> {
    let cols: &[DataCol];

    match schema_name {
        "hospital_price_transparency" => {
            // Columns
            cols = &[
                DataCol {
                    name: "ccn",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "reporting_entity_name_legal",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "reporting_entity_name_common",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "reporting_entity_type",
                    required: false,
                    col_type: ColType::Enum(&["hospital", "other"]),
                },
                DataCol {
                    name: "machine_readable_url",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "machine_readable_url_status",
                    required: false,
                    col_type: ColType::Enum(&["up", "down", "corrupt"]),
                },
                DataCol {
                    name: "machine_readable_page",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "supplemental_url",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "file_name",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "file_format",
                    required: false,
                    col_type: ColType::Enum(ACCEPTED_FILE_TYPES),
                },
                DataCol {
                    name: "file_size",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "meets_standard",
                    required: false,
                    col_type: ColType::Enum(&["true", "false"]),
                },
                DataCol {
                    name: "standard_issue",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "state_or_region",
                    required: false,
                    col_type: ColType::Enum(STATE_OR_REGION),
                },
                DataCol {
                    name: "last_updated_date",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "entry_date",
                    required: true,
                    col_type: ColType::NaiveDate,
                },
                DataCol {
                    name: "notes",
                    required: false,
                    col_type: ColType::Other,
                },
            ];
        }
        "insurer_price_transparency" => {
            // Columns
            cols = &[
                DataCol {
                    name: "reporting_entity_name_legal",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "reporting_entity_name_common",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "reporting_entity_type",
                    required: false,
                    col_type: ColType::Enum(&["insurer", "other"]),
                },
                DataCol {
                    name: "machine_readable_url",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "machine_readable_url_status",
                    required: true,
                    col_type: ColType::Enum(&["up", "down", "corrupt"]),
                },
                DataCol {
                    name: "machine_readable_page",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "supplemental_url",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "file_name",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "file_format",
                    required: true,
                    col_type: ColType::Enum(ACCEPTED_FILE_TYPES),
                },
                DataCol {
                    name: "meets_standard",
                    required: false,
                    col_type: ColType::Enum(&["true", "false"]),
                },
                DataCol {
                    name: "standard_issue",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "state_or_region",
                    required: false,
                    col_type: ColType::Enum(STATE_OR_REGION),
                },
                DataCol {
                    name: "last_updated_date",
                    required: true,
                    col_type: ColType::Other,
                },
            ];
        }
        "hospital_existence" => {
            // Columns
            cols = &[
                DataCol {
                    name: "street_address",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "phone_number",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "zip_code",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "has_501c3",
                    required: false,
                    col_type: ColType::Enum(&["true", "false"]),
                },
                DataCol {
                    name: "fips_county_code",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "bed_count",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "city",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "state_or_region",
                    required: false,
                    col_type: ColType::Enum(STATE_OR_REGION),
                },
                DataCol {
                    name: "name_common",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "name_legal",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "ccn",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "facility_type",
                    required: false,
                    col_type: ColType::Enum(&[
                        "Short Term",
                        "Critical Access Hospitals",
                        "Long Term",
                        "Rehabilitation",
                        "Childrens Hospitals",
                        "Psychiatric",
                        "Transplant Hospitals",
                        "Religious Non-Medical Health Care Institutions",
                        "Medicaid Only Children's Psychiatric",
                        "other",
                    ]),
                },
                DataCol {
                    name: "medicare_medicaid_eligible",
                    required: false,
                    col_type: ColType::Enum(&["true", "false"]),
                },
                DataCol {
                    name: "lat",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "lon",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "ein",
                    required: false,
                    col_type: ColType::Other,
                },
            ];
        }
        "insurer_existence_insurers" => {
            // Columns
            cols = &[
                DataCol {
                    name: "homepage_url",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "state_or_regions",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "naic_company_code",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "insurer_name_legal",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "org_street_address",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "org_city",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "org_state_or_region",
                    required: false,
                    col_type: ColType::Enum(STATE_OR_REGION),
                },
                DataCol {
                    name: "org_zip_code",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "org_phone",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "ein",
                    required: false,
                    col_type: ColType::Other,
                },
            ];
        }
        "insurer_existence_issuers" => {
            // Columns
            cols = &[
                DataCol {
                    name: "hios_issuer_id",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "insurer_name_legal",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "state_or_region",
                    required: false,
                    col_type: ColType::Enum(STATE_OR_REGION),
                },
                DataCol {
                    name: "serff_id",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "naic_company_code",
                    required: false,
                    col_type: ColType::Other,
                },
            ];
        }
        "insurer_existence_plans" => {
            // Columns
            cols = &[
                DataCol {
                    name: "hios_issuer_id",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "hios_plan_id",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "plan_name",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "marketplace_type",
                    required: false,
                    col_type: ColType::Enum(&["individual", "small group", "large group"]),
                },
                DataCol {
                    name: "hios_product_id",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "plan_type",
                    required: false,
                    col_type: ColType::Enum(&[
                        "hmo",
                        "ppo",
                        "epo",
                        "pos",
                        "dental",
                        "other",
                        "indemnity",
                    ]),
                },
                DataCol {
                    name: "sob_url",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "full_benefits_url",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "effective_date",
                    required: false,
                    col_type: ColType::NaiveDate,
                },
                DataCol {
                    name: "expiration_date",
                    required: false,
                    col_type: ColType::NaiveDate,
                },
            ];
        }
        "insurer_homepages" => {
            // Columns
            cols = &[
                DataCol {
                    name: "id",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "insurer_name_legal",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "insurer_name_common",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "price_transparency_url",
                    required: true,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "price_transparency_url_status",
                    required: true,
                    col_type: ColType::Enum(&["up", "down", "corrupt"]),
                },
                DataCol {
                    name: "supplemental_url",
                    required: false,
                    col_type: ColType::Other,
                },
                DataCol {
                    name: "last_updated_date",
                    required: false,
                    col_type: ColType::NaiveDate,
                },
            ];
        }
        _ => return Err(format!("\"{}\" is not a valid schema name", schema_name))?,
    }

    return Ok(cols);
}

fn main() {
    // Parse command line args
    let args: Vec<String> = env::args().collect();
    let (file_path, schema_name): (&str, &str);
    match parse_args(&args) {
        Err(err) => {
            println!("ERROR:\n\n{}", err);
            process::exit(1);
        }
        Ok(res) => {
            (file_path, schema_name) = res;
        }
    }

    // Get relevant schema
    let schema_result = get_schema(schema_name);
    let cols: &[DataCol];
    match schema_result {
        Err(err) => {
            println!("ERROR:\n\n{}", err);
            process::exit(1);
        }
        Ok(schema) => {
            cols = schema;
        }
    }

    // Validate specified csv based on schema
    if let Err(err) = validate_csv(cols, file_path) {
        println!("ERROR:\n\n{}", err);
        process::exit(1);
    }
}

fn parse_args(args: &[String]) -> Result<(&str, &str), Box<dyn Error>> {
    if args.len() != 3 {
        return Err(
            "This utility script accepts exactly two arguments. A csv path, and a schema name.",
        )?;
    }

    let file_path = &args[1];
    let schema_name = &args[2];

    Ok((file_path, schema_name))
}

fn validate_csv(cols: &[DataCol], file_path: &str) -> Result<(), Box<dyn Error>> {
    // file handle and reader
    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut line: u32 = 1;

    // Collect intended headers
    let mut intended_headers: Vec<String> = vec![];
    for col in cols {
        intended_headers.push(col.name.to_string())
    }
    // Validate headers
    if let Err(err) = validate_headers(&mut reader, intended_headers) {
        return Err(From::from(err));
    }

    // Check each result, return read errors
    for result in reader.records() {
        line += 1;
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                if let Err(err) = validate_record(record, line, cols) {
                    return Err(From::from(err));
                };
            }
        }
    }
    return Ok(());
}

fn validate_headers(
    reader: &mut csv::Reader<std::fs::File>,
    intended_headers: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let headers = reader.headers()?;
    let intended_headers = csv::StringRecord::from(intended_headers);
    let new_len = headers.len();
    let old_len = intended_headers.len();
    if new_len != old_len {
        return Err(format!("Expected {} header cols, but found {}.\nPlease don't propose alterations to the headers in the same PR as a data submission.\n", old_len, new_len))?;
    }
    for (idx, header) in headers.iter().enumerate() {
        let intended_header = &intended_headers[idx];
        if header != intended_header {
            return Err(format!("Expected header \"{}\" in column {} but found \"{}\".\nPlease don't alter the headers in a data submission.\n", intended_header, idx, header))?;
        }
    }
    return Ok(());
}

fn validate_record(
    record: csv::StringRecord,
    line: u32,
    cols: &[DataCol],
) -> Result<(), Box<dyn Error>> {
    for (idx, col) in cols.iter().enumerate() {
        let field = &record[idx];

        // Disallow beginning and trailng whitespace in entries
        let trimmed_field = str::trim(field);
        if field != trimmed_field {
            return Err(format!(
                "Remove beginning or trailing whitespace from entry in column \"{}\" on line {}\n",
                col.name, line
            ))?;
        }

        if col.required {
            let disallowed = "";
            match col.col_type {
                ColType::Enum(enum_values) => {
                    if !enum_values.contains(&field) {
                        return Err(format!(
                            "Invalid entry: \"{}\" specified in required column \"{}\" on line {}.\n",
                            field, col.name, line
                        ))?;
                    }
                }
                ColType::NaiveDate => {
                    if trimmed_field == disallowed {
                        return Err(format!("Invalid entry: \"{}\" specified in required column \"{}\". Line {} does not abide.\n", field, col.name, line))?;
                    } else if NaiveDate::parse_from_str(field, "%Y-%m-%d").is_err() {
                        return Err(format!("Invalid date format entry: \"{}\" specified in column \"{}\" on line {}.\n Please use YYYY-MM-DD format.\n", field, col.name, line))?;
                    }
                }
                ColType::Other => {
                    if trimmed_field == disallowed {
                        return Err(format!("Invalid entry: \"{}\" specified in required column \"{}\". Line {} does not abide.\n", field, col.name, line))?;
                    }
                }
            }
        } else {
            match col.col_type {
                ColType::Enum(enum_values) => {
                    if !enum_values.contains(&field) && &field != &"" {
                        return Err(format!(
                            "Invalid entry: \"{}\" specified in column \"{}\" on line {}.\n",
                            field, col.name, line
                        ))?;
                    }
                }
                ColType::NaiveDate => {
                    if field == ""{
                    }
                    else if NaiveDate::parse_from_str(field, "%Y-%m-%d").is_err() {
                        return Err(format!("Invalid date format entry: \"{}\" specified in column \"{}\" on line {}.\n Please use YYYY-MM-DD format.\n", field, col.name, line))?;
                    }
                }
                ColType::Other => {}
            }
        }
    }

    return Ok(());
}
