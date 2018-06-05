// @todo - It would be nice to break each parser into it's own module
// e.g. parser::roll, parser::say, parser::core

use arg::*;
use nom::{
    IResult,
    alphanumeric,
    digit,
    recognize_float,
};
use nom::types::CompleteByteSlice;
use step::*;
use std::collections::HashMap;
use std::str;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub name: MacroOp,
    pub steps: Vec<Step>,
}

/// Matches advantage roll argument
pub fn advantage_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    map!(input, alt_complete!(tag!("advantage") | tag!("adv")), |_| Arg::Roll(RollArg::Advantage))
}

/// Matches left = right scenarios
pub fn assignment_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Assign> {
    do_parse!(input,
        left: assignment_left_p >>
        ws!(tag!("=")) >>
        right: alt_complete!(
            parse_inline_step_p => { | a | vec![ ArgValue::Step(a) ] } |
            assignment_right_p
        ) >>
        (Assign {
            left,
            right,
        })
    )
}

/// Match the left of an assignment/concat expression
pub fn assignment_left_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, ArgValue> {
    // we can only assign to tokens and variables
    ws!(input, alt_complete!(
        variable_p  => { | a | ArgValue::Variable(a)    } |
        token_p     => { | a | ArgValue::Token(a)       }
    ))
}

/// Match the right of an assignment/concat expression
pub fn assignment_right_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Vec<ArgValue>> {
    // we can assign almost anything else to them (except inline arguments, for now)
    many0!(input, alt_complete!(
        parse_inline_function_p => { | a | ArgValue::Step(a)                } |
        boolean_p               => { | a | ArgValue::Boolean(a)             } |
        num_p                   => { | a | ArgValue::Number(a)              } |
        float_p                 => { | a | ArgValue::Float(a)               } |
        word_p                  => { | a | ArgValue::Text(a)                } |
        quoted_interpolated_p   => { | a | ArgValue::TextInterpolated(a)    } |
        single_quoted_p         => { | a | ArgValue::Text(a)                } |
        variable_reserved_p     => { | a | ArgValue::VariableReserved(a)    } |
        variable_p              => { | a | ArgValue::Variable(a)            } |
        token_p                 => { | a | ArgValue::Token(a)               } |
        json_array_p            => { | a | ArgValue::Array(a)               } |
        json_hash_p             => { | a | ArgValue::Object(a)              } |
        primitive_p             => { | a | ArgValue::Primitive(a)           }
    ))
}

/// Matches arguments of unknown commands
pub fn arguments_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    alt_complete!(input,
        conditional_p           =>  { | a | Arg::Conditional(a)                                 } |
        assignment_p            =>  { | a | Arg::Assign(a)                                      } |
        concat_p                =>  { | a | Arg::Concat(a)                                      } |
        deduct_p                =>  { | a | Arg::Deduct(a)                                      } |
        variable_p              =>  { | a | Arg::Variable(a)                                    } |
        token_p                 =>  { | a | Arg::Token(a)                                       }
        // quoted_interpolated_p   =>  { | a | Arg::Unrecognized(ArgValue::TextInterpolated(a))    } |
        // single_quoted_p         =>  { | a | Arg::Unrecognized(ArgValue::Text(a))                } |
        // ws!(word_p)             =>  { | a | Arg::Unrecognized(ArgValue::Text(a))                }
    )
}

/// Matches !case arguments
pub fn arguments_case_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    add_return_error!(input, ErrorKind::Custom(4), do_parse!(
        input: ws!(alt_complete!(
            boolean_p               =>  { | a | ArgValue::Boolean(a)            } |
            num_p                   =>  { | a | ArgValue::Number(a)             } |
            float_p                 =>  { | a | ArgValue::Float(a)              } |
            word_p                  =>  { | a | ArgValue::Text(a)               } |
            quoted_interpolated_p   =>  { | a | ArgValue::TextInterpolated(a)   } |
            single_quoted_p         =>  { | a | ArgValue::Text(a)               } |
            variable_reserved_p     =>  { | a | ArgValue::VariableReserved(a)   } |
            variable_p              =>  { | a | ArgValue::Variable(a)           } |
            token_p                 =>  { | a | ArgValue::Token(a)              }
        )) >>
        options: switch!(options_p,
            Some(opts) => value!(opts) |
            _ => value!(vec![])
        ) >>
        (Arg::Case(Case {
            input,
            options,
        }))
    ))
}

