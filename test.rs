text
/* this is a
/* nested
block */ comment.
And should stay a comment
even with "strings"
or 42 0x18 0b01011   // blah
or u32 as i16 if impl */
text
/** this is a
/*! nested
block */ doc comment */
text

/// test
/// *test*
/// **test**
/// ***test***
/// _test_
/// __test__
/// __***test***__
/// ___test___
/// # test
/// ## test
/// ### test
/// #### test
/// ##### test
/// ###### test
/// ####### test
/// # test **test**
/// # test _test_
/// [text]()
/// [test](http://test.com)
/// [test](#test)
/// ![test](#test)
/// [test]
/// [test]: http://test.com
/// `test code`
/// ```rust
/// test code block
/// ```
/// test
///

///
/// # Test
///
/// ```
/// assert!(true);
/// ```
///
fn test(&self) {
    asdf
    // test
    asdf
}

/**
 * Deprecated
 */

/*!
 * Deprecated
 */

text /**/ text
text /***/ text
text /****/ text

text
text // line comment
text /// line doc comment
text //! line doc comment
text

text #![main] text
text #![allow(great_algorithms)] text
text #![!resolve_unexported] text
text #[deny(silly_comments)] text
#[doc = "This attribute contains ] an attribute ending character"]

text r"This is a raw string" text
text r"This raw string ends in \" text
text r#"This is also valid"# text
text r##"This is ##"# also valid."## text
text r#"This is #"## not valid."# text  //"
text b"This is a bytestring" text
text br"And a raw byte string" text
text rb"Invalid raw byte string" text
text br##"This is ##"# also valid."## text
text r##"Raw strings can
span multiple lines"## text

text "double-quote string" text
text "string\nwith\x20escaped\"characters" text
text "string with // comment /* inside" text
text "strings can
span multiple lines" text

text 'c' text
text 'cc' text
text '\n' text
text '\nf' text
text '\n\n' text
text '\x20' text
text '\'' text
text '\\' text
text b'b' text
text b'bb' text
text b'\x20' text

text 42i32 text
text 42is text
text 42int text
text 42f32 text
text 42e+18 text
text 42.1415 text
text 42.1415f32 text
text 42.1415e18 text
text 42.1415e+18 text
text 42.1415e-18f64 text

text 42 text
text 0xf00b text
text 0o755 text
text 0b101010 text

text bool text char text usize text isize text
text u8 text u16 text u32 text u64 text
text i8 text i16 text i32 text i64 text
text Self text
text str text &str text String text &String text

text true text false text

text break text continue text do text else text
text if text in text for text loop text
text match text return text while text

text as text crate text extern text mod text
text let text proc text ref text

text
extern crate foo;
text
use std::slice;
text
use std::{num, str};
text
use self::foo::{bar, baz};
text
use super::foo::{bar, baz};
text

x = 1+1;
y = 4-2;
x *= 3;
y++;
y += 1;

text
pub enum MyEnum {
    One,
    Two
}
text
pub struct MyStruct<'foo> {
    pub one: u32,
    two: Option<'a, MyEnum>,
    three: &'foo i32,
}
text
pub struct MyTupleStruct(pub i32, u32);
text

text
type MyType = u32;
text

text
static MY_CONSTANT: &str = "hello";
text

text
pub trait MyTrait {
    text
    fn create_something (param: &str, mut other_param: u32) -> Option<Self>;
    text
    fn do_whatever<T: Send+Share+Whatever, U: Freeze> (param: &T, other_param: u32) -> Option<U>;
    text
    fn do_all_the_work (&mut self, param: &str, mut other_param: u32) -> bool;
    text
    fn do_even_more<'a, T: Send+Whatever, U: Something<T>+Freeze> (&'a mut self, param: &T) -> &'a U;
    text
}
text

text
impl<'foo> MyTrait for MyStruct<'foo> {
    text
    fn create_something (param: &str, mut other_param: u32) -> Option<Self> {
        text
        return Some(cake);
        text
    }
    text
    fn do_whatever<T: Send+Share+Whatever, U: Freeze> (param: &T, other_param: u32) -> Option<U> {
        assert!(1 != 2);
        text
        self.with_something(param, |arg1, arg2| {
            text
        }, other_param);
    }
    text
    fn do_all_the_work (&mut self, param: &str, mut other_param: u32) -> bool {
        announce!("There's no cake");
        if !test_subject.under_control() {
            text
            let list: Vec<item> = some_iterator.map(|elem| elem.dosomething()).collect();
            text
            let boxed_list = box list;
            text
            self.announce_warning();
            text
            if test_subject.ignored_warnings > 3 {
                text
                test_subject.incinerate();
                text
            }
            text
        }
        text
    }
    text
    fn do_even_more<'a, T: Send+Whatever+'static, U: Something<T>+Freeze> (&'a mut self, param: &T) -> &'a U {
        text
        let foo: Option<'a u32> = Some(18);
        text
        if self.one < 1 {
            text
            self.complain(&foo);
            text
        }
    }
    text
}
text

text
impl MyStruct<'foo> {
    text
    pub fn with_something<T: Send> (param: &T, f: |i32, &str| -> T, other_param: u32) -> T {
        text
        f(123, "hello")
        text
    }
    text
}
text

