use crate::substance::BackendName;
use crate::uom::si::f64::Ratio;
use crate::uom::si::ratio::ratio;
use std::str::FromStr;
use strum::EnumProperty;
#[cfg(test)]
use strum_macros::EnumIter;
use strum_macros::{AsRefStr, EnumProperty, EnumString};

/// CoolProp incompressible binary mixtures _(mass-based or volume-based)_.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::substance::BinaryMix;
///
/// assert_eq!(BinaryMix::MPG.as_ref(), "MPG");
/// assert_eq!(BinaryMix::from_str("MPG"), Ok(BinaryMix::MPG));
/// assert_eq!(BinaryMix::try_from("MPG"), Ok(BinaryMix::MPG));
/// ```
///
/// # See also
///
/// - [Incompressible substances](https://coolprop.github.io/CoolProp/fluid_properties/Incomps.html)
//noinspection SpellCheckingInspection
#[derive(AsRefStr, EnumString, EnumProperty, Debug, Copy, Clone, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(test, derive(EnumIter))]
pub enum BinaryMix {
    #[strum(to_string = "FRE", props(min_fraction = "0.19", max_fraction = "0.5"))]
    FRE,

    #[strum(
        to_string = "IceEA",
        props(min_fraction = "0.05", max_fraction = "0.35")
    )]
    IceEA,

    #[strum(
        to_string = "IceNA",
        props(min_fraction = "0.05", max_fraction = "0.35")
    )]
    IceNA,

    #[strum(
        to_string = "IcePG",
        props(min_fraction = "0.05", max_fraction = "0.35")
    )]
    IcePG,

    #[strum(to_string = "LiBr", props(min_fraction = "0.0", max_fraction = "0.75"))]
    LiBr,

    #[strum(to_string = "MAM", props(min_fraction = "0.0", max_fraction = "0.3"))]
    MAM,

    #[strum(
        to_string = "MAM2",
        props(min_fraction = "0.078", max_fraction = "0.236")
    )]
    MAM2,

    #[strum(to_string = "MCA", props(min_fraction = "0.0", max_fraction = "0.3"))]
    MCA,

    #[strum(
        to_string = "MCA2",
        props(min_fraction = "0.09", max_fraction = "0.294")
    )]
    MCA2,

    #[strum(to_string = "MEA", props(min_fraction = "0.0", max_fraction = "0.6"))]
    MEA,

    #[strum(to_string = "MEA2", props(min_fraction = "0.11", max_fraction = "0.6"))]
    MEA2,

    #[strum(to_string = "MEG", props(min_fraction = "0.0", max_fraction = "0.6"))]
    MEG,

    #[strum(to_string = "MEG2", props(min_fraction = "0.0", max_fraction = "0.56"))]
    MEG2,

    #[strum(to_string = "MGL", props(min_fraction = "0.0", max_fraction = "0.6"))]
    MGL,

    #[strum(
        to_string = "MGL2",
        props(min_fraction = "0.195", max_fraction = "0.63")
    )]
    MGL2,

    #[strum(
        to_string = "MITSW",
        props(min_fraction = "0.0", max_fraction = "0.12")
    )]
    MITSW,

    #[strum(to_string = "MKA", props(min_fraction = "0.0", max_fraction = "0.45"))]
    MKA,

    #[strum(
        to_string = "MKA2",
        props(min_fraction = "0.11", max_fraction = "0.41")
    )]
    MKA2,

    #[strum(to_string = "MKC", props(min_fraction = "0.0", max_fraction = "0.4"))]
    MKC,

    #[strum(to_string = "MKC2", props(min_fraction = "0.0", max_fraction = "0.39"))]
    MKC2,

    #[strum(to_string = "MKF", props(min_fraction = "0.0", max_fraction = "0.48"))]
    MKF,

    #[strum(to_string = "MLI", props(min_fraction = "0.0", max_fraction = "0.24"))]
    MLI,

    #[strum(to_string = "MMA", props(min_fraction = "0.0", max_fraction = "0.6"))]
    MMA,

    #[strum(
        to_string = "MMA2",
        props(min_fraction = "0.078", max_fraction = "0.474")
    )]
    MMA2,

    #[strum(to_string = "MMG", props(min_fraction = "0.0", max_fraction = "0.3"))]
    MMG,

    #[strum(
        to_string = "MMG2",
        props(min_fraction = "0.0", max_fraction = "0.205")
    )]
    MMG2,

    #[strum(to_string = "MNA", props(min_fraction = "0.0", max_fraction = "0.23"))]
    MNA,

    #[strum(to_string = "MNA2", props(min_fraction = "0.0", max_fraction = "0.23"))]
    MNA2,

    #[strum(to_string = "MPG", props(min_fraction = "0.0", max_fraction = "0.6"))]
    MPG,

    #[strum(
        to_string = "MPG2",
        props(min_fraction = "0.15", max_fraction = "0.57")
    )]
    MPG2,

    #[strum(
        to_string = "VCA",
        props(min_fraction = "0.147", max_fraction = "0.299")
    )]
    VCA,

    #[strum(
        to_string = "VKC",
        props(min_fraction = "0.128", max_fraction = "0.389")
    )]
    VKC,

    #[strum(to_string = "VMA", props(min_fraction = "0.1", max_fraction = "0.9"))]
    VMA,

    #[strum(
        to_string = "VMG",
        props(min_fraction = "0.072", max_fraction = "0.206")
    )]
    VMG,

    #[strum(
        to_string = "VNA",
        props(min_fraction = "0.07", max_fraction = "0.231")
    )]
    VNA,

    #[strum(to_string = "AEG", props(min_fraction = "0.1", max_fraction = "0.6"))]
    AEG,

    #[strum(to_string = "AKF", props(min_fraction = "0.4", max_fraction = "1.0"))]
    AKF,

    #[strum(to_string = "AL", props(min_fraction = "0.1", max_fraction = "0.6"))]
    AL,

    #[strum(to_string = "AN", props(min_fraction = "0.1", max_fraction = "0.6"))]
    AN,

    #[strum(to_string = "APG", props(min_fraction = "0.1", max_fraction = "0.6"))]
    APG,

    #[strum(to_string = "GKN", props(min_fraction = "0.1", max_fraction = "0.6"))]
    GKN,

    #[strum(to_string = "PK2", props(min_fraction = "0.3", max_fraction = "1.0"))]
    PK2,

    #[strum(to_string = "PKL", props(min_fraction = "0.1", max_fraction = "0.6"))]
    PKL,

    #[strum(to_string = "ZAC", props(min_fraction = "0.06", max_fraction = "0.5"))]
    ZAC,

    #[strum(to_string = "ZFC", props(min_fraction = "0.3", max_fraction = "0.6"))]
    ZFC,

    #[strum(to_string = "ZLC", props(min_fraction = "0.3", max_fraction = "0.7"))]
    ZLC,

    #[strum(to_string = "ZM", props(min_fraction = "0.0", max_fraction = "1.0"))]
    ZM,

    #[strum(to_string = "ZMC", props(min_fraction = "0.3", max_fraction = "0.7"))]
    ZMC,
}

