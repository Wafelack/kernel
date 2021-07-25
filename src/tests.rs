use crate::{memory::pmm::{alloc_page, free_page}, ok, err, info, serialn};

#[macro_export]
macro_rules! test {
    ($name:literal, $fn:expr) => {
        $crate::serialn!("\x1b[0;34m>>>\x1b[0m {} \x1b[0;34m<<<\x1b[0m", $name);
        let res = ($fn)();
        $crate::serialn!("{} ... {}\x1b[0m", $name, if res == 0 {
            "\x1b[0;32mok"
        } else {
            "\x1b[0;31mfailed"
        });
    }
}

pub fn run_tests() {
    info!("Running tests...\n");
    test!("page_alloc", || if let Some(idx) = alloc_page(1) {
            ok!("Successfully allocated page at address {:#08x}.", idx);
            free_page (idx, 1);
            assert_eq!(Some(idx), alloc_page(1));
            ok!("Successfully freed page at address {:#08x}.", idx);;
            free_page (idx, 1);
            0
        } else {
            err!("Failed to allocate page.");
            1
        }
    );
    serialn!("");
    ok!("Ran all tests.");
}
