use opengl::*;

#[test]
fn test_version() {
    let (a, b) = OPENGL_VERSION;
    assert_eq!(a, 4);
    assert_eq!(b, 5);
}
