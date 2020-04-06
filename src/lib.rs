extern crate chrono;

pub mod entity;
mod extensions;

fn add_one (input: isize) -> isize {
    unimplemented!()
}

fn not (input: bool) -> bool {
    unimplemented!()
}

fn list_add_one (input: &Vec<isize>) -> Vec<isize> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_succeeds () {
        // assign
        let input = 0;
        let expected = 1;

        // act
        let output = add_one(input);

        // assert
        assert_eq!(output, expected);
    }

    #[test]
    fn not_succeeds () {
        // assign
        let input = true;
        let expected = false;

        // act
        let output = not(input);

        // assert
        assert_eq!(output, expected);
    }

    #[test]
    fn list_add_one_succeeds () {
        // assign
        let input = vec![1, 2, 3];
        let expected = vec![2, 3, 4];

        // act
        let output = list_add_one(&input);

        // assert
        for (&a, &b) in output.iter().zip(expected.iter()) {
            assert_eq!(a, b);
        }
    }
}
