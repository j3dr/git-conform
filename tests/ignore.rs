mod common;

use git_conform::core::api::ignore;

use std::fs;
use std::path::Path;

use serial_test::serial;

#[test]
#[serial]
fn case_ignore_real() {
    let (_home_dir, _tracking_file, ignored_file, tests_dir) = common::setup().unwrap();

    // Remove the ignored file if it already exists
    if Path::new(ignored_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&ignored_file.path).unwrap();
    }

    // The function executes without errors
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/repo{n}"));
        repos.push(format!("{tests_dir}/.hidden/repo{n}"));
    }
    assert_eq!(ignore(repos, &ignored_file), Ok(()));

    // Read the updated ignored file
    let ign_file_up = fs::read_to_string(ignored_file.path).unwrap();

    for n in 1..=3 {
        // The ignored file contains real repositories
        assert!(ign_file_up.contains(
            format!("{tests_dir}/repo{n}").as_str()
        ));
        assert!(ign_file_up.contains(
            format!("{tests_dir}/.hidden/repo{n}").as_str()
        ));
    }
}

#[test]
fn case_ignore_fake() {
    let (_home_dir, _tracking_file, ignored_file, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/fake_repo{n}"));
        repos.push(format!("{tests_dir}/.hidden/fake_repo{n}"));
    }
    assert_eq!(ignore(repos, &ignored_file), Err(String::from("Repositories validation failed")) );
}

#[test]
fn case_ignore_regular_dirs() {
    let (_home_dir, _tracking_file, ignored_file, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/dir{n}"));
        repos.push(format!("{tests_dir}/.hidden/dir{n}"));
    }
    assert_eq!(ignore(repos, &ignored_file), Err(String::from("Repositories validation failed")) );
}

#[test]
fn case_ignore_non_existent() {
    let (_home_dir, _tracking_file, ignored_file, _tests_dir) = common::setup().unwrap();

    // The function throws an error
    let repos = vec![
        format!("quegq4tq4q"),
        format!("lvdslns"),
        format!("fjioadbaob")
    ];
    assert_eq!(ignore(repos, &ignored_file), Err(String::from("Repositories validation failed")) );
}

#[test]
fn case_ignore_files() {
    let (_home_dir, _tracking_file, ignored_file, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/file{n}"));
    }
    assert_eq!(ignore(repos, &ignored_file), Err(String::from("Repositories validation failed")) );
}