// Loop expression labels (#2)
'infinity: loop {
    do_serious_stuff();
    use_a_letter('Z');
    break 'infinity;
}

// isize/usize suffixes (#22)
let x = 123usize;

// Float literals without +- after E (#30)
let x = 1.2345e6;

// Nested generic (#33, #37)
let x: Vec<Vec<u8>> = Vec::new();

// Correct detection of == (#40)
struct Foo { x: i32 }
if x == 1 { }

// const function parameter (#52)
fn foo(bar: *const i32) {
    let _ = 1234 as *const u32;
}

// Keywords and known types in wrapper structs (#56)
pub struct Foobar(pub Option<bool>);

pub struct Foobar(pub Test<W=bool>);
pub struct Foobar(pub Test<W==bool>);
pub struct Foobar(pub Test<W = bool>);

struct Test(i32);
struct Test(String);
struct Test(Vec<i32>);
struct Test(BTreeMap<String, i32>);

// Lifetimes in associated type definitions
trait Foo {
    type B: A + 'static;
}

// where clause
impl Foo<A, B> where text { }
text
impl Foo<A, B> for C where text { }
text
impl Foo<A, B> for C {
    fn foo<A, B> -> C where text { }
}
text
fn foo<A, B> -> C where text { }
text
struct Foo<A, B> where text { }
text
trait Foo<A, B> : C where { }
text

fn do_work<T: Any + Debug>(value: &T) {}

impl Cookie {}
impl<T> PrintInOption for T where Option<T>: Debug {}
impl<K,V> HashMap<K, V> where K : Hash + Eq {}
impl<A, D> MyTrait<A, D> for YourType where A: TraitB + TraitC, D: TraitE + TraitF {}
impl Debug for a where asdf {}

impl<Flavor> Eat where Cookie<Flavor>: Oatmeal {}

// Unsafe in function arguments
unsafe fn foo();
fn foo(f: unsafe fn());

// Format macros
format!("text");
format!("text{}text", 1);
format!("text{0}text", 1);
format!("text{named}text", named=1);
format!("text{:?}text", to_debug);
format!("text{0:?}text", to_debug);
format!("text{named:?}text", named=to_debug);
format!("text{:<}text", 1);
format!("text{:.>}text", 1);
format!("text{:+}text", pos_or_neg);
format!("text{:+0}text", pos_or_neg);
format!("text{:+04}text", pos_or_neg);
format!("text{:6}", "text");
format!("text{:1$}", "text", 6);
format!("text{1:6$}", 6, "text");
format!("text{:w$}", "text", w=6);
format!("text{:8b}text", byte);
format!("text{:08b}text", byte);
format!("text{:#010b}text", byte);
format!("text{:2x}text", byte);
format!("text{:#4x}text", byte);
format!("text{:.2}text", 0.5);
format!("text{:0.2}text", 0.5);
format!("text{:06.2}text", 0.5);
format!("text{named:.prec$}text", prec=0.5, named=2.0);
format!("text{}text{2:.*}text", "text", 6, 0.5);
format!("text{named:-^+#06.2e}text", named=0.5);
format!("text{named:-^+#0width$.prec$e}text", named=0.5, width=6, prec=2);
format!("text{{escaped}}text\n{0}text", "only {text} first {{text}}");

not_format!("text{}text");
format_args!("{}", "text");
write!("{}", "text");
writeln!("{}", "text");
print!("{}", "text");
println!("{}", "text");
log!("{}", "text");
error!("{}", "text");
warn!("{}", "text");
info!("{}", "text");
debug!("{}", "text");
trace!("{}", "text");
eprint!("{}", "text");
eprintln!("{}", "text");

// Unused reserved words

abstract fn do_thing();

final let MAX = 10us;

let i = 0;
do {
    yield do_thing(i++);
} while(i < MAX);

impl Float for f32 {
    fn infinity() -> Self {
        f32::INFINITY
    }
}

#[derive(Default)]
enum State {
    /// Currently open in a sane state
    Open,
    
    /// Waiting to send a GO_AWAY frame
    GoAway(frame::GoAway)
    
    /// Waiting to send a GO_AWAY_ frame
    GoAway(frame::GoAway)
    
    /// Waiting to send a GO*AWAY frame
    GoAway(frame::GoAway)
    
    /// Waiting to send a GO*AWAY* frame
    GoAway(frame::GoAway)
    
    /// Waiting to send a GO__AWAY frame
    GoAway(frame::GoAway)
    
    /// Waiting to send a GO__AWAY__ frame
    GoAway(frame::GoAway)
    
    /// Waiting to send a GO**AWAY frame
    GoAway(frame::GoAway)
    
    /// Waiting to send a GO**AWAY** frame
    GoAway(frame::GoAway)
}
