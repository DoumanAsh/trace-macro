#[macro_use(TRACE)]
extern crate trace_macros;

#[test]
fn dummy() {
    TRACE!("This", "is", "dummy", "test");
}
