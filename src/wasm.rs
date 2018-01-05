use stdweb;

pub fn prompt (input: &str) {
    // should this be here or somewhere else?
    stdweb::initialize();

    js! {
        if (window.TTML) {
            window.TTML.prompt( @{input} );
        }
    }
}

pub fn target (input: &str) -> &str {
    stdweb::initialize();

    let result = js! {
        if (window.TTML) {
            return window.TTML.target( @{input} );
        }
    };

    // return the value as a str
    result.as_str().unwrap()
}
