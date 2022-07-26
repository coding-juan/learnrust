struct StructWithLifeTime<'a> {
    some_str: &'a str,
}

impl<'a> StructWithLifeTime<'a> {
    fn print_content(&self) {
        eprintln!("{}", self.some_str);
    }
}

fn get_longest<'a>(first: &'a StructWithLifeTime, second: &'a StructWithLifeTime) -> &'a StructWithLifeTime<'a> {
    if first.some_str.len() > second.some_str.len() {
        return first;
    } else {
        return second;
    }
}

fn main() {
    let str1 = String::from("This is a test");
    let stc1 = StructWithLifeTime {
        some_str: str1.as_str()
    };
    
    {
        let static_string: &'static str = "hehehe my lifetime is INFINITE";
        let str2 = String::from("This is also, a test");
        let stc2 = StructWithLifeTime {
            some_str: str2.as_str()
        };

        let longest = get_longest(&stc1, &stc2);
        longest.print_content();
    }
}
