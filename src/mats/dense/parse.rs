use std::str::FromStr;

/// Parses a given string: `s` using the given separator: `sep` and
/// returns the data and dimensions of the given matrix.
///
/// # Usage
///
/// ```ignore
/// use mat_lib::mats::dense::parse::mat_from_str;
/// let (data, rows, cols) = mat_from_str::<i32>("1 2 3; 4 5 6", ";").unwrap();
/// assert_eq!(vec![1, 2, 3, 4, 5, 6], data);
/// assert_eq!(2, rows);
/// assert_eq!(3, cols);
///
/// let (data, rows, cols) = mat_from_str::<f32>("1 2 sep 3 4", "sep").unwrap();
/// assert_eq!(vec![1.0, 2.0, 3.0, 4.0], data);
/// assert_eq!(2, rows);
/// assert_eq!(2, cols);
/// ```
///
/// # Errors
/// 1. Separator `sep` is all whitespace.
/// 1. At least one of the given entries is not the same type as the given `T`.
/// 1. The shape of the matrix is invalid.
///
/// ## Example
/// ```ignore
/// use mat_lib::parse::mat_from_str;
///
/// // Sep is whitespace.
/// assert!(mat_from_str::<i32>("1 2 3, 4 5 6", " ").is_err());
///
/// // Different type for entris.
/// assert!(mat_from_str::<u128>("1 2 3, d 5 6", ",").is_err());
///
/// // Invalid matrix shape.
/// assert!(mat_from_str::<i32>("1 2 3, 4 5", ",").is_err());
/// ```
///
pub fn mat_from_str<T: FromStr>(
    s: &str,
    sep: &str,
) -> Result<(Vec<T>, usize, usize), &'static str> {
    if sep.trim().is_empty() {
        return Err("Invalid separator");
    }

    if s.is_empty() {
        return Err("Given string is empty");
    }

    let mut data = vec![];
    let mut rows = 0;
    let mut cols = 0;

    for row in s.split(sep) {
        let mut read = 0;
        rows += 1;

        for num in row.split_whitespace() {
            match num.parse::<T>() {
                Ok(num) => {
                    data.push(num);
                    read += 1;
                }

                Err(_) => return Err("Invalid syntax in string"),
            };
        }

        if cols == 0 {
            cols = read;
        } else if cols != read {
            return Err("Invalid shape for matrix");
        }
    }

    Ok((data, rows, cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid1() {
        let (data, rows, cols) = mat_from_str::<i32>("1     2 3;4 5   6 ; 7  8 9", ";").unwrap();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], data);
        assert_eq!(3, rows);
        assert_eq!(3, cols);
    }

    #[test]
    fn valid2() {
        let (data, rows, cols) = mat_from_str::<f32>("1    2,3 4,5  6", ",").unwrap();
        assert_eq!(vec![1., 2., 3., 4., 5., 6.], data);
        assert_eq!(3, rows);
        assert_eq!(2, cols);
    }

    #[test]
    fn valid3() {
        let (data, rows, cols) = mat_from_str::<f64>("1 2 3. 4 5 6", ".").unwrap();
        assert_eq!(vec![1., 2., 3., 4., 5., 6.], data);
        assert_eq!(2, rows);
        assert_eq!(3, cols);
    }

    #[test]
    fn invalid() {
        assert!(mat_from_str::<i32>("1 2 3; 4 5", ";").is_err());
        assert!(mat_from_str::<u16>("1; 3 4; 5", ";").is_err());
        assert!(mat_from_str::<f64>("1 2 3 4 5; a", ";").is_err());
        assert!(mat_from_str::<f32>("1 2; b 4", ";").is_err());
        assert!(mat_from_str::<i64>("", ".").is_err());
    }
}
