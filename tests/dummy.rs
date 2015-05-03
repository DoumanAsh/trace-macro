#[macro_use(connect_args, TRACE, strace, INFO, ENTER, DEBUG_TRACE)]
extern crate trace_macros;

#[test]
fn dummy() {
    let a = 5;
    let b = "string";
    ENTER!(a, b);
    TRACE!("This", "is", "dummy", "test");
    TRACE!(sep=>"+", "This", "is", "dummy", "test");
    INFO!("This", "is", "dummy", "test");
    TRACE!("lolka");
    TRACE!(type=>"ENTER");
    TRACE!(type=>"INFO", "lolka");
    TRACE!(type=>"INFO", "lolka", 666);
    TRACE!(type=>"INFO", sep=>"_", "lolka");
    TRACE!(type=>"INFO", sep=>"_", "lolka", 2);
    strace!("This is dummy test");

    INFO!("\n\nDEBUG TRACE section");
    DEBUG_TRACE!("lolka");
    DEBUG_TRACE!(type=>"ENTER");
    DEBUG_TRACE!(type=>"INFO", "lolka");
    DEBUG_TRACE!(type=>"INFO", "lolka", 666);
    DEBUG_TRACE!(type=>"INFO", sep=>"_", "lolka");
    DEBUG_TRACE!(type=>"INFO", sep=>"_", "lolka", 2);
    DEBUG_TRACE!(type=>"INFO", sep=>"_", "lolka", (1..22));
}

#[test]
fn connect_args() {
    assert_eq!(connect_args!(1, 2, 3), "1 2 3".to_string());
    assert_eq!(connect_args!(sep=>"-", 1, 2, 3), "1-2-3".to_string());
    assert_eq!(connect_args!(sep=>"+", 1, 2, 666), "1+2+666".to_string());
    assert_eq!(connect_args!(666), "666".to_string());
    assert_eq!(connect_args!(formatter=>"{:?}", sep=>"+", 1, 2, "KIK"), "1+2+\"KIK\"".to_string());
}
