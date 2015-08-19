use std::fs::File;
use std::io::Cursor;
use std::result::Result;

use ::errors::Error;
use ::parser::{Parser, Event};
use ::builder::{Builder, decode};


fn reference_events() -> Vec<Event> {
    vec![
    Event::StartMap,
        Event::Key("docs".to_string()),
        Event::StartArray,
            Event::StartMap,
                Event::Key("null".to_string()),
                Event::Null,
                Event::Key("boolean".to_string()),
                Event::Boolean(false),
                Event::Key("true".to_string()),
                Event::Boolean(true),
                Event::Key("integer".to_string()),
                Event::Number(0f64),
                Event::Key("double".to_string()),
                Event::Number(0.5f64),
                Event::Key("exponent".to_string()),
                Event::Number(100f64),
                Event::Key("long".to_string()),
                Event::Number(10000000000f64),
                Event::Key("string".to_string()),
                Event::String("строка - тест".to_string()),
            Event::EndMap,
            Event::StartMap,
                Event::Key("meta".to_string()),
                Event::StartArray,
                    Event::StartArray,
                        Event::Number(1f64),
                    Event::EndArray,
                    Event::StartMap,
                    Event::EndMap,
                Event::EndArray,
            Event::EndMap,
            Event::StartMap,
                Event::Key("meta".to_string()),
                Event::StartMap,
                    Event::Key("key".to_string()),
                    Event::String("value".to_string()),
                Event::EndMap,
            Event::EndMap,
            Event::StartMap,
                Event::Key("meta".to_string()),
                Event::Null,
            Event::EndMap,
        Event::EndArray,
    Event::EndMap,
    ]
}


#[test]
fn parser() {
    let f = File::open("test.json").unwrap();
    let events: Vec<_> = Parser::new(f).map(Result::unwrap).collect();
    assert_eq!(events, reference_events());
}

#[test]
fn prefixes() {
    let f = File::open("test.json").unwrap();
    let full: Vec<_> = Parser::new(f).map(Result::unwrap).collect();
    let f = File::open("test.json").unwrap();
    let result: Vec<_> = Parser::new(f).prefix("").map(Result::unwrap).collect();
    assert_eq!(result, full);

    let f = File::open("test.json").unwrap();
    let result: Vec<_> = Parser::new(f).prefix("docs.item.meta.item").map(Result::unwrap).collect();
    assert_eq!(result, vec![
        Event::StartArray,
        Event::Number(1f64),
        Event::EndArray,
        Event::StartMap,
        Event::EndMap,
    ]);
}

#[test]
fn items() {
    let f = File::open("test.json").unwrap();
    let result: Vec<_> = Parser::new(f).items("").map(Result::unwrap).collect();
    assert_eq!(result.len(), 1);

    #[derive(RustcDecodable, Debug, PartialEq)]
    struct Person {
        name: String,
        friends: Vec<String>,
    }

    let f = File::open("people.json").unwrap();
    let json = Parser::new(f).items("item").next().unwrap().unwrap();
    let result: Person = decode(json).unwrap();
    let reference = Person {
        name: "John".to_string(),
        friends: vec!["Mary".to_string(), "Michael".to_string()],
    };
    assert_eq!(result, reference);
}

#[test]
fn unterminated_string() {
    let s = Cursor::new(br#"{"key": "value"#.to_vec());
    let r = Parser::new(s).last().unwrap();
    assert!(r.is_err());
    match r.err().unwrap() {
        Error::Unterminated => (),
        _ => panic!("Not {}", Error::Unterminated),
    }
}
