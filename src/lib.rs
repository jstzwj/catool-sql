pub mod db;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{}", super::db::table("abc").select().column("a").get_sql());
        assert_eq!(1+1, 2);
    }
}
