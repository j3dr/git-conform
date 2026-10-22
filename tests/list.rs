mod common;

use git_conform::core::api::list;
use git_conform::utils::AppFile;

use serial_test::serial;

#[test]
#[serial]
fn case_tracked_list_empty() {
    let (_home_dir, mut tracking_file, _ignored_file, _tests_dir) = common::setup().unwrap();

    // Make contents of the tracking file empty
    tracking_file.contents = String::new();

    // The function throws an error
    assert_eq!(list(&tracking_file), Err(format!("The file '{}' is empty or doesn't exist yet", tracking_file.path)));
}

#[test]
#[serial]
fn case_ignored_list_empty() {
    let (_home_dir, _tracking_file, mut ignored_file, _tests_dir) = common::setup().unwrap();

    // Make contents of the ignored file empty
    ignored_file.contents = String::new();

    // The function throws an error
    assert_eq!(list(&ignored_file), Err(format!("The file '{}' is empty or doesn't exist yet", ignored_file.path)));
}
