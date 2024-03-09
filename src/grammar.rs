// By CKKitty on 2024-Mar-09
// Does the actual parsing

peg::parser!{
    pub grammar text_parser() for str {
        rule number() -> u32
        = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        pub rule text() -> Vec<u32>
        = "[" l:(number() ** ",") "]" { l }
    }
}