/// Matches !input arguments
pub fn arguments_input_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    add_return_error!(input, ErrorKind::Custom(5), do_parse!(
        message: alt_complete!(
            quoted_interpolated_p |
            single_quoted_p => { | quote | TextInterpolated { parts: vec![ ArgValue::Text(quote) ] } }
        ) >>
        (Arg::Input(message))
    ))
}

/// Matches !prompt arguments
pub fn arguments_prompt_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    add_return_error!(input, ErrorKind::Custom(4), do_parse!(
        message: ws!(alt_complete!(
            quoted_interpolated_p |
            single_quoted_p => { |quote| TextInterpolated { parts: vec![ ArgValue::Text(quote) ] } }
        )) >>
        options: switch!(options_p,
            Some(opts) => value!(opts) |
            _ => value!(vec![])
        ) >>
        (Arg::Prompt(Prompt {
            message,
            options,
        }))
    ))
}

/// Matches an optional list of options
pub fn options_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Option<Vec<SwitchOption>>> {
    opt!(input, do_parse!(
        tag!("[") >>
        options: many0!(parse_option_p) >>
        tag!("]") >>
        (options)
    ))
}

/// Matches !roll arguments
pub fn arguments_roll_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    alt_complete!(input,
        advantage_p             |
        disadvantage_p          |
        roll_num_p              |
        roll_die_p              |
        roll_flag_e_p           |
        roll_flag_gt_p          |
        roll_flag_gte_p         |
        roll_flag_h_p           |
        roll_flag_l_p           |
        roll_flag_lt_p          |
        roll_flag_lte_p         |
        roll_flag_max_p         |
        roll_flag_min_p         |
        roll_flag_ro_p          |
        roll_flag_rr_p          |
        roll_modifier_pos_p     |
        roll_modifier_neg_p     |
        quoted_interpolated_p   => { | a | Arg::Roll(RollArg::Comment(ArgValue::TextInterpolated(a)))   } |
        single_quoted_p         => { | a | Arg::Roll(RollArg::Comment(ArgValue::Text(a)))               } |
        ws!(delimited!(
            tag!("["),
            alt_complete!(
                string_with_spaces_p    => { | a | Arg::Roll(RollArg::Comment(ArgValue::Text(a)))               } |
                quoted_interpolated_p   => { | a | Arg::Roll(RollArg::Comment(ArgValue::TextInterpolated(a)))   } |
                single_quoted_p         => { | a | Arg::Roll(RollArg::Comment(ArgValue::Text(a)))               }
            ),
            tag!("]")
        )) |
        token_p                 => { | a | Arg::Token(a)    } |
        variable_p              => { | a | Arg::Variable(a) }
        // map!(primitive_p,           | a | Arg::Roll(RollArg::Primitive(a)))
    )
}

/// Matches a custom side
pub fn roll_side_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Vec<ArgValue>> {
    delimited!(input,
        tag!("["),
        ws!(separated_list!(tag!(","), alt_complete!(roll_flag_var_p | num_p => { |n| ArgValue::Number(n) }))),
        tag!("]")
    )
}

/// Matches !say arguments
pub fn arguments_say_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    alt_complete!(input,
        quoted_interpolated_p   => { | a | Arg::Say(SayArg::Message(a))     } |
        single_quoted_p         => { | a | Arg::Say(SayArg::Message(TextInterpolated {
            parts: vec![ ArgValue::Text(a) ],
        }))                                                                 } |
        token_p                 => { | a | Arg::Say(SayArg::From(a))        }
    )
}

/// Matches !target arguments
pub fn arguments_target_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    alt_complete!(input,
        quoted_interpolated_p   => { | a | Arg::Say(SayArg::Message(a))     } |
        single_quoted_p         => { | a | Arg::Say(SayArg::Message(TextInterpolated {
            parts: vec![ ArgValue::Text(a) ],
        }))                                                                 }
    )
}

/// Matches !template arguments
pub fn arguments_template_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    alt_complete!(input,
        variable_word_p => { | a | Arg::Template(TemplateArg::Name(a))                          } |
        single_quoted_p => { | a | Arg::Template(TemplateArg::Name(a))                          } |
        double_quoted_p => { | a | Arg::Template(TemplateArg::Name(a))                          } |
        json_hash_p     => { | a | Arg::Template(TemplateArg::Attributes(ArgValue::Object(a)))  }
    )
}

