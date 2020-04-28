pub fn sort<T>(values: &mut [T]) where T: Ord {
    for i in 1..values.len() {
        for j in (1..=i).rev() {
            if values[j] >= values[j-1] {
                break;
            }

            values.swap(j, j-1);
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn sorts_integers() {
        let mut integers : [i32; 5] = [2,5,1,3,4];
        super::sort(&mut integers);
        assert_eq!(integers, [1,2,3,4,5]);
    }

    #[test]
    fn sorts_strings() {
        let mut strings : [&str; 5] = ["hey","how","are","you","today"];
        super::sort(&mut strings);
        assert_eq!(strings, ["are", "hey", "how", "today", "you"]);
    }
}
