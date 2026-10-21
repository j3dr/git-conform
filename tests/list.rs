use git_conform::core::api::list;

#[test]
fn case_list_empty() {
    // The function throws an error
    assert_eq!(list(""), Err(String::from("The file is empty")));
}