/// Matches !test arguments
pub fn arguments_test_mode_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    map!(input, boolean_p, | b | Arg::TestMode(b))
}

/// Matches !whisper arguments
pub fn arguments_whisper_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    alt_complete!(input,
        quoted_interpolated_p   => { | a | Arg::Say(SayArg::Message(a))     } |
        single_quoted_p         => { | a | Arg::Say(SayArg::Message(TextInterpolated {
            parts: vec![ ArgValue::Text(a) ],
        }))                                                                 } |
        token_p                 => { | a | Arg::Say(SayArg::To(a))          }
    )
}

/// Matches a boolean operator
pub fn boolean_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, bool> {
    ws!(input, alt!(
        tag!("true")    => { |_| true   } |
        tag!("false")   => { |_| false  }
    ))
}

/// Matches left += right scenarios
pub fn concat_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Assign> {
    do_parse!(input,
        left: assignment_left_p >>
        ws!(tag!("+=")) >>
        right: assignment_right_p >>
        (Assign {
            left,
            right,
        })
    )
}
/// Matches any command
pub fn command_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, MacroOp> {
    ws!(input, alt!(
        tag_no_case!("!exit")                               => { |_| MacroOp::Exit          } |
        tag_no_case!("!template")                           => { |_| MacroOp::Template      } |
        tag_no_case!("!test")                               => { |_| MacroOp::TestMode      } |
        alt!(tag_no_case!("!case") | tag_no_case!("!c"))    => { |_| MacroOp::Case          } |
        alt!(tag_no_case!("!hroll") | tag_no_case!("!hr"))  => { |_| MacroOp::RollHidden    } |
        alt!(tag_no_case!("!input") | tag_no_case!("!i"))   => { |_| MacroOp::Input         } |
        alt!(tag_no_case!("!prompt") | tag_no_case!("!p"))  => { |_| MacroOp::Prompt        } |
        alt!(tag_no_case!("!roll") | tag_no_case!("!r"))    => { |_| MacroOp::Roll          } |
        alt!(tag_no_case!("!say") | tag_no_case!("!s"))     => { |_| MacroOp::Say           } |
        alt!(tag_no_case!("!target") | tag_no_case!("!t"))  => { |_| MacroOp::Target        } |
        alt!(tag_no_case!("!wroll") | tag_no_case!("!wr"))  => { |_| MacroOp::RollWhisper   } |
        alt!(tag_no_case!("!whisper") | tag_no_case!("!w")) => { |_| MacroOp::Whisper       }
    ))
}

pub fn comparison_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, ComparisonArg> {
    ws!(input, alt_complete!(
        tag!("==")  => { |_| ComparisonArg::EqualTo             } |
        tag!(">=")  => { |_| ComparisonArg::GreaterThanOrEqual  } |
        tag!("<=")  => { |_| ComparisonArg::LessThanOrEqual     } |
        tag!(">")   => { |_| ComparisonArg::GreaterThan         } |
        tag!("<")   => { |_| ComparisonArg::LessThan            }
    ))
}

/// Matches conditional statements (e.g. "1 > 2 ? success : failure")
pub fn conditional_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Conditional> {
    add_return_error!(input, ErrorKind::Custom(3), do_parse!(
        left: ws!(alt_complete!(
            variable_reserved_p => { | a | ArgValue::VariableReserved(a)    } |
            variable_p          => { | a | ArgValue::Variable(a)            } |
            token_p             => { | a | ArgValue::Token(a)               } |
            num_p               => { | a | ArgValue::Number(a)              } |
            float_p             => { | a | ArgValue::Float(a)               }
        )) >>
        comparison: comparison_p >>
        // but we can assign almost anything else to them (except inline arguments)
        right: ws!(alt_complete!(
            num_p               => { | a | ArgValue::Number(a)              } |
            float_p             => { | a | ArgValue::Float(a)               } |
            token_p             => { | a | ArgValue::Token(a)               } |
            variable_reserved_p => { | a | ArgValue::VariableReserved(a)    } |
            variable_p          => { | a | ArgValue::Variable(a)            }
        )) >>
        ws!(tag!("?")) >>
        success: ws!(alt_complete!(
            tag!("|") => { |_| None } |
            opt!(parse_step_p)
        )) >>
        ws!(tag!(":")) >>
        failure: ws!(alt_complete!(
            tag!("|") => { |_| None } |
            opt!(parse_step_p)
        )) >>
        (Conditional {
            left,
            comparison,
            right,
            success: success,
            failure: failure,
        })
    ))
}

