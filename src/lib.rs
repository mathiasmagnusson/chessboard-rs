#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(0b11111111 ^ 7, 0xf8);
    }
}
