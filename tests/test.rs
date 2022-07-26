
#[cfg(test)]
#[macro_use]
mod tests {
    use enumn::N;

    // Test an empty enum
    #[derive(Debug, N, PartialEq, Eq)]
    enum EmptyEnum{}

    #[test]
    fn test_empty() {
        assert_eq!(EmptyEnum::n(0), None);
        assert_eq!(EmptyEnum::n(1), None);
        assert_eq!(EmptyEnum::n(-1), None);
    }

    // Test a simple enum
    #[derive(Debug, N, PartialEq, Eq)]
    enum SimpleEnum{
        Case0,
        Case1,
        Case2
    }

    #[test]
    fn test_simple() {
        assert_eq!(SimpleEnum::n(0), Some(SimpleEnum::Case0));
        assert_eq!(SimpleEnum::n(1), Some(SimpleEnum::Case1));
        assert_eq!(SimpleEnum::n(2), Some(SimpleEnum::Case2));
        assert_eq!(SimpleEnum::n(4), None);
        assert_eq!(SimpleEnum::n(-1), None);
    }

    // Test an enum with repr(u8)
    #[derive(Debug, N, PartialEq, Eq)]
    #[repr(u8)]
    enum SimpleEnumWithRepr{
        Case0,
        Case1,
        Case2
    }

    #[test]
    fn test_simple_with_repr() {
        assert_eq!(SimpleEnumWithRepr::n(0), Some(SimpleEnumWithRepr::Case0));
        assert_eq!(SimpleEnumWithRepr::n(1), Some(SimpleEnumWithRepr::Case1));
        assert_eq!(SimpleEnumWithRepr::n(2), Some(SimpleEnumWithRepr::Case2));
        assert_eq!(SimpleEnumWithRepr::n(255), None);
    }

    // Test an enum with discriminant
    #[derive(Debug, N, PartialEq, Eq)]
    enum EnumWithDiscriminant {
        A = 10,
        B = 290,
        C = -10
    }

    #[test]
    fn test_discriminant() {
        assert_eq!(EnumWithDiscriminant::n(10), Some(EnumWithDiscriminant::A));
        assert_eq!(EnumWithDiscriminant::n(290), Some(EnumWithDiscriminant::B));
        assert_eq!(EnumWithDiscriminant::n(-10), Some(EnumWithDiscriminant::C));
        assert_eq!(EnumWithDiscriminant::n(0), None);
        assert_eq!(EnumWithDiscriminant::n(9), None);
        assert_eq!(EnumWithDiscriminant::n(291), None);
        assert_eq!(EnumWithDiscriminant::n(-11), None);
    }

    // Test an enum with discriminant and implicit values
    #[derive(Debug, N, PartialEq, Eq)]
    enum MixedDiscriminant {
        A = 10,
        B, // B will implicitly be set to 11
        C = 80
    }

    #[test]
    fn test_mixed() {
        assert_eq!(MixedDiscriminant::n(10), Some(MixedDiscriminant::A));
        assert_eq!(MixedDiscriminant::n(11), Some(MixedDiscriminant::B));
        assert_eq!(MixedDiscriminant::n(80), Some(MixedDiscriminant::C));
        assert_eq!(MixedDiscriminant::n(9), None);
        assert_eq!(MixedDiscriminant::n(0), None);
        assert_eq!(MixedDiscriminant::n(1), None);
        assert_eq!(MixedDiscriminant::n(81), None);
    }


}   