impl BinaryMix {
    /// Minimum possible fraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance;
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// assert_eq!(substance::BinaryMix::MPG.min_fraction(), Ratio::new::<percent>(0.0));
    /// ```
    pub fn min_fraction(&self) -> Ratio {
        Ratio::new::<ratio>(f64::from_str(self.get_str("min_fraction").unwrap()).unwrap())
    }

    /// Maximum possible fraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance;
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    ///
    /// assert_eq!(substance::BinaryMix::MPG.max_fraction(), Ratio::new::<percent>(60.0));
    /// ```
    pub fn max_fraction(&self) -> Ratio {
        Ratio::new::<ratio>(f64::from_str(self.get_str("max_fraction").unwrap()).unwrap())
    }
}

impl BackendName for BinaryMix {
    fn backend_name(&self) -> &'static str {
        "INCOMP"
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryMix::*;
    use super::*;
    use rstest::*;
    use strum::IntoEnumIterator;

    #[rstest]
    #[case(FRE, 0.19, 0.5)]
    #[case(IceEA, 0.05, 0.35)]
    #[case(IceNA, 0.05, 0.35)]
    #[case(IcePG, 0.05, 0.35)]
    #[case(LiBr, 0.0, 0.75)]
    #[case(MAM, 0.0, 0.3)]
    #[case(MAM2, 0.078, 0.236)]
    #[case(MCA, 0.0, 0.3)]
    #[case(MCA2, 0.09, 0.294)]
    #[case(MEA, 0.0, 0.6)]
    #[case(MEA2, 0.11, 0.6)]
    #[case(MEG, 0.0, 0.6)]
    #[case(MEG2, 0.0, 0.56)]
    #[case(MGL, 0.0, 0.6)]
    #[case(MGL2, 0.195, 0.63)]
    #[case(MITSW, 0.0, 0.12)]
    #[case(MKA, 0.0, 0.45)]
    #[case(MKA2, 0.11, 0.41)]
    #[case(MKC, 0.0, 0.4)]
    #[case(MKC2, 0.0, 0.39)]
    #[case(MKF, 0.0, 0.48)]
    #[case(MLI, 0.0, 0.24)]
    #[case(MMA, 0.0, 0.6)]
    #[case(MMA2, 0.078, 0.474)]
    #[case(MMG, 0.0, 0.3)]
    #[case(MMG2, 0.0, 0.205)]
    #[case(MNA, 0.0, 0.23)]
    #[case(MNA2, 0.0, 0.23)]
    #[case(MPG, 0.0, 0.6)]
    #[case(MPG2, 0.15, 0.57)]
    #[case(VCA, 0.147, 0.299)]
    #[case(VKC, 0.128, 0.389)]
    #[case(VMA, 0.1, 0.9)]
    #[case(VMG, 0.072, 0.206)]
    #[case(VNA, 0.07, 0.231)]
    #[case(AEG, 0.1, 0.6)]
    #[case(AKF, 0.4, 1.0)]
    #[case(AL, 0.1, 0.6)]
    #[case(AN, 0.1, 0.6)]
    #[case(APG, 0.1, 0.6)]
    #[case(GKN, 0.1, 0.6)]
    #[case(PK2, 0.3, 1.0)]
    #[case(PKL, 0.1, 0.6)]
    #[case(ZAC, 0.06, 0.5)]
    #[case(ZFC, 0.3, 0.6)]
    #[case(ZLC, 0.3, 0.7)]
    #[case(ZM, 0.0, 1.0)]
    #[case(ZMC, 0.3, 0.7)]
    fn min_max_fractions_always_returns_expected_values(
        #[case] substance: BinaryMix,
        #[case] expected_min_fraction: f64,
        #[case] expected_max_fraction: f64,
    ) {
        assert_eq!(
            substance.min_fraction(),
            Ratio::new::<ratio>(expected_min_fraction)
        );
        assert_eq!(
            substance.max_fraction(),
            Ratio::new::<ratio>(expected_max_fraction)
        );
    }

