use crate::io::{FluidParam, KeyedInput};
use crate::uom::si::f64::{
    AvailableEnergy, MassDensity, MolarConcentration, MolarEnergy, MolarHeatCapacity, Pressure,
    Ratio, SpecificHeatCapacity, ThermodynamicTemperature,
};

/// Fluid keyed input.
///
/// # Examples
///
/// ```
/// use rfluids::io::FluidInput;
/// use rfluids::uom::si::f64::{MassDensity, Pressure, ThermodynamicTemperature};
/// use rfluids::uom::si::mass_density::gram_per_cubic_centimeter;
/// use rfluids::uom::si::pressure::atmosphere;
/// use rfluids::uom::si::thermodynamic_temperature::degree_celsius;
///
/// let pressure =
///     FluidInput::pressure(Pressure::new::<atmosphere>(1.0));
/// let temperature =
///     FluidInput::temperature(ThermodynamicTemperature::new::<degree_celsius>(20.0));
/// let density =
///     FluidInput::density(MassDensity::new::<gram_per_cubic_centimeter>(1.0));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FluidInput(FluidParam, f64);

impl FluidInput {
    /// Mass density _(key: [`DMass`](FluidParam::DMass), SI units: kg/m³)_.
    pub fn density(value: MassDensity) -> Self {
        Self(FluidParam::DMass, value.value)
    }

    /// Mass specific enthalpy _(key: [`HMass`](FluidParam::HMass), SI units: J/kg)_.
    pub fn enthalpy(value: AvailableEnergy) -> Self {
        Self(FluidParam::HMass, value.value)
    }

    /// Mass specific entropy _(key: [`SMass`](FluidParam::SMass), SI units: J/kg/K)_.
    pub fn entropy(value: SpecificHeatCapacity) -> Self {
        Self(FluidParam::SMass, value.value)
    }

    /// Mass specific internal energy _(key: [`UMass`](FluidParam::UMass), SI units: J/kg)_.
    pub fn internal_energy(value: AvailableEnergy) -> Self {
        Self(FluidParam::UMass, value.value)
    }

    /// Molar density _(key: [`DMolar`](FluidParam::DMolar), SI units: mol/m³)_.
    pub fn molar_density(value: MolarConcentration) -> Self {
        Self(FluidParam::DMolar, value.value)
    }

    /// Molar specific enthalpy _(key: [`HMolar`](FluidParam::HMolar), SI units: J/mol)_.
    pub fn molar_enthalpy(value: MolarEnergy) -> Self {
        Self(FluidParam::HMolar, value.value)
    }

    /// Molar specific entropy _(key: [`SMolar`](FluidParam::SMolar), SI units: J/mol/K)_.
    pub fn molar_entropy(value: MolarHeatCapacity) -> Self {
        Self(FluidParam::SMolar, value.value)
    }

    /// Molar specific internal energy _(key: [`UMolar`](FluidParam::UMolar), SI units: J/mol)_.
    pub fn molar_internal_energy(value: MolarEnergy) -> Self {
        Self(FluidParam::UMolar, value.value)
    }

    /// Pressure _(key: [`P`](FluidParam::P), SI units: Pa)_.
    pub fn pressure(value: Pressure) -> Self {
        Self(FluidParam::P, value.value)
    }

    /// Vapor quality _(key: [`Q`](FluidParam::Q), SI units: dimensionless, from 0 to 1)_.
    pub fn quality(value: Ratio) -> Self {
        Self(FluidParam::Q, value.value)
    }

    /// Temperature _(key: [`T`](FluidParam::T), SI units: K)_.
    pub fn temperature(value: ThermodynamicTemperature) -> Self {
        Self(FluidParam::T, value.value)
    }
}

impl KeyedInput<FluidParam> for FluidInput {
    fn key(&self) -> FluidParam {
        self.0
    }

