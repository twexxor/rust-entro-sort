pub mod entro_sort;

#[cfg(test)]
mod test {
    use entro_sort;

    #[test]
    fn test_entro_sort_ascending() {
        let mut input: [u8;10] = [24, 132, 22, 217, 185, 9, 65, 245, 88, 148];
        let mut i: usize = 1;

        entro_sort::entro_sort_ascending(&mut input);

        while i != input.len() {
            assert_eq!(true, input[i] > input[i - 1]);
            i += 1;
        }
    }

    #[test]
    fn test_entro_sort_descending() {
        let mut input: [u8;10] = [24, 132, 22, 217, 185, 9, 65, 245, 88, 148];
        let mut i: usize = 1;

        entro_sort::entro_sort_descending(&mut input);

        while i != input.len() {
            assert_eq!(true, input[i] < input[i - 1]);
            i += 1;
        }
    }
}
