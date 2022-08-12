mod asgardian;
mod transport;
mod protobuf_messages;
mod messages;
mod role;
mod asgard_error;
mod log;
mod asgard_data;
mod common;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
