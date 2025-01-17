use crate::error::CustomMixError;
use crate::io::FluidTrivialParam::MolarMass;
use crate::native::AbstractState;
use crate::substance::{BackendName, Pure, Refrigerant, RefrigerantCategory};
use crate::uom::si::f64::Ratio;
use crate::uom::si::ratio::ratio;
use crate::uom::ConstZero;
use std::collections::HashMap;

/// CoolProp custom mixture
/// _(only pure substances and pure refrigerants are supported)_.
///
/// # See also
///
/// - [Custom mixtures](https://coolprop.github.io/CoolProp/fluid_properties/Mixtures.html)
#[derive(Debug, Clone, PartialEq)]
pub enum CustomMix {
    /// Mole-based mixture _(with mole fractions)_.
    #[non_exhaustive]
    MoleBased(HashMap<CustomMixComponent, Ratio>),

    /// Mass-based mixture _(with mass fractions)_.
    #[non_exhaustive]
    MassBased(HashMap<CustomMixComponent, Ratio>),
}

impl CustomMix {
    /// Creates and returns a new [`CustomMix::MoleBased`] instance.
    ///
    /// # Args
    ///
    /// - `components` -- hash map of components and their _mole_ fractions.
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CustomMixError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance::{CustomMix, Pure, Refrigerant};
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    /// use std::collections::HashMap;
    ///
    /// assert!(CustomMix::mole_based(HashMap::from([
    ///     (Pure::Water.into(), Ratio::new::<percent>(80.0)),
    ///     (Pure::Ethanol.into(), Ratio::new::<percent>(20.0)),
    /// ]))
    /// .is_ok());
    ///
    /// assert!(CustomMix::mole_based(HashMap::from([
    ///     (Refrigerant::R32.into(), Ratio::new::<percent>(70.0)),
    ///     (Refrigerant::R125.into(), Ratio::new::<percent>(30.0)),
    /// ]))
    /// .is_ok());
    /// ```
    pub fn mole_based(
        components: HashMap<CustomMixComponent, Ratio>,
    ) -> Result<Self, CustomMixError> {
        Self::validate(&components)?;
        Ok(Self::MoleBased(components))
    }

    /// Creates and returns a new [`CustomMix::MassBased`] instance.
    ///
    /// # Args
    ///
    /// - `components` -- hash map of components and their _mass_ fractions.
    ///
    /// # Errors
    ///
    /// For invalid inputs, a [`CustomMixError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance::{CustomMix, Pure, Refrigerant};
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    /// use std::collections::HashMap;
    ///
    /// assert!(CustomMix::mass_based(HashMap::from([
    ///     (Pure::Water.into(), Ratio::new::<percent>(60.0)),
    ///     (Pure::Ethanol.into(), Ratio::new::<percent>(40.0)),
    /// ]))
    /// .is_ok());
    ///
    /// assert!(CustomMix::mass_based(HashMap::from([
    ///     (Refrigerant::R32.into(), Ratio::new::<percent>(50.0)),
    ///     (Refrigerant::R125.into(), Ratio::new::<percent>(50.0)),
    /// ]))
    /// .is_ok());
    /// ```
    pub fn mass_based(
        components: HashMap<CustomMixComponent, Ratio>,
    ) -> Result<Self, CustomMixError> {
        Self::validate(&components)?;
        Ok(Self::MassBased(components))
    }

    /// Clone and convert to [`CustomMix::MoleBased`]
    /// _(mass fractions will be converted to mole fractions)_.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance::{CustomMix, Pure, Refrigerant};
    /// use rfluids::uom::si::f64::Ratio;
    /// use rfluids::uom::si::ratio::percent;
    /// use std::collections::HashMap;
    ///
    /// let mole_based_mix = CustomMix::mole_based(HashMap::from([
    ///     (Pure::Water.into(), Ratio::new::<percent>(80.0)),
    ///     (Pure::Ethanol.into(), Ratio::new::<percent>(20.0)),
    /// ]))
    /// .unwrap();
    /// assert_eq!(mole_based_mix.to_mole_based(), mole_based_mix);
    ///
    /// let mass_based_mix = CustomMix::mass_based(HashMap::from([
    ///     (Refrigerant::R32.into(), Ratio::new::<percent>(50.0)),
    ///     (Refrigerant::R125.into(), Ratio::new::<percent>(50.0)),
    /// ]))
    /// .unwrap();
    /// assert_ne!(mass_based_mix.to_mole_based(), mass_based_mix);
    /// ```
    pub fn to_mole_based(&self) -> Self {
        match self {
            CustomMix::MassBased(c) => {
                let mut components = c.clone().into_iter().collect::<Vec<_>>();
                let mut sum = 0.0;
                for component in &mut components {
                    component.1 /= Self::molar_mass(&component.0);
                    sum += component.1.value;
                }
                for component in &mut components {
                    component.1 /= sum;
                }
                Self::MoleBased(HashMap::from_iter(components))
            }
            _ => self.clone(),
        }
    }

