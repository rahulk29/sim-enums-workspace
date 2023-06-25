pub trait Analysis {
    type Output;
}

pub trait Supports<A: Analysis>: Simulator {
    fn into_input(a: A, inputs: &mut Vec<Self::Input>);
    fn from_output(outputs: &mut impl Iterator<Item = Self::Output>) -> A::Output;
}

pub trait SupportedBy<S: Simulator>: Analysis {
    fn into_input(self, inputs: &mut Vec<S::Input>);
    fn from_output(outputs: &mut impl Iterator<Item = S::Output>) -> Self::Output;
}

impl<S, A> SupportedBy<S> for A
where
    A: Analysis,
    S: Supports<A>,
{
    fn into_input(self, inputs: &mut Vec<<S as Simulator>::Input>) {
        S::into_input(self, inputs);
    }
    fn from_output(outputs: &mut impl Iterator<Item = <S as Simulator>::Output>) -> Self::Output {
        S::from_output(outputs)
    }
}

pub trait Simulator {
    type Input;
    type Output;
    fn raw_simulate(&self, input: Vec<Self::Input>) -> Vec<Self::Output>;

    fn simulate<A>(&self, input: A) -> A::Output
    where
        A: Analysis + SupportedBy<Self>,
        Self: Sized,
    {
        let mut inputs = Vec::new();
        input.into_input(&mut inputs);
        let output = self.raw_simulate(inputs);
        let mut output = output.into_iter();
        A::from_output(&mut output)
    }

    fn simulate_convert<A>(&self, input: A) -> A::Output
    where
        A: ConvertAnalysis,
        A::Inner: SupportedBy<Self>,
        Self: Sized,
    {
        let input: A::Inner = input.into();
        let output: <A::Inner as Analysis>::Output = self.simulate(input);
        A::from(output)
    }
}

pub trait ConvertAnalysis: Analysis {
    type Inner: Analysis;
    fn into(self) -> Self::Inner;
    fn from(output: <Self::Inner as Analysis>::Output) -> Self::Output;
}

impl<T1, T2> Analysis for (T1, T2)
where
    T1: Analysis,
    T2: Analysis,
{
    type Output = (T1::Output, T2::Output);
}

impl<T1, T2, S> Supports<(T1, T2)> for S
where
    T1: Analysis + SupportedBy<S>,
    T2: Analysis + SupportedBy<S>,
    S: Simulator,
{
    fn into_input(a: (T1, T2), inputs: &mut Vec<S::Input>) {
        a.0.into_input(inputs);
        a.1.into_input(inputs);
    }

    fn from_output(
        outputs: &mut impl Iterator<Item = Self::Output>,
    ) -> <(T1, T2) as Analysis>::Output {
        let o0 = T1::from_output(outputs);
        let o1 = T2::from_output(outputs);
        (o0, o1)
    }
}