/// Matches disadvantage roll argument
pub fn disadvantage_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    map!(input, alt_complete!(tag!("disadvantage") | tag!("dis")), |_| Arg::Roll(RollArg::Disadvantage))
}

/// Matches left -= right scenarios
pub fn deduct_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Assign> {
    do_parse!(input,
        left: assignment_left_p >>
        ws!(tag!("-=")) >>
        right: assignment_right_p >>
        (Assign {
            left,
            right,
        })
    )
}

/// Matches arguments in double quotes ("") - no interpolation
pub fn double_quoted_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, String> {
    do_parse!(input,
        word: delimited!(tag!("\""),take_until!("\""), tag!("\"")) >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
}

/// Match floats to argument strings
pub fn float_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, f32> {
    flat_map!(input, recognize_float, parse_to!(f32))
}

/// Matches "json" objects
pub fn json_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, ArgValue> {
    ws!(input,
        alt!(
            json_hash_p            => { | a | ArgValue::Object(a)           } |
            json_array_p           => { | a | ArgValue::Array(a)            } |
            boolean_p              => { | a | ArgValue::Boolean(a)          } |
            num_p                  => { | a | ArgValue::Number(a)           } |
            float_p                => { | a | ArgValue::Float(a)            } |
            quoted_interpolated_p  => { | a | ArgValue::TextInterpolated(a) } |
            single_quoted_p        => { | a | ArgValue::Text(a)             } |
            variable_reserved_p    => { | a | ArgValue::VariableReserved(a) } |
            variable_p             => { | a | ArgValue::Variable(a)         } |
            token_p                => { | a | ArgValue::Token(a)            } |
            string_with_spaces_p   => { | a | ArgValue::Text(a)             } |
            word_p                 => { | a | ArgValue::Text(a)             }
        )
    )
}


pub fn json_array_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Vec<ArgValue>> {
    ws!(input,
        delimited!(
            tag!("["),
            separated_list!(tag!(","), json_p),
            tag!("]")
        )
    )
}

pub fn json_hash_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, HashMap<String, ArgValue>> {
    ws!(input,
        map!(
            delimited!(
                tag!("{"),
                separated_list!(tag!(","), json_key_value_p),
                tag!("}")
            ),
            |tuple_vec| {
                let mut h: HashMap<String, ArgValue> = HashMap::new();
                for (k, v) in tuple_vec {
                    h.insert(k, v);
                }
                h
            }
        )
    )
}

pub fn json_key_value_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, (String, ArgValue)> {
    ws!(input,
        separated_pair!(
            alt_complete!(word_p | single_quoted_p | double_quoted_p),
            tag!(":"),
            json_p
        )
    )
}

/// Matches a macro name
pub fn name_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, MacroOp> {
    add_return_error!(input, ErrorKind::Custom(1), ws!(
        do_parse!(
            tag!("#") >>
            name: map_res!(is_not!(" \t\r\n"), |r: CompleteByteSlice| String::from_utf8(r.to_vec())) >>
            (MacroOp::Name(name))
        )
    ))
}

/// Match numbers to argument strings
pub fn num_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, i32> {
    do_parse!(input,
        sign: opt!(tag!("-")) >>
        num: digit >>
        not!(tag!(".")) >>
        s: value!(String::from_utf8(num.to_vec()).unwrap()) >>
        val: value!(s.parse::<i32>().unwrap()) >>
        switch: switch!(value!(&sign),
            &Some(_) => value!(-1 * val) |
            &None => value!(val)
        ) >>
        (switch)
    )
}

/// Matches any type of operation
pub fn op_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, MacroOp> {
    alt_complete!(input,
        name_p |
        command_p |
        value!(MacroOp::Lambda)
    )
}

/// Parse an option string (does not require quotes)
pub fn string_with_spaces_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, String> {
    do_parse!(input,
        word: is_not!("\t\r\n,?\\=<>|:;!#%^&*()+=/-[]{}'\"") >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
}

pub fn parse_option_key_value_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, ArgValue> {
    alt_complete!(input,
        boolean_p               => { | a | ArgValue::Boolean(a)             } |
        num_p                   => { | a | ArgValue::Number(a)              } |
        float_p                 => { | a | ArgValue::Float(a)               } |
        quoted_interpolated_p   => { | a | ArgValue::TextInterpolated(a)    } |
        single_quoted_p         => { | a | ArgValue::Text(a)                } |
        variable_reserved_p     => { | a | ArgValue::VariableReserved(a)    } |
        variable_p              => { | a | ArgValue::Variable(a)            } |
        token_p                 => { | a | ArgValue::Token(a)               } |
        string_with_spaces_p    => { | a | ArgValue::Text(a)                } |
        word_p                  => { | a | ArgValue::Text(a)                }
    )
}
/// Parses a valid option (e.g. Label 1, "Label 1", 'Label 1', Label:Value)
pub fn parse_option_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, SwitchOption> {
    // do not parse the key right away because
    do_parse!(input,
        label: ws!(parse_option_key_value_p) >>
        value: ws!(switch!(opt!(tag!(":")),
            // If we have a delim, parse the value
            Some(_) => ws!(parse_option_key_value_p) |
            None => value!(label.clone())
        )) >>
        opt!(tag!(",")) >>
        key: switch!(value!(label),
            ArgValue::Boolean(v)    => value!(Some(v.to_string())) |
            ArgValue::Float(v)      => value!(Some(v.to_string())) |
            ArgValue::Number(v)     => value!(Some(v.to_string())) |
            ArgValue::Text(v)       => value!(Some(v)) |
            _                       => value!(None)
        ) >>
        (SwitchOption {
            key,
            value,
        })
    )
}

/// Parse the complete macro
pub fn parse_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Program> {
    do_parse!(input,
        prog_name: name_p >>
        steps: many0!(parse_step_p) >>
        (Program {
            name: prog_name,
            steps: steps,
        })
    )
}

/// Parse a function
/// Step order matters!
///
/// e.g. "word{...}" where curly-braces delimit comma-separated values and
/// the word prior to the braces represents the name of the function
pub fn parse_inline_function_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Step> {
    let mut args = vec![];

    do_parse!(input,
        map!(variable_word_p, | name | args.push(Arg::Function(ArgValue::Text(name)))) >>
        delimited!(
            tag!("{"),
            separated_list_complete!(
                tag!("|"),
                map!(parse_option_key_value_p, | arg | args.push(Arg::Function(arg)))
            ),
            tag!("}")
        ) >>
        (Step {
            args,
            op: MacroOp::Lambda,
            result: StepResult::Ignore,
        })
    )
}

/// Parse a step for possible assignment, it must be a command that starts with a "!"
pub fn parse_inline_step_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Step> {
    do_parse!(input,
        op_type: command_p >>
        args: many0!(switch!(value!(&op_type),
            &MacroOp::Case          => call!(arguments_case_p) |
            &MacroOp::Input         => call!(arguments_input_p) |
            &MacroOp::Prompt        => call!(arguments_prompt_p) |
            &MacroOp::Roll          => call!(arguments_roll_p) |
            &MacroOp::RollHidden    => call!(arguments_roll_p) |
            &MacroOp::RollWhisper   => call!(arguments_roll_p) |
            &MacroOp::Target        => call!(arguments_target_p)
        )) >>
        (Step {
            args,
            op: op_type,
            result: StepResult::Ignore,
        })
    )
}

/// Parse a step of the program
pub fn parse_step_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Step> {
    do_parse!(input,
        op_type: op_p >>
        args: many0!(switch!(value!(&op_type),
            &MacroOp::Case          => call!(arguments_case_p) |
            &MacroOp::Input         => call!(arguments_input_p) |
            &MacroOp::Prompt        => call!(arguments_prompt_p) |
            &MacroOp::Roll          => call!(arguments_roll_p) |
            &MacroOp::RollHidden    => call!(arguments_roll_p) |
            &MacroOp::RollWhisper   => call!(arguments_roll_p) |
            &MacroOp::Say           => call!(arguments_say_p) |
            &MacroOp::Target        => call!(arguments_target_p) |
            &MacroOp::Template      => call!(arguments_template_p) |
            &MacroOp::TestMode      => call!(arguments_test_mode_p) |
            &MacroOp::Whisper       => call!(arguments_whisper_p) |
            _                       => call!(arguments_p)
        )) >>
        result: step_result_p >>
        (Step {
            args,
            op: op_type,
            result,
        })
    )
}

/// Matches primitive operations (starts with a number)
pub fn primitive_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Primitive> {
    ws!(input, alt_complete!(
        tag!("+") => { |_| Primitive::Add       } |
        tag!("-") => { |_| Primitive::Subtract  } |
        tag!("/") => { |_| Primitive::Divide    } |
        tag!("*") => { |_| Primitive::Multiply  }
    ))
}

/// Matches arguments in any type of quotes with variable interpolation
pub fn quoted_interpolated_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, TextInterpolated> {
    do_parse!(input,
        tag!("\"") >>
        parts: many0!(alt_complete!(
            variable_reserved_p => { | a | ArgValue::VariableReserved(a)                            } |
            variable_p          => { | a | ArgValue::Variable(a)                                    } |
            token_p             => { | a | ArgValue::Token(a)                                       } |
            not_a_token_or_variable_p     => { | a | ArgValue::Text(a)   }
        )) >>
        tag!("\"") >>
        (TextInterpolated {
            parts,
        })
    )
}

pub fn not_a_token_or_variable_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, String> {
    map!(input, is_not!("@$\""), | text | String::from_utf8(text.to_vec()).unwrap())
}

/// Matches digits for "D" and parses to i32
pub fn roll_digit_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, i32> {
    do_parse!(input,
        var: digit >>
        num: value!(String::from_utf8(var.to_vec()).unwrap()) >>
        (num.parse::<i32>().unwrap())
    )
}

/// Matches roll flag "e"
pub fn roll_flag_e_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("e") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::E(var)))
    )
}

