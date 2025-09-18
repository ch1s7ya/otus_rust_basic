use ::std::time::{Duration, Instant};
pub fn sum(a: u64, b: u64) -> u64 {
    a + b
}

pub fn concat(a: &[&str]) -> String {
    a.join(" ")
}

pub fn measure_time<F, T, R>(f: F, args: T) -> (R, Duration)
where
    F: FnOnce(T) -> R,
{
    let now = Instant::now();
    let result = f(args);
    (result, now.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_concat() {
        let result = concat(&["hello", "bender"]);
        assert_eq!(result, "hello bender");
    }

    #[test]
    fn test_sum_measure_time() {
        let (res, execution_time) = measure_time(|(a, b)| sum(a, b), (10, 20));
        assert_eq!(30, res);
        println!("sum took {execution_time:?}");
    }

    #[test]
    fn test_concat_measure_time() {
        let (res, execution_time) = measure_time(concat, &["hello", "world"]);
        assert_eq!("hello world", res);
        println!("concat took {execution_time:?}");
    }

    #[test]
    fn test_println_measure_time() {
        let (_, execution_time) = measure_time(
            |_| {
                println!("printing to stdout");
            },
            (),
        );
        println!("printing took {execution_time:?}");
    }
}
