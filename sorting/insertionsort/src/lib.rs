pub fn sort<T: PartialOrd>(&mut [T]: coll) {
    
}


#[cfg(test)]
mod tests {
    #[test]
    fn sorts_integers() {
        let mut integers : [i32; 5] = [2,5,1,3,4];
        super::sort(integers);
        assert_eq!(integers, [1,2,3,4,5]);
    }
}
