use arg::ArgValue;
use std::collections::HashMap;

#[cfg(feature = "web")]
use stdweb;

#[derive(Serialize)]
struct Options {
    options: Vec<PromptOption>,
}

#[derive(Serialize)]
struct PromptOption {
    key: String,
    value: String,
}

pub fn prompt (input: &str, opts: &HashMap<String, ArgValue>) -> String {
    let mut prompt_options = vec![];

    for (key, value) in opts {
        prompt_options.push(PromptOption {
            key: key.clone(),
            value: match value {
                &ArgValue::Boolean(ref boolean) => boolean.to_string(),
                &ArgValue::Float(ref float) => float.to_string(),
                &ArgValue::Number(ref number) => number.to_string(),
                &ArgValue::Text(ref text) => text.clone(),
                &ArgValue::Token(ref token) => token.to_string(),
                &ArgValue::Variable(ref var) => var.clone(),
                &ArgValue::VariableReserved(ref var) => var.to_string(),
                _ => "".to_string(),
            },
        })
    }

    let options = Options {
        options: prompt_options,
    };

    #[cfg(feature = "web")]
    {
        // should this be here or somewhere else?
        stdweb::initialize();
        js_serializable!( Options );

        let result = js! {
            if (window.TTML) {
                return window.TTML.prompt( @{input}, @{options} );
            }
        };

        // return the value
        result.as_str().unwrap().to_string()
    }

    #[cfg(not(feature = "web"))]
    {
        "test_value".to_string()
    }
}

pub fn target (input: &str) -> String {
    #[cfg(feature = "web")]
    {
        stdweb::initialize();

        let result = js! {
            if (window.TTML) {
                return window.TTML.target( @{input} );
            }
        };

        // return the value
        result.as_str().unwrap().to_string()
    }

    #[cfg(not(feature = "web"))]
    {
        "test_id".to_string()
    }
}