    fn si_value(&self) -> f64 {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uom::si::available_energy::joule_per_kilogram;
    use crate::uom::si::mass_density::kilogram_per_cubic_meter;
    use crate::uom::si::molar_concentration::mole_per_cubic_meter;
    use crate::uom::si::molar_energy::joule_per_mole;
    use crate::uom::si::molar_heat_capacity::joule_per_kelvin_mole;
    use crate::uom::si::pressure::pascal;
    use crate::uom::si::ratio::ratio;
    use crate::uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use crate::uom::si::thermodynamic_temperature::kelvin;

    #[test]
    fn density_always_returns_expected_key_and_si_value() {
        let sut = FluidInput::density(MassDensity::new::<kilogram_per_cubic_meter>(1.0));
        assert_eq!(sut.key(), FluidParam::DMass);
        assert_eq!(sut.si_value(), 1.0);
    }

    #[test]
    fn enthalpy_always_returns_expected_key_and_si_value() {
        let sut = FluidInput::enthalpy(AvailableEnergy::new::<joule_per_kilogram>(1.0));
        assert_eq!(sut.key(), FluidParam::HMass);
        assert_eq!(sut.si_value(), 1.0);
    }

    #[test]
    fn entropy_always_returns_expected_key_and_si_value() {
        let sut = FluidInput::entropy(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(1.0));
        assert_eq!(sut.key(), FluidParam::SMass);
        assert_eq!(sut.si_value(), 1.0);
    }

    #[test]
    fn internal_energy_always_returns_expected_key_and_si_value() {
        let sut = FluidInput::internal_energy(AvailableEnergy::new::<joule_per_kilogram>(1.0));
        assert_eq!(sut.key(), FluidParam::UMass);
        assert_eq!(sut.si_value(), 1.0);
    }

    #[test]
    fn molar_density_always_returns_expected_key_and_si_value() {
        let sut = FluidInput::molar_density(MolarConcentration::new::<mole_per_cubic_meter>(1.0));
        assert_eq!(sut.key(), FluidParam::DMolar);
        assert_eq!(sut.si_value(), 1.0);
    }

    #[test]
    fn molar_enthalpy_always_returns_expected_key_and_si_value() {
        let sut = FluidInput::molar_enthalpy(MolarEnergy::new::<joule_per_mole>(1.0));
        assert_eq!(sut.key(), FluidParam::HMolar);
        assert_eq!(sut.si_value(), 1.0);
    }

    #[test]
    fn molar_entropy_always_returns_expected_key_and_si_value() {
        let sut = FluidInput::molar_entropy(MolarHeatCapacity::new::<joule_per_kelvin_mole>(1.0));
        assert_eq!(sut.key(), FluidParam::SMolar);
        assert_eq!(sut.si_value(), 1.0);
    }

    #[test]
    fn molar_internal_energy_always_returns_expected_key_and_si_value() {
        let sut = FluidInput::molar_internal_energy(MolarEnergy::new::<joule_per_mole>(1.0));
        assert_eq!(sut.key(), FluidParam::UMolar);
        assert_eq!(sut.si_value(), 1.0);
    }

    #[test]
    fn pressure_always_returns_expected_key_and_si_value() {
        let sut = FluidInput::pressure(Pressure::new::<pascal>(1.0));
        assert_eq!(sut.key(), FluidParam::P);
        assert_eq!(sut.si_value(), 1.0);
    }

    #[test]
    fn quality_returns_expected_key_and_si_value() {
        let sut = FluidInput::quality(Ratio::new::<ratio>(1.0));
        assert_eq!(sut.key(), FluidParam::Q);
        assert_eq!(sut.si_value(), 1.0);
    }

    #[test]
    fn temperature_returns_expected_key_and_si_value() {
        let sut = FluidInput::temperature(ThermodynamicTemperature::new::<kelvin>(1.0));
        assert_eq!(sut.key(), FluidParam::T);
        assert_eq!(sut.si_value(), 1.0);
    }
}
