pub trait Supplier<T> {
    fn supply(&mut self) -> T;
}

impl<T, F> Supplier<T> for F
where
    F: FnMut() -> T,
{
    fn supply(&mut self) -> T {
        self()
    }
}

pub trait Mapper<I, O> {
    fn map(&mut self, input: I) -> O;
}

impl<I, O, F> Mapper<I, O> for F
where
    F: FnMut(I) -> O,
{
    fn map(&mut self, input: I) -> O {
        self(input)
    }
}

pub trait Consumer<T> {
    fn consume(&mut self, input: &T);
}

impl<T, F> Consumer<T> for F
where
    F: FnMut(&T),
{
    fn consume(&mut self, input: &T) {
        self(input)
    }
}

pub struct Pipeline<T> {
    supplier: Box<dyn Supplier<T>>,
}

impl<T: 'static> Pipeline<T> {
    pub fn supply<S>(supplier: S) -> Self
    where
        S: Supplier<T> + 'static,
    {
        Self {
            supplier: Box::new(supplier),
        }
    }

    pub fn map<M, O: 'static>(mut self, mut mapper: M) -> Pipeline<O>
    where
        M: Mapper<T, O> + 'static,
    {
        Pipeline {
            supplier: Box::new(move || mapper.map(self.supplier.supply())),
        }
    }

    pub fn consume<C>(mut self, mut consumer: C) -> Self
    where
        C: Consumer<T> + 'static,
    {
        Self {
            supplier: Box::new(move || {
                let input = self.supplier.supply();
                consumer.consume(&input);
                input
            }),
        }
    }

    pub fn execute(&mut self) -> T {
        self.supplier.supply()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invoke_all_pipeline_steps() {
        const EXPECTED_RESULT: i32 = 20;
        let mut pipeline = Pipeline::supply(|| 10)
            .map(|value| value + 10)
            .consume(|value: &_| assert_eq!(*value, EXPECTED_RESULT));

        let result = pipeline.execute();

        assert_eq!(result, EXPECTED_RESULT);
    }
}
