mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {
        inside::inner_function();
    }

    pub mod inside {
        pub fn inner_function() {
            ::outermost::middle_secret_function();
        }

        pub fn inner_secet_funtion() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::inside::inner_function();
    outermost::inside::inner_secet_funtion();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
