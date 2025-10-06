use aga8::{composition::Composition, gerg2008};


pub struct Units {
    pub pressure: Pressure,
    pub temp: Temperature,
    pub density: Density,
    pub energy: Energy,
    pub entropy: Entropy,
    pub speed: Speed,
    pub jt_coeff: JT_Coeff,
    pub flow: Flow,
}
impl Default for Units {
    fn default() -> Self {
        Units {
            pressure: Pressure::PSI,
            temp: Temperature::F,
            density: Density::lbm_ft3,
            energy: Energy::BTU_lbm,
            entropy: Entropy::BTU_lbm_R,
            speed: Speed::ft_s,
            jt_coeff: JT_Coeff::R_PSI,
            flow: Flow::scfm,
        }
    }
}


pub trait PrintUnit {
    fn print_unit(&self) -> &'static str;
}

#[derive(Clone, Copy)]
pub enum Pressure {
    kPa,
    PSI,
    Bar,
}
impl PrintUnit for Pressure {
    fn print_unit(&self) -> &'static str{
        match self {
            Pressure::kPa => "kPa",
            Pressure::Bar => "Bar",
            Pressure::PSI => "PSI",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Temperature {
    C,
    K,
    F,
    R,
}
impl PrintUnit for Temperature {
    fn print_unit(&self) -> &'static str{
        match self {
            Temperature::C => "C",
            Temperature::K => "K",
            Temperature::F => "F",
            Temperature::R => "R",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Density {
    mol_l,
    kg_m3,
    lbm_ft3,
}
impl PrintUnit for Density {
    fn print_unit(&self) -> &'static str{
        match self {
           Density::mol_l => "mol/l",
           Density::kg_m3 => "kg/m^3",
           Density::lbm_ft3 => "lbm/ft^3",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Energy {
    J_mol,
    kJ_kg,
    BTU_lbm,
}
impl PrintUnit for Energy {
    fn print_unit(&self) -> &'static str{
        match self {
           Energy::J_mol => "J/mol",
           Energy::kJ_kg => "kJ/kg",
           Energy::BTU_lbm => "BTU/lbm",
        }
    }
}


#[derive(Clone, Copy)]
pub enum Entropy {
    J_mol_K,
    kJ_kg_K,
    BTU_lbm_R,
}
impl PrintUnit for Entropy {
    fn print_unit(&self) -> &'static str{
        match self {
           Entropy::J_mol_K => "J/(mol-K)",
           Entropy::kJ_kg_K => "kJ/(kg-K)",
           Entropy::BTU_lbm_R => "BTU/(lbm-R)",
        }
    }
}


#[derive(Clone, Copy)]
pub enum Speed {
    m_s,
    ft_s,
}
impl PrintUnit for Speed {
    fn print_unit(&self) -> &'static str{
        match self {
           Speed::m_s => "m/s",
           Speed::ft_s => "ft/s"
        }
    }
}

#[derive(Clone, Copy)]
pub enum JT_Coeff {
    K_kPa,
    K_bar,
    R_PSI,
}
impl PrintUnit for JT_Coeff {
    fn print_unit(&self) -> &'static str{
        match self {
           JT_Coeff::K_kPa => "K/kPa",
           JT_Coeff::K_bar => "K/Bar",
           JT_Coeff::R_PSI => "R/PSI",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Flow {
    kg_s,
    kg_m,
    kg_h,
    lbm_s,
    lbm_m,
    lbm_h,
    Nm3_h,
    scfm,
    scfh,
}
impl PrintUnit for Flow {
    fn print_unit(&self) -> &'static str{
        match self {
           Flow::kg_s => "kg/s",
           Flow::kg_m => "kg/min",
           Flow::kg_h => "kg/hr",
           Flow::lbm_s => "lbm/s",
           Flow::lbm_m => "lbm/min",
           Flow::lbm_h => "lbm/hr",
           Flow::Nm3_h => "Nm^3/hr",
           Flow::scfm => "scfm",
           Flow::scfh => "scfh",
        }
    }
}

pub fn get_pressure(pressure: f64, unit: Pressure) -> f64 {
    match unit {
        Pressure::kPa => pressure,
        Pressure::Bar => pressure * 0.01,
        Pressure::PSI => pressure * 0.145038,
    }
}

pub fn set_pressure(pressure: f64, unit: Pressure) -> f64 {
    match unit {
        Pressure::kPa => pressure,
        Pressure::Bar => pressure / 0.01,
        Pressure::PSI => pressure / 0.145038,
    }
}

pub fn get_temperature(temperature: f64, unit: Temperature) -> f64 {
    match unit {
        Temperature::K => temperature,
        Temperature::C => temperature - 273.15,
        Temperature::F => (temperature - 273.15) * 9.0 / 5.0 + 32.0,
        Temperature::R => temperature * 9.0 / 5.0,
    }
}

pub fn set_temperature(temperature: f64, unit: Temperature) -> f64 {
    match unit {
        Temperature::K => temperature,
        Temperature::C => temperature + 273.15,
        Temperature::F => (temperature - 32.0) * 5.0 / 9.0 + 273.15,
        Temperature::R => temperature * 5.0 / 9.0,
    }
}

pub fn get_density(density: f64, unit: Density, molar_mass: f64) -> f64 {
    match unit {
        Density::mol_l => density,
        Density::kg_m3 => density * molar_mass,
        Density::lbm_ft3 => density * molar_mass * 2.20462 / 35.3147,
    }
}

pub fn get_energy(energy: f64, unit: Energy, molar_mass: f64) -> f64 {
    match unit {
        Energy::J_mol => energy,
        Energy::kJ_kg => energy / molar_mass,
        Energy::BTU_lbm => energy / molar_mass * 0.429923,
    }
}

pub fn set_energy(energy: f64, unit: Energy, molar_mass: f64) -> f64 {
    match unit {
        Energy::J_mol => energy,
        Energy::kJ_kg => energy * molar_mass,
        Energy::BTU_lbm => energy * molar_mass / 0.429923,
    }
}

pub fn get_entropy(entropy: f64, unit: Entropy, molar_mass: f64) -> f64 {
    match unit {
        Entropy::J_mol_K => entropy,
        Entropy::kJ_kg_K => entropy / molar_mass,
        Entropy::BTU_lbm_R => entropy / molar_mass * 0.429923 * 5.0 / 9.0,
    }
}

pub fn set_entropy(entropy: f64, unit: Entropy, molar_mass: f64) -> f64 {
    match unit {
        Entropy::J_mol_K => entropy,
        Entropy::kJ_kg_K => entropy * molar_mass,
        Entropy::BTU_lbm_R => entropy * molar_mass / 0.429923 / 5.0 * 9.0,
    }
}

pub fn get_speed(speed:f64, unit: Speed) -> f64 {
    match unit {
        Speed::m_s => speed,
        Speed::ft_s => speed * 3.28084,
    }
}

pub fn set_speed(speed:f64, unit: Speed) -> f64 {
    match unit {
        Speed::m_s => speed,
        Speed::ft_s => speed / 3.28084,
    }
}

pub fn get_gibbs_energy(g: f64, p: Pressure, t: Temperature) -> f64 {
    let mut val = g;
    match p {
        Pressure::kPa => (),
        Pressure::Bar => val = val / 0.01,
        Pressure::PSI => val = val / 0.145038,
    }
    match t {
        Temperature::K => (),
        Temperature::C => (),
        Temperature::R => val = val * 9.0 / 5.0,
        Temperature::F => val = val * 9.0 / 5.0,
    }
    val
}

pub fn get_jt_coeff(jt_coeff:f64, unit: JT_Coeff) -> f64 {
    match unit {
        JT_Coeff::K_kPa => jt_coeff,
        JT_Coeff::K_bar => jt_coeff * 0.01,
        JT_Coeff::R_PSI => jt_coeff * 9.0 / 5.0 / 0.145038,
    }
}

pub fn set_jt_coeff(jt_coeff:f64, unit: JT_Coeff) -> f64 {
    match unit {
        JT_Coeff::K_kPa => jt_coeff,
        JT_Coeff::K_bar => jt_coeff / 0.01,
        JT_Coeff::R_PSI => jt_coeff / 9.0 * 5.0 * 0.145038,
    }
}

pub fn get_flow(flow_kg_s: f64, unit: Flow, gas_comp: &Composition, stp_60: bool, use_gerg2008: bool) -> f64 {
    match unit {
        Flow::kg_s => flow_kg_s,
        Flow::kg_m => flow_kg_s * 60.0,
        Flow::kg_h => flow_kg_s * 3600.0,
        Flow::lbm_s => flow_kg_s * 2.20462,
        Flow::lbm_m => flow_kg_s * 2.20462 * 60.0,
        Flow::lbm_h => flow_kg_s * 2.20462 * 3600.0,
        Flow::Nm3_h => {
            let kg_h = flow_kg_s * 3600.0;
            // Calculate density at STP
            let density_kg_m3;
            if use_gerg2008 {
                let mut gas_state = gerg2008::Gerg2008::new();
                gas_state.set_composition(gas_comp);
                gas_state.p = 101.325;
                gas_state.t = 273.15;
                gas_state.density(0);
                gas_state.properties();
                density_kg_m3 = gas_state.mm * gas_state.d;
            } else {
                let mut gas_state = aga8::detail::Detail::new();
                gas_state.set_composition(gas_comp);
                gas_state.p = 101.325;
                gas_state.t = 273.15;
                gas_state.density();
                gas_state.properties();
                density_kg_m3 = gas_state.mm * gas_state.d;
            }
            kg_h / density_kg_m3
        },
        Flow::scfm => {
            if stp_60 {
                let kg_m = flow_kg_s * 60.0;
                // Calculate density at STP
                let density_kg_m3;
                if use_gerg2008 {
                    let mut gas_state = gerg2008::Gerg2008::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 288.706;
                    gas_state.density(0);
                    gas_state.properties();
                    density_kg_m3 = gas_state.mm * gas_state.d;
                } else {
                    let mut gas_state = aga8::detail::Detail::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 288.706;
                    gas_state.density();
                    gas_state.properties();
                    density_kg_m3 = gas_state.mm * gas_state.d;
                }
                let m3_min = kg_m / density_kg_m3;
                m3_min * 35.3147
            } else {
                let kg_m = flow_kg_s * 60.0;
                // Calculate density at STP
                let density_kg_m3;
                if use_gerg2008 {
                    let mut gas_state = gerg2008::Gerg2008::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 294.261;
                    gas_state.density(0);
                    gas_state.properties();
                    density_kg_m3 = gas_state.mm * gas_state.d;
                } else {
                    let mut gas_state = aga8::detail::Detail::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 294.261;
                    gas_state.density();
                    gas_state.properties();
                    density_kg_m3 = gas_state.mm * gas_state.d;
                }
                let m3_min = kg_m / density_kg_m3;
                m3_min * 35.3147
            }
        },
        Flow::scfh => {
            if stp_60 {
                let kg_hr = flow_kg_s * 3600.0;
                // Calculate density at STP
                let density_kg_m3;
                if use_gerg2008 {
                    let mut gas_state = gerg2008::Gerg2008::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 288.706;
                    gas_state.density(0);
                    gas_state.properties();
                    density_kg_m3 = gas_state.mm * gas_state.d;
                } else {
                    let mut gas_state = aga8::detail::Detail::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 288.706;
                    gas_state.density();
                    gas_state.properties();
                    density_kg_m3 = gas_state.mm * gas_state.d;
                }
                let m3_hr = kg_hr / density_kg_m3;
                m3_hr * 35.3147
            } else {
                let kg_hr = flow_kg_s * 3600.0;
                // Calculate density at STP
                let density_kg_m3;
                if use_gerg2008 {
                    let mut gas_state = gerg2008::Gerg2008::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 294.261;
                    gas_state.density(0);
                    gas_state.properties();
                    density_kg_m3 = gas_state.mm * gas_state.d;
                } else {
                    let mut gas_state = aga8::detail::Detail::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 294.261;
                    gas_state.density();
                    gas_state.properties();
                    density_kg_m3 = gas_state.mm * gas_state.d;
                }
                let m3_hr = kg_hr / density_kg_m3;
                m3_hr * 35.3147
            }
        },
    }
}

pub fn set_flow(flow: f64, unit: Flow, gas_comp: &Composition, stp_60: bool, use_gerg2008: bool) -> f64 {
    match unit {
        Flow::kg_s => flow,
        Flow::kg_m => flow / 60.0,
        Flow::kg_h => flow / 3600.0,
        Flow::lbm_s => flow / 2.20462,
        Flow::lbm_m => flow / 2.20462 / 60.0,
        Flow::lbm_h => flow / 2.20462 / 3600.0,
        Flow::Nm3_h => {
            let Nm3_s = flow / 60.0;
            let kg_s;
            // Calculate density at STP
            if use_gerg2008 {
                let mut gas_state = gerg2008::Gerg2008::new();
                gas_state.set_composition(gas_comp);
                gas_state.p = 101.325;
                gas_state.t = 273.15;
                gas_state.density(0);
                gas_state.properties();
                let density_kg_m3 = gas_state.mm * gas_state.d;
                kg_s = density_kg_m3 * Nm3_s;

            } else {
                let mut gas_state = aga8::detail::Detail::new();
                gas_state.set_composition(gas_comp);
                gas_state.p = 101.325;
                gas_state.t = 273.15;
                gas_state.density();
                gas_state.properties();
                let density_kg_m3 = gas_state.mm * gas_state.d;
                kg_s = density_kg_m3 * Nm3_s;
            }
            kg_s
        },
        Flow::scfm => {
            if stp_60 {
                let scfs = flow / 60.0;
                // Calculate density at STP
                let kg_s;
                if use_gerg2008 {
                    let mut gas_state = gerg2008::Gerg2008::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 288.706;
                    gas_state.density(0);
                    gas_state.properties();
                    let density_kg_m3 = gas_state.mm * gas_state.d;
                    let density_kg_ft3 = density_kg_m3 / density_kg_m3;
                    kg_s = density_kg_ft3 * scfs;
                } else {
                    let mut gas_state = aga8::detail::Detail::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 288.706;
                    gas_state.density();
                    gas_state.properties();
                    let density_kg_m3 = gas_state.mm * gas_state.d;
                    let density_kg_ft3 = density_kg_m3 / density_kg_m3;
                    kg_s = density_kg_ft3 * scfs;
                }
                kg_s
            } else {
                let scfs = flow / 60.0;
                // Calculate density at STP
                let kg_s;
                if use_gerg2008 {
                    let mut gas_state = gerg2008::Gerg2008::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 294.261;
                    gas_state.density(0);
                    gas_state.properties();
                    let density_kg_m3 = gas_state.mm * gas_state.d;
                    let density_kg_ft3 = density_kg_m3 / density_kg_m3;
                    kg_s = density_kg_ft3 * scfs;
                } else {
                    let mut gas_state = aga8::detail::Detail::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 294.261;
                    gas_state.density();
                    gas_state.properties();
                    let density_kg_m3 = gas_state.mm * gas_state.d;
                    let density_kg_ft3 = density_kg_m3 / density_kg_m3;
                    kg_s = density_kg_ft3 * scfs;
                }
                kg_s
            }
        },
        Flow::scfh => {
            if stp_60 {
                let scfs = flow / 3600.0;
                // Calculate density at STP
                let kg_s;
                if use_gerg2008 {
                    let mut gas_state = gerg2008::Gerg2008::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 288.706;
                    gas_state.density(0);
                    gas_state.properties();
                    let density_kg_m3 = gas_state.mm * gas_state.d;
                    let density_kg_ft3 = density_kg_m3 / density_kg_m3;
                    kg_s = density_kg_ft3 * scfs;
                } else {
                    let mut gas_state = aga8::detail::Detail::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 288.706;
                    gas_state.density();
                    gas_state.properties();
                    let density_kg_m3 = gas_state.mm * gas_state.d;
                    let density_kg_ft3 = density_kg_m3 / density_kg_m3;
                    kg_s = density_kg_ft3 * scfs;
                }
                kg_s
            } else {
                let scfs = flow / 60.0;
                // Calculate density at STP
                let kg_s;
                if use_gerg2008 {
                    let mut gas_state = gerg2008::Gerg2008::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 294.261;
                    gas_state.density(0);
                    gas_state.properties();
                    let density_kg_m3 = gas_state.mm * gas_state.d;
                    let density_kg_ft3 = density_kg_m3 / density_kg_m3;
                    kg_s = density_kg_ft3 * scfs;
                } else {
                    let mut gas_state = aga8::detail::Detail::new();
                    gas_state.set_composition(gas_comp);
                    gas_state.p = 101.325;
                    gas_state.t = 294.261;
                    gas_state.density();
                    gas_state.properties();
                    let density_kg_m3 = gas_state.mm * gas_state.d;
                    let density_kg_ft3 = density_kg_m3 / density_kg_m3;
                    kg_s = density_kg_ft3 * scfs;
                }
                kg_s
            }
        },
    }
}