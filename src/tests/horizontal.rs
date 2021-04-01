use super::*;

#[test]
fn horizontal() {
    assert_parsed!("{}" horizontal: [_]);
    assert_parsed!("{aaa}" horizontal: [_]);
    assert_parsed!("{|aaa|}" horizontal: [_]);
    assert_parsed!("{* aaa}" horizontal: [_]);
}

#[test]
fn horizontal_single() {
    assert_parsed!("aaa" horizontal_single: [ "aaa" regular_text: []; ]);
    // assert_parsed!(r"\foo;" horizontal_single: [ r"\foo" regular_text: []; ]);
    // assert_parsed!("#foo;" horizontal_single: [ "#foo" regular_text: []; ]);
    // assert_parsed!("${a}" horizontal_single: [ "${a}" regular_text: []; ]);
    assert_parsed!("`foo`" horizontal_single: [ "`foo`" const_string: []; ]);
    assert_parsed!("aaa `foo`" horizontal_single: [
        "aaa " regular_text: [];
        "`foo`" const_string: [];
    ]);
}

#[test]
fn horizontal_list() {
    assert_parsed!("|aaa|" horizontal_list: [ "aaa" horizontal_single, regular_text: []; ]);
    assert_parsed!("|aaa| bbb |" horizontal_list: [
        "aaa" horizontal_single, regular_text: [];
        "bbb " horizontal_single, regular_text: [];
    ]);
}

#[test]
fn horizontal_bullet_list() {
    assert_parsed!("* foo" horizontal_bullet_list: [
        "* foo" horizontal_bullet: [
            "*" horizontal_bullet_star: [];
            "foo" horizontal_single, regular_text: [];
        ];
    ]);
    assert_parsed!("* foo ** bar" horizontal_bullet_list: [
        "* foo " horizontal_bullet: [
            "*" horizontal_bullet_star: [];
            "foo " horizontal_single, regular_text: [];
        ];
        "** bar" horizontal_bullet: [
            "**" horizontal_bullet_star: [];
            "bar" horizontal_single, regular_text: [];
        ];
    ]);
}