    /// Specified components and their fractions.
    pub fn components(&self) -> &HashMap<CustomMixComponent, Ratio> {
        match self {
            CustomMix::MoleBased(components) => components,
            CustomMix::MassBased(components) => components,
        }
    }

    fn validate(components: &HashMap<CustomMixComponent, Ratio>) -> Result<(), CustomMixError> {
        if components.len() < 2 {
            return Err(CustomMixError::NotEnoughComponents);
        }
        if components.keys().any(|c| {
            matches!(
                c, CustomMixComponent::Refrigerant(r) if r.category() != RefrigerantCategory::Pure
            )
        }) {
            return Err(CustomMixError::InvalidComponent);
        }
        if components
            .values()
            .any(|f| f <= &Ratio::ZERO || f >= &Ratio::new::<ratio>(1.0))
        {
            return Err(CustomMixError::InvalidFraction);
        }
        if (components.values().map(|f| f.value).sum::<f64>() - 1.0).abs() > 1e-6 {
            return Err(CustomMixError::InvalidFractionsSum);
        }
        Ok(())
    }

    fn molar_mass(component: &CustomMixComponent) -> f64 {
        AbstractState::new(component.backend_name(), component.as_ref())
            .unwrap()
            .keyed_output(MolarMass)
            .unwrap()
    }
}

impl BackendName for CustomMix {
    fn backend_name(&self) -> &'static str {
        "HEOS"
    }
}

/// Custom mixture component.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CustomMixComponent {
    /// Pure substance.
    Pure(Pure),

    /// Pure refrigerant.
    Refrigerant(Refrigerant),
}

impl BackendName for CustomMixComponent {
    fn backend_name(&self) -> &'static str {
        match self {
            CustomMixComponent::Pure(pure) => pure.backend_name(),
            CustomMixComponent::Refrigerant(refrigerant) => refrigerant.backend_name(),
        }
    }
}

impl AsRef<str> for CustomMixComponent {
    fn as_ref(&self) -> &str {
        match self {
            CustomMixComponent::Pure(pure) => pure.as_ref(),
            CustomMixComponent::Refrigerant(refrigerant) => refrigerant.as_ref(),
        }
    }
}

impl From<Pure> for CustomMixComponent {
    fn from(value: Pure) -> Self {
        Self::Pure(value)
    }
}

