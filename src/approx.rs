#[macro_export]
macro_rules! assert_approx {
    ($a:expr, $b:expr, $tol:expr) => {
        if ($a - $b).abs() > $tol {
            panic!(
                "assertion failed: `abs(left - right) <= tol`\n  left: `{:?}`\n right: `{:?}`",
                $a, $b
            )
        }
    };
}
