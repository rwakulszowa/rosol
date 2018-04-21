pub fn selections<T: Clone>(mut sources: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if sources.len() == 0 {
        vec![vec![]]
    } else {
        let head = sources.remove(0);
        // sources is 1-element shorter now
        let subresults = selections(sources);

        let mut ans: Vec<Vec<T>> = Vec::new();

        for head_element in head.iter() {
            for result in subresults.iter() {
                let mut selection = vec![head_element.clone()];
                selection.extend(result.clone());
                ans.push(selection);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    fn assert_selects<T: Clone + Debug + Eq>(sources: Vec<Vec<T>>, expected: Vec<Vec<T>>) {
        assert_eq!(
            selections(sources),
            expected);
    }

    #[test]
    fn test_selections() {
        assert_selects::<u8>(
            vec![],
            vec![vec![]]
        );

        assert_selects(
            vec![vec![1]],
            vec![vec![1]]
        );

        assert_selects(
            vec![vec![1, 2], vec![3, 4]],
            vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]]
        );

        assert_selects(
            vec![vec![1, 2], vec![3], vec![4]],
            vec![vec![1, 3, 4], vec![2, 3, 4]]
        );
    }
}