/// Matches roll flag "gt"
pub fn roll_flag_gt_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("gt") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::GT(var)))
    )
}

/// Matches roll flag "gte"
pub fn roll_flag_gte_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("gte") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::GTE(var)))
    )
}

/// Matches roll flag "lt"
pub fn roll_flag_lt_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("lt") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::LT(var)))
    )
}

/// Matches roll flag "lte"
pub fn roll_flag_lte_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("lte") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::LTE(var)))
    )
}

/// Matches roll flag "h"
pub fn roll_flag_h_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("kh") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::H(var)))
    )
}

/// Matches roll flag "l"
pub fn roll_flag_l_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("kl") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::L(var)))
    )
}

/// Matches roll flag "max"
pub fn roll_flag_max_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("max") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::Max(var)))
    )
}

/// Matches roll flag "min"
pub fn roll_flag_min_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("min") >>
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::Min(var)))
    )
}

/// Matches roll flag "ro"
pub fn roll_flag_ro_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("ro") >>
        comparitive_op: opt!(comparison_p) >>
        value: roll_flag_var_p >>
        op: switch!(value!(comparitive_op),
            Some(o) => value!(o) |
            _ => value!(ComparisonArg::LessThan)
        ) >>
        (Arg::Roll(RollArg::RO(Comparitive {
            op,
            value,
        })))
    )
}

