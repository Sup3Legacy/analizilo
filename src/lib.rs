pub mod word;

#[cfg(test)]
mod tests {
    #[test]
    fn prefix_parsing_test() {
        let truc = crate::word::token::parse_src("eksterbonegege");
        println!("Got {:#?}", truc);
    }
}