    #[test]
    fn backend_name_always_returns_incomp() {
        for substance in BinaryMix::iter() {
            assert_eq!(substance.backend_name(), "INCOMP");
        }
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case(FRE, "FRE")]
    #[case(IceEA, "IceEA")]
    #[case(IceNA, "IceNA")]
    #[case(IcePG, "IcePG")]
    #[case(LiBr, "LiBr")]
    #[case(MAM, "MAM")]
    #[case(MAM2, "MAM2")]
    #[case(MCA, "MCA")]
    #[case(MCA2, "MCA2")]
    #[case(MEA, "MEA")]
    #[case(MEA2, "MEA2")]
    #[case(MEG, "MEG")]
    #[case(MEG2, "MEG2")]
    #[case(MGL, "MGL")]
    #[case(MGL2, "MGL2")]
    #[case(MITSW, "MITSW")]
    #[case(MKA, "MKA")]
    #[case(MKA2, "MKA2")]
    #[case(MKC, "MKC")]
    #[case(MKC2, "MKC2")]
    #[case(MKF, "MKF")]
    #[case(MLI, "MLI")]
    #[case(MMA, "MMA")]
    #[case(MMA2, "MMA2")]
    #[case(MMG, "MMG")]
    #[case(MMG2, "MMG2")]
    #[case(MNA, "MNA")]
    #[case(MNA2, "MNA2")]
    #[case(MPG, "MPG")]
    #[case(MPG2, "MPG2")]
    #[case(VCA, "VCA")]
    #[case(VKC, "VKC")]
    #[case(VMA, "VMA")]
    #[case(VMG, "VMG")]
    #[case(VNA, "VNA")]
    #[case(AEG, "AEG")]
    #[case(AKF, "AKF")]
    #[case(AL, "AL")]
    #[case(AN, "AN")]
    #[case(APG, "APG")]
    #[case(GKN, "GKN")]
    #[case(PK2, "PK2")]
    #[case(PKL, "PKL")]
    #[case(ZAC, "ZAC")]
    #[case(ZFC, "ZFC")]
    #[case(ZLC, "ZLC")]
    #[case(ZM, "ZM")]
    #[case(ZMC, "ZMC")]
    fn as_ref_always_returns_expected_str(#[case] substance: BinaryMix, #[case] expected: &str) {
        assert_eq!(substance.as_ref(), expected);
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case("FRE", FRE)]
    #[case("IceEA", IceEA)]
    #[case("IceNA", IceNA)]
    #[case("IcePG", IcePG)]
    #[case("LiBr", LiBr)]
    #[case("MAM", MAM)]
    #[case("MAM2", MAM2)]
    #[case("MCA", MCA)]
    #[case("MCA2", MCA2)]
    #[case("MEA", MEA)]
    #[case("MEA2", MEA2)]
    #[case("MEG", MEG)]
    #[case("MEG2", MEG2)]
    #[case("MGL", MGL)]
    #[case("MGL2", MGL2)]
    #[case("MITSW", MITSW)]
    #[case("MKA", MKA)]
    #[case("MKA2", MKA2)]
    #[case("MKC", MKC)]
    #[case("MKC2", MKC2)]
    #[case("MKF", MKF)]
    #[case("MLI", MLI)]
    #[case("MMA", MMA)]
    #[case("MMA2", MMA2)]
    #[case("MMG", MMG)]
    #[case("MMG2", MMG2)]
    #[case("MNA", MNA)]
    #[case("MNA2", MNA2)]
    #[case("MPG", MPG)]
    #[case("MPG2", MPG2)]
    #[case("VCA", VCA)]
    #[case("VKC", VKC)]
    #[case("VMA", VMA)]
    #[case("VMG", VMG)]
    #[case("VNA", VNA)]
    #[case("AEG", AEG)]
    #[case("AKF", AKF)]
    #[case("AL", AL)]
    #[case("AN", AN)]
    #[case("APG", APG)]
    #[case("GKN", GKN)]
    #[case("PK2", PK2)]
    #[case("PKL", PKL)]
    #[case("ZAC", ZAC)]
    #[case("ZFC", ZFC)]
    #[case("ZLC", ZLC)]
    #[case("ZM", ZM)]
    #[case("ZMC", ZMC)]
    fn from_valid_str_returns_ok(#[case] valid_value: &str, #[case] expected: BinaryMix) {
        assert_eq!(BinaryMix::from_str(valid_value), Ok(expected));
        assert_eq!(BinaryMix::try_from(valid_value), Ok(expected));
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str_returns_err(#[case] invalid_value: &str) {
        assert!(BinaryMix::from_str(invalid_value).is_err());
        assert!(BinaryMix::try_from(invalid_value).is_err());
    }
}
