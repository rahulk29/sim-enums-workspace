use core::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Input {
    Tran(Tran),
    Ac(Ac),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Output {
    Tran(TranOutput),
    Ac(AcOutput),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Spectre;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ngspice;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tran;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ac;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TranOutput;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AcOutput;

impl Analysis for Tran {
    type Output = TranOutput;
}
impl Analysis for Ac {
    type Output = AcOutput;
}

impl Supports<Tran> for Spectre {
    fn into_input(a: Tran, inputs: &mut Vec<Self::Input>) {
        inputs.push(Input::Tran(a));
    }
    fn from_output(outputs: &mut impl Iterator<Item = Self::Output>) -> <Tran as Analysis>::Output {
        let output = outputs.next().unwrap();
        match output {
            Output::Tran(tran) => tran,
            _ => panic!("tran analysis output did not get back tran output"),
        }
    }
}
impl Supports<Ac> for Spectre {
    fn into_input(a: Ac, inputs: &mut Vec<Self::Input>) {
        inputs.push(Input::Ac(a));
    }
    fn from_output(outputs: &mut impl Iterator<Item = Self::Output>) -> <Ac as Analysis>::Output {
        let output = outputs.next().unwrap();
        match output {
            Output::Ac(ac) => ac,
            _ => panic!("ac analysis output did not get back ac output"),
        }
    }
}

impl Simulator for Spectre {
    type Input = Input;
    type Output = Output;

    fn raw_simulate(&self, input: Vec<Input>) -> Vec<Output> {
        let mut outputs = Vec::new();
        for input in input {
            match input {
                Input::Tran(_) => outputs.push(Output::Tran(TranOutput)),
                Input::Ac(_) => outputs.push(Output::Ac(AcOutput)),
            }
        }
        outputs
    }
}

impl Supports<Tran> for Ngspice {
    fn into_input(a: Tran, inputs: &mut Vec<Self::Input>) {
        inputs.push(Input::Tran(a));
    }
    fn from_output(outputs: &mut impl Iterator<Item = Self::Output>) -> <Tran as Analysis>::Output {
        let output = outputs.next().unwrap();
        match output {
            Output::Tran(tran) => tran,
            _ => panic!("tran analysis output did not get back tran output"),
        }
    }
}
impl Supports<Ac> for Ngspice {
    fn into_input(a: Ac, inputs: &mut Vec<Self::Input>) {
        inputs.push(Input::Ac(a));
    }
    fn from_output(outputs: &mut impl Iterator<Item = Self::Output>) -> <Ac as Analysis>::Output {
        let output = outputs.next().unwrap();
        match output {
            Output::Ac(ac) => ac,
            _ => panic!("ac analysis output did not get back ac output"),
        }
    }
}

impl Simulator for Ngspice {
    type Input = Input;
    type Output = Output;

    fn raw_simulate(&self, input: Vec<Input>) -> Vec<Output> {
        let mut outputs = Vec::new();
        for input in input {
            match input {
                Input::Tran(_) => outputs.push(Output::Tran(TranOutput)),
                Input::Ac(_) => outputs.push(Output::Ac(AcOutput)),
            }
        }
        outputs
    }
}
