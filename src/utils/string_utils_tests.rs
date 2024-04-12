use crate::utils::string_utils_impl::{convert_str_to_title_case, convert_str_to_underscore_case};

#[test]
fn test_to_title_case() {
    let input_case = "this is a stRing to conVert to title case.";
    let title_case = convert_str_to_title_case(input_case);
    let expected_case = "This Is A String To Convert To Title Case.";
    assert_eq!(title_case, expected_case);
    println!("[{input_case}] -> [{title_case}]");
}

#[test]
fn test_to_underscore_case() {
    let input_case = "this is a stRing to conVert to UnderScore case.";
    let underscore_case = convert_str_to_underscore_case(input_case);
    let expected_case = "this is a st_ring to con_vert to under_score case.";
    assert_eq!(underscore_case, expected_case);
    println!("[{input_case}] -> [{underscore_case}]");
}