/// Matches roll flag "rr"
pub fn roll_flag_rr_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        tag!("rr") >>
        comparitive_op: opt!(comparison_p) >>
        value: roll_flag_var_p >>
        op: switch!(value!(comparitive_op),
            Some(o) => value!(o) |
            _ => value!(ComparisonArg::LessThan)
        ) >>
        (Arg::Roll(RollArg::RR(Comparitive {
            op,
            value,
        })))
    )
}

/// Matches valid roll flag inputs
pub fn roll_flag_var_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, ArgValue> {
    ws!(input, alt_complete!(
        variable_reserved_p => { |n| ArgValue::VariableReserved(n)  } |
        variable_p          => { |n| ArgValue::Variable(n)          } |
        roll_digit_p        => { |n| ArgValue::Number(n)            }
    )) 
}


/// Matches + modifiers
pub fn roll_modifier_neg_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        var: ws!(preceded!(tag!("-"), roll_modifier_var_p)) >>
        (Arg::Roll(RollArg::ModifierNeg(var)))
    )
}

/// Matches - modifiers
pub fn roll_modifier_pos_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    do_parse!(input,
        var: ws!(preceded!(tag!("+"), roll_modifier_var_p)) >>
        (Arg::Roll(RollArg::ModifierPos(var)))
    )
}

/// Matches valid modifier inputs
pub fn roll_modifier_var_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, ArgValue> {
    alt!(input,
        variable_reserved_p => { |n| ArgValue::VariableReserved(n)  } |
        variable_p          => { |n| ArgValue::Variable(n)          } |
        roll_digit_p        => { |n| ArgValue::Number(n)            } |
        token_p             => { |n| ArgValue::Token(n)             }
    )
}

