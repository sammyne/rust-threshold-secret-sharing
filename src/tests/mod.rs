pub fn do_rsgx_tests() -> usize {
    crate::fields::montgomery::tests::do_rsgx_tests()
        + crate::fields::native::tests::do_rsgx_tests()
        + crate::numtheory::tests::do_rsgx_tests()
        + crate::packed::tests::do_rsgx_tests()
        + crate::shamir::tests::do_rsgx_tests()
}
