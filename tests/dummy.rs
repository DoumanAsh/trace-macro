#[macro_use(TRACE, strace, INFO, ENTER, DEBUG_TRACE)]
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
