// circuits to implement:
/*def relu(X):
    a = torch.zeros_like(X)
    return torch.max(X, a) */

// Relu circuit: 
// inputs:
// 1. X: input[]
// outputs:
// 1. Y: output[]

// Potential problems:
// no negative numbers in a finite field

/*
The chip logic will have:
- A lookup table with all 0's for any arbitrary input length (let's say max 8 bits)
- A check to see if one element is greater than another
- A way to loop through the instance and return 0 if the instance is negative (large int) or return the number if the number
is positive 
 */

/*
Questions:
- How will we decide what a negative number is?
- How will we input negative numbers and convert them?
 */

struct ReluConfig<F> {
    input: Column<Instance>,
    output: Column<Instance>,
}

impl Circuit<F> for ReluConfig<F> {
    fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let input = meta.instance_column();
        let output = meta.instance_column();
        let zeros = meta.lookup();
        meta.lookup(|meta| {
            let input = meta.query_instance(input, Rotation::cur());
            let output = meta.query_instance(output, Rotation::cur());
            let zero = meta.query_lookup_table(zero, Rotation::cur());
        });
    }
}

struct ReluChip<F> {
    config: ReluConfig<F>,
}

impl Chip<F> for ReluChip<F> {

    fn load(&self, meta: &mut ConstraintSystem<F>, config: Self::Config) -> Self::Loaded {
        // load lookup table with zeroes
    }

    fn max_value(
        &self,
        ctx: &mut RegionCtx<'_, F>,
        num_limbs: usize,
    ) -> Result<AssignedInteger<F, Fresh>, Error> {
        // return max value in the array between input and zero table lookup.
    }

    fn construct(&self, meta: &mut ConstraintSystem<F>, config: ReluConfig, loaded: Self::Loaded) -> Self::Output {
        self.config = config;
    }

    fn synthesize(&self, meta: &mut ConstraintSystem<F>, config: Self::Config, loaded: Self::Loaded, output: Self::Output) {
        // synthesize the circuit
        // constrain columns?
    }
}