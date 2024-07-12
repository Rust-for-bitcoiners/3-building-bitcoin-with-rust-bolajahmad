#![allow(unused)]

// #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[derive(PartialEq, Eq, Debug)]
enum MResult<T, E> {
    // holds the success value
    Ok(T),
    // holds the error value
    Err(E),
}

impl<T, E> MResult<T, E> {
    // This creates a variant of MResult.
    // Returns a MResult::Ok(value)
    // Use as let ok_value = MResult::ok(10);
    fn ok(value: T) -> Self {
        MResult::Ok(value)
    }
    // Function to create an Err variant of MResult
    // Returns a MResult::Err(error)
    // Use as let err_value = MResult::err("Error message");
    fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        match self {
            MResult::Ok(_) => true,
            MResult::Err(_) => false,
        }
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        !self.is_ok()
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        match self {
            MResult::Ok(value) => value,
            MResult::Err(_) => panic!("Called `unwrap` on an `Err` value"),
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        match self {
            MResult::Ok(_) => panic!("Called `unwrap_err` on an `Ok` value"),
            MResult::Err(error) => error,
        }
    }
}

// Add unit tests below
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let ok_value: MResult<u64, u64> = MResult::ok(10);
        assert_eq!(ok_value, MResult::Ok(10));
        assert_eq!(ok_value.is_ok(), true);
    }

    #[test]
    fn test_err() {
        let err_value: MResult<u64, &str> = MResult::err("Error message");
        assert_eq!(err_value, MResult::Err("Error message"));
        assert_eq!(err_value.is_err(), true);
    }

    // This test covers also the is_err variant of it.
    #[test]
    fn test_is_ok() {
        let ok_value: MResult<u64, u64> = MResult::ok(10);
        assert_eq!(ok_value.is_ok(), true);
    }

    #[test]
    fn test_unwrap() {
        let ok_value: MResult<u64, u64> = MResult::ok(10);
        assert_eq!(ok_value, MResult::Ok(10));
        assert_eq!(ok_value.unwrap(), 10);
    }

    #[test]
    #[should_panic(expected = "Called `unwrap` on an `Err` value")]
    fn test_unwrap_panics() {
        let err_value: MResult<u64, u64> = MResult::err(10);
        assert_eq!(err_value, MResult::Err(10));
        err_value.unwrap();
    }

    #[test]
    fn test_unwrap_err() {
        let err_value: MResult<u64, &str> = MResult::err("Error happened!");
        assert_eq!(err_value.is_err(), true);
        err_value.unwrap_err();
    }

    #[test]
    #[should_panic( expected = "Called `unwrap_err` on an `Ok` value" )]
    fn test_unwrap_err_panics() {
        let ok_value: MResult<u64, &str> = MResult::ok(18);
        assert!(ok_value.is_ok());
        ok_value.unwrap_err();
    }
}