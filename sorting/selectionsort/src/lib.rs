pub fn sort<T>(values: &mut [T])
where
    T: Ord,
{
    for i in 0..values.len() {
        let mut min = i;
        for j in i + 1..values.len() {
            if values[j] < values[min] {
                min = j;
            }
        }
        values.swap(i, min);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sorts_integers() {
        let mut integers: [i32; 5] = [2, 5, 1, 3, 4];
        super::sort(&mut integers);
        assert_eq!(integers, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_strings() {
        let mut strings: [&str; 5] = ["hey", "how", "are", "you", "today"];
        super::sort(&mut strings);
        assert_eq!(strings, ["are", "hey", "how", "today", "you"]);
    }

    #[test]
    fn sorts_chars() {
        let mut chars: [char; 13] = [
            'S', 'E', 'L', 'E', 'C', 'T', 'I', 'O', 'N', 'S', 'O', 'R', 'T',
        ];
        super::sort(&mut chars);
        assert_eq!(
            chars,
            ['C', 'E', 'E', 'I', 'L', 'N', 'O', 'O', 'R', 'S', 'S', 'T', 'T']
        );
    }
}
