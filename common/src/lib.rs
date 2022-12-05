

pub mod file_util {
    use std::fs;

    // read a file as string and return an iterator that returns each elements

    
    pub fn read_file<'a>(filename: &str, split_token: &str) -> Vec<String> {
        fs::read_to_string(filename)
            .expect("Something went wrong while reading the file")
            .split(split_token)
            .map(|a| a.to_string())
            .collect()
    }

    pub fn read_to_casted_vec<T: std::str::FromStr>(filename: &str) -> Vec<T>
    where T: std::str::FromStr,
         <T as std::str::FromStr>::Err: std::fmt::Debug
    {
        return read_file(filename, "\n").iter().map(|s| s.parse::<T>().unwrap() ).collect();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
