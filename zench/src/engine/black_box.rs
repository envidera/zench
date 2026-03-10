/// ## Black Box
///
/// Compiler optimization barrier
///
/// A wrapper for:
///     std::hint::black_box()
///
/// /// # Example
/// ```rust,ignore
/// use zench::bx;
/// bx(42);
/// ```
pub fn bx<T>(dummy: T) -> T {
    std::hint::black_box(dummy)
}

/* #[inline(never)]
pub fn bx_extra<T>(dummy: T) -> T {
    extra_barrier(|| bx(dummy))
}

#[inline(never)]
fn extra_barrier<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    f()
}
 */