impl From<Refrigerant> for CustomMixComponent {
    fn from(value: Refrigerant) -> Self {
        Self::Refrigerant(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod custom_mix {
        use super::*;
        use crate::uom::si::ratio::percent;
        use approx::relative_eq;
        use rstest::*;

        #[rstest]
        #[case(HashMap::from([(Pure::Water.into(), 60.0), (Pure::Ethanol.into(), 40.0)]))]
        #[case(HashMap::from([(Refrigerant::R32.into(), 50.0), (Refrigerant::R125.into(), 50.0)]))]
        fn mole_or_mass_based_from_valid_input_returns_ok(
            #[case] components: HashMap<CustomMixComponent, f64>,
        ) {
            assert!(CustomMix::mole_based(HashMap::from_iter(
                components
                    .clone()
                    .into_iter()
                    .map(|c| (c.0, Ratio::new::<percent>(c.1)))
            ))
            .is_ok());
            assert!(CustomMix::mass_based(HashMap::from_iter(
                components
                    .into_iter()
                    .map(|c| (c.0, Ratio::new::<percent>(c.1)))
            ))
            .is_ok());
        }

        #[rstest]
        #[case(HashMap::from([(Pure::Water.into(), 60.0)]), CustomMixError::NotEnoughComponents)]
        #[case(
            HashMap::from([(Pure::Water.into(), 50.0), (Pure::Water.into(), 50.0)]),
            CustomMixError::NotEnoughComponents
        )]
        #[case(
            HashMap::from([(Refrigerant::R32.into(), 50.0), (Refrigerant::R407C.into(), 50.0)]),
            CustomMixError::InvalidComponent
        )]
        #[case(
            HashMap::from([(Refrigerant::R32.into(), -50.0), (Refrigerant::R125.into(), 50.0)]),
            CustomMixError::InvalidFraction
        )]
        #[case(
            HashMap::from([(Refrigerant::R32.into(), 150.0), (Refrigerant::R125.into(), 50.0)]),
            CustomMixError::InvalidFraction
        )]
        #[case(
            HashMap::from([(Refrigerant::R32.into(), 40.0), (Refrigerant::R125.into(), 40.0)]),
            CustomMixError::InvalidFractionsSum
        )]
        fn mole_or_mass_based_from_invalid_input_returns_err(
            #[case] components: HashMap<CustomMixComponent, f64>,
            #[case] expected: CustomMixError,
        ) {
            assert_eq!(
                CustomMix::mole_based(HashMap::from_iter(
                    components
                        .clone()
                        .into_iter()
                        .map(|c| (c.0, Ratio::new::<percent>(c.1)))
                ))
                .unwrap_err(),
                expected
            );
            assert_eq!(
                CustomMix::mass_based(HashMap::from_iter(
                    components
                        .into_iter()
                        .map(|c| (c.0, Ratio::new::<percent>(c.1)))
                ))
                .unwrap_err(),
                expected
            );
        }

        #[test]
        fn to_mole_based_from_mole_based_returns_same() {
            let sut = CustomMix::mole_based(HashMap::from([
                (Pure::Water.into(), Ratio::new::<percent>(80.0)),
                (Pure::Ethanol.into(), Ratio::new::<percent>(20.0)),
            ]))
            .unwrap();
            let result = sut.to_mole_based();
            assert_eq!(result, sut);
            assert!(matches(result, [("Water", 0.8), ("Ethanol", 0.2)]));
        }

        #[test]
        fn to_mole_based_from_mass_based_returns_other_with_converted_fractions() {
            let sut = CustomMix::mass_based(HashMap::from([
                (Refrigerant::R32.into(), Ratio::new::<percent>(50.0)),
                (Refrigerant::R125.into(), Ratio::new::<percent>(50.0)),
            ]))
            .unwrap();
            let result = sut.to_mole_based();
            assert_ne!(result, sut);
            assert!(matches(sut, [("R32", 0.5), ("R125", 0.5)]));
            assert!(matches(
                result,
                [("R32", 0.6976146993758624), ("R125", 0.30238530062413754)]
            ));
        }

        #[test]
        fn backend_name_returns_heos() {
            let sut = CustomMix::mass_based(HashMap::from([
                (Pure::Water.into(), Ratio::new::<percent>(60.0)),
                (Pure::Ethanol.into(), Ratio::new::<percent>(40.0)),
            ]))
            .unwrap();
            assert_eq!(sut.backend_name(), "HEOS");
        }

        fn matches(mix: CustomMix, expected: [(&str, f64); 2]) -> bool {
            mix.components().len() == expected.len()
                && mix
                    .components()
                    .iter()
                    .filter(|component| {
                        expected.iter().any(|exp| {
                            component.0.as_ref() == exp.0 && relative_eq!(component.1.value, exp.1)
                        })
                    })
                    .count()
                    == expected.len()
        }
    }

    mod custom_mix_component {
        use super::*;

        #[test]
        pub fn custom_mix_component_is_transparent() {
            assert_eq!(
                CustomMixComponent::from(Pure::Water).backend_name(),
                Pure::Water.backend_name()
            );
            assert_eq!(
                CustomMixComponent::from(Pure::Water).as_ref(),
                Pure::Water.as_ref()
            );
            assert_eq!(
                CustomMixComponent::from(Refrigerant::R32).backend_name(),
                Refrigerant::R32.backend_name()
            );
            assert_eq!(
                CustomMixComponent::from(Refrigerant::R32).as_ref(),
                Refrigerant::R32.as_ref()
            );
        }
    }
}
