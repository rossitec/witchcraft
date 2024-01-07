use crate::core::messages::standard_messages;
use crate::core::utils::*;
use crate::meow::meow::read_meow;
use crate::modules::antivirus::entropy::advanced_entropy_scanner;
use sha256::{digest, try_digest};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn search_malware_hash(search_term: &str, debug: bool) -> bool {
    let config = read_meow("/var/witch_craft/witch_spells/embedded/config.meow", false);
    let malware_db = &format!("{}{}", config["GENERAL_BASE_PATH"], config["MALWARE_HASH"]);

    let file_path = malware_db;

    if debug {
        standard_messages(
            "warning",
            "File path search_malware_hash",
            file_path,
            "cute",
        );
    }

    match search_csv(file_path, search_term) {
        Ok(matches) => {
            if !matches.is_empty() {
                println!("Matching Rows:");
                for row in matches {
                    standard_messages("warning", "Malware was discovered", &row, "cute");
                }
                return true;
            } else {
                standard_messages("warning", "Unidentified malware in search. Either your system is clean or no malware has been found", "", "cute");
                return false;
            }
        }
        Err(err) => {
            let message = format!("{}", err);
            standard_messages(
                "error",
                "Error found while looking for malware hash's",
                &message,
                "cute",
            );
            return false;
        }
    }
}

pub fn search_malware_pattern(pattern: &str, _debug: bool) -> bool {
    let config = read_meow("/var/witch_craft/witch_spells/embedded/config.meow", false);
    let malware_db = &format!("{}{}", config["GENERAL_BASE_PATH"], config["MALWARE_HASH"]);

    standard_messages("flagged", "Searching for malware pattern", "", "cute");

    match find_all_matching_lines(malware_db, pattern) {
        Ok(result) => {
            if !result.is_empty() {
                for line in result {
                    standard_messages("flagged", "Malware was discovered.", &line, "cute");
                }
                return true;
            } else {
                standard_messages("warning", "Pattern not found in any line.", "", "cute");
                return true;
            }
        }
        Err(err) => {
            let message = format!("{}", err);
            standard_messages(
                "error",
                "Error found while looking for malware.",
                &message,
                "cute",
            );
            return false;
        }
    }
}

pub fn calculate_sha256_hash(file_path: &str, debug: bool) -> Result<String, io::Error> {
    let input = Path::new(file_path);
    let hash = try_digest(input).unwrap();

    if debug == true {
        standard_messages("flagged", "SHA256", &hash, "cute");
        standard_messages("flagged", "File path", &hash, "cute");
    }

    Ok(hash)
}

pub fn list_files_and_folders(dir_path: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut items = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        items.push(path.clone());

        if path.is_dir() {
            let sub_items = list_files_and_folders(path.to_str().unwrap())?;
            items.extend(sub_items);
        }
    }

    Ok(items)
}

pub fn active_malware_scanner(directory: &str, debug: bool) -> bool {
    if debug == true {
        println!("{}", directory);
    }

    match list_files_and_folders(directory) {
        Ok(items) => {
            for _item in items {
                match calculate_sha256_hash("/bin/yes", true) {
                    Ok(result) => {
                        return search_malware_pattern(&result, debug);
                    }
                    Err(err) => {
                        let message = format!("{}", err);
                        standard_messages(
                            "error",
                            "Error found while processing file sha256 signature.",
                            &message,
                            "cute",
                        );
                        return false;
                    }
                }
            }
            return true;
        }
        Err(err) => {
            let message = format!("{}", err);
            standard_messages(
                "error",
                "Error found while running active scanning for malware.",
                &message,
                "cute",
            );
            return false;
        }
    }
}

pub fn shell_antivirus(system_input: &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--hash" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = &take_system_args(system_input, "--hash");

            search_malware_hash(instance, debug)
        }

        "--pattern" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = &take_system_args(system_input, "--pattern");

            search_malware_pattern(instance, debug)
        }

        "--scanner" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = &take_system_args(system_input, "--directory");

            active_malware_scanner(instance, debug)
        }

        "--entropy" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = &take_system_args(system_input, "--path");
            advanced_entropy_scanner(instance, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_antivirus", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}