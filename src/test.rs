// Fix the `fetch_last` function. Do not add any other statement.

fn fetch_last<T>(list: &mut Vec<T>) -> Option<T> {
    list.pop()
}

fn main() {
    let mut my_nums = Vec::<i32>::new();
    match fetch_last(&mut my_nums) {
        Ok(ele) => println!("Last element: {ele}"),
        Err(error) => {
            println!("Error: {error}");
            assert_eq!(error, "Empty list".to_owned());
        }
    }
}
