use macros::DiffFields;

#[derive(DiffFields)]
struct Foo {
    a: u32,
    b: i32,
    #[skip_diff]
    skipped: i32
}

#[test]
fn foo() {
    let f = FooDiff::A(4);
    let mut foo = Foo { a: 1, b: 2, skipped: 3};
    match f {
        FooDiff::A(x) => {}
        FooDiff::B(y) => {}
    }
    apply_foo_diff(&mut foo, f);
    assert_eq!(foo.a, 4);
}