/// Matches "N" in NdD
pub fn roll_num_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    // @todo @error if string/invalid throw error
    do_parse!(input,
        var: roll_flag_var_p >>
        (Arg::Roll(RollArg::N(var)))
    )
}

/// Matches "D" in NdD
pub fn roll_die_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Arg> {
    // @todo @error if string/invalid throw error
    do_parse!(input,
        var: ws!(preceded!(tag!("d"), alt_complete!(
            roll_flag_var_p => { | a | Arg::Roll(RollArg::D(a))     } |
            roll_side_p     => { | a | Arg::Roll(RollArg::Sides(a)) }
        ))) >>
        (var)
    )
}

/// Matches arguments in quotes ('')
pub fn single_quoted_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, String> {
    do_parse!(input,
        word: delimited!(tag!("'"),take_until!("'"), tag!("'")) >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
}

/// Matches a passed or ignored result
pub fn step_result_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, StepResult> {
    alt_complete!(input,
        ws!(tag!(">>")) => { |_| StepResult::Save   } |
        ws!(tag!("|"))  => { |_| StepResult::Ignore } |
        value!(StepResult::Ignore)
    )
}

/// Matches tokens
pub fn token_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, TokenArg> {
    // @todo match that we cannot start with a digit
    do_parse!(input,
        name: ws!(preceded!(tag!("@"), token_name_p)) >>
        attribute: switch!(opt!(complete!(preceded!(tag!("."), variable_name_p))),
            Some(a) => value!(Some(String::from_utf8(a.to_vec()).unwrap())) |
            _ => value!(None)
        ) >>
        macro_name: switch!(opt!(complete!(preceded!(tag!("->"), variable_name_p))),
            Some(a) => value!(Some(String::from_utf8(a.to_vec()).unwrap())) |
            _ => value!(None)
        ) >>
        (TokenArg { name, attribute, macro_name })
    )
}

/// Parse a valid string for names
pub fn token_name_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, String> {
    do_parse!(input,
        word: alt_complete!(
            delimited!(tag!("{"), is_not!(" \t\r\n.,?\\=<>|:;@!#$%^&*()+=/-[]{}'\""), tag!("}")) |
            is_not!(" \t\r\n.,?\\=<>|:;@!#$%^&*()+=/-[]{}'\"")
        ) >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
}

/// Matches a valid variable name
pub fn variable_name_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, CompleteByteSlice> {
    alt_complete!(input,
        delimited!(tag!("{"), is_not!(" \t\r\n,?\\=<>|:;@!#$%^&*()+=/-[]{}'\""), tag!("}")) |
        is_not!(" \t\r\n.,?\\=<>|:;@!#$%^&*()+=/-[]{}'\"")
    )
}

/// Matches variables
pub fn variable_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, String> {
    // @todo match that we cannot start with a digit
    do_parse!(input,
        var: preceded!(tag!("$"), variable_name_p) >>
        (String::from_utf8(var.to_vec()).unwrap())
    )
}

/// Matches reserved variables (digits only)
pub fn variable_reserved_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, i16> {
    do_parse!(input,
        var: ws!(preceded!(tag!("$"), alt_complete!(
            delimited!(tag!("{"), digit, tag!("}")) |
            digit
        ))) >>
        num: value!(String::from_utf8(var.to_vec()).unwrap()) >>
        (num.parse::<i16>().unwrap())
    )
}

/// Match alphanumeric words to strings
pub fn word_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, String> {
    do_parse!(input,
        word: alphanumeric >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
}

/// Match variable words to strings
pub fn variable_word_p(input: CompleteByteSlice) -> IResult<CompleteByteSlice, String> {
    do_parse!(input,
        word: variable_name_p >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
}

// /// Maps error codes to readable strings
// pub fn error_to_string(e: Err) -> String {
    // let err = match e {
        // ErrorKind::Custom(1)    => "Missing or invalid macro name",
        // ErrorKind::Custom(2)    => "Invalid or unrecognized command",
        // ErrorKind::Custom(3)    => "Problem parsing conditional statement",
        // ErrorKind::Custom(4)    => "Problem parsing prompt options",
        // _                       => "Unknown problem encountered while parsing",
    // };
    // err.to_string()
// }
