use core::*;
use simulator::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DoubleAnalysis<T> {
    first: T,
    second: T,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DoubleAnalysisOutput<T>
where
    T: Analysis,
{
    first: T::Output,
    second: T::Output,
}

impl<T> Analysis for DoubleAnalysis<T>
where
    T: Analysis,
{
    type Output = DoubleAnalysisOutput<T>;
}

impl<T> SupportedBy<Spectre> for DoubleAnalysis<T>
where
    T: SupportedBy<Spectre>,
{
    fn into_input(self, inputs: &mut Vec<<Spectre as Simulator>::Input>) {
        self.first.into_input(inputs);
        self.second.into_input(inputs);
    }

    fn from_output(
        outputs: &mut impl Iterator<Item = <Spectre as Simulator>::Output>,
    ) -> Self::Output {
        let first = T::from_output(outputs);
        let second = T::from_output(outputs);
        Self::Output { first, second }
    }
}

impl<T> ConvertAnalysis for DoubleAnalysis<T>
where
    T: Analysis,
{
    type Inner = (T, T);
    fn into(self) -> Self::Inner {
        (self.first, self.second)
    }
    fn from(output: <Self::Inner as Analysis>::Output) -> Self::Output {
        Self::Output {
            first: output.0,
            second: output.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_analysis_spectre() {
        let a = DoubleAnalysis {
            first: Tran,
            second: Tran,
        };
        let spectre = Spectre;
        let output = spectre.simulate(a);
        assert_eq!(
            output,
            DoubleAnalysisOutput {
                first: TranOutput,
                second: TranOutput,
            }
        );
    }

    #[test]
    fn double_analysis_ngspice() {
        let a = DoubleAnalysis {
            first: Tran,
            second: Tran,
        };
        let ngspice = Ngspice;
        let output = ngspice.simulate_convert(a);
        assert_eq!(
            output,
            DoubleAnalysisOutput {
                first: TranOutput,
                second: TranOutput,
            }
        );
    }
}
