use arg::ArgValue;
use futures::prelude::*;
use futures::unsync::oneshot::{channel, Receiver};
use futures::{Future, Poll};
use futures::{Stream, Async};
use futures::executor;
use std::collections::HashMap;

// structs to list options for the user during prompt!
#[derive(Serialize)]
struct Options {
    options: Vec<PromptOption>,
}

#[derive(Serialize)]
struct PromptOption {
    key: String,
    value: String,
}

pub struct JsFuture(Receiver<String>);
impl Future for JsFuture {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(self.0.poll().expect("Unexpected error"))
    }
}
#[cfg(feature = "web")]
fn js_prompt (input: &str, options: Options) -> JsFuture {
    js_serializable!( Options );

    let (tx, rx) = channel();
    let mut tx = Some(tx);
    let resolve = move |result: String| {
        tx.take()
            .expect("Unexpected second call of resolve()")
            .send(result)
            .ok();
    };

    js! {
        if (window.TTML) {
            const resolve = @{resolve};

            // Execute the prompt and return the promise
            window.TTML.prompt( @{input}, @{options} ).then(function(result) {
                resolve(result);
                resolve.drop();
            }).catch(function(error) {
                resolve("Unexpected error");
                resolve.drop();
            });
        } else {
            resolve("TTML object not found");
            resolve.drop();
        }
    };

    JsFuture(rx)
}

pub fn prompt (input: &str, opts: &HashMap<String, ArgValue>) -> String {
    let mut prompt_options = vec![];

    // Convert the hashmap to a list of values we can send to the user
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
        executor::spawn(js_prompt(input, options)).wait_future().unwrap()
    }

    #[cfg(not(feature = "web"))]
    {
        "test_value".to_string()
    }
}

pub fn target (input: &str) -> String {
    #[cfg(feature = "web")]
    {
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
