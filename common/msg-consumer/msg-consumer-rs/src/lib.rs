fn say_hello(to: &str) -> String {
    let mut msg = String::from("Hello, ");
    msg.push_str(to);
    msg
}

#[cfg(test)]
mod tests {
    use crate::say_hello;

    #[test]
    fn hello_to_me() {
        assert_eq!(say_hello("me"), "Hello, me");
    }
}
