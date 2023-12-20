pub trait Day {
    fn new() -> Self;
}

pub trait Solve {
    fn solve(&self) -> String;
}

#[macro_export]
macro_rules! impl_day{
    ($day:ident) => {
        pub struct $day {
            file_contents: Vec<String>,
        }

        impl Day for $day {
            fn new() -> Self {
                let file_path = concat!("resources/", stringify!($day), ".txt");
                let file_contents = std::fs::read_to_string(&file_path).expect(&file_path).lines().map(String::from).collect();

                Self {
                    file_contents
                }
            }
        }
    }
}
