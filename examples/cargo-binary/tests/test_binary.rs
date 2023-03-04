fn main() {
    #[cfg(test)]
    mod tests {

        #[test]
        fn test_sum() {
            assert_eq!(2 + 3, 5);
        }
    }
}
