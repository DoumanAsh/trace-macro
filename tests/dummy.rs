#[macro_use(TRACE, strace, INFO, ENTER)]
extern crate trace_macros;

#[test]
fn dummy() {
    let a = 5;
    let b = "string";
    ENTER!(a, b);
    TRACE!("This", "is", "dummy", "test");
    INFO!("This", "is", "dummy", "test");
    TRACE!(1);
    TRACE!(type=>"ENTER");
    strace!("This is dummy test");
}
