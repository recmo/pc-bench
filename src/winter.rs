use rand::{thread_rng, Fill, Rng};
use rayon::prelude::*;
use std::{
    mem::size_of,
    time::{Duration, Instant},
};
use winterfell::{
    crypto::hashers,
    math::{
        fields::{f128::BaseElement, f62::BaseElement as F},
        FieldElement,
    },
    Air, AirContext, Assertion, ByteWriter, EvaluationFrame, FieldExtension, HashFunction, Matrix,
    ProofOptions, Prover, Serializable, TraceInfo, TraceTable, TransitionConstraintDegree,
};
use winter_prover::StarkDomain;

type Hasher = hashers::Blake3_256<F>;

pub fn rand_vec(size: usize) -> Vec<F> {
    let now = Instant::now();
    println!(
        "Memory allocation ({} GB)",
        (size * size_of::<F>()) as f64 / 1.0e9
    );
    let mut result = vec![F::default(); size];
    println!("Randomizing...");
    result.par_chunks_mut(1024).for_each_init(
        || thread_rng(),
        |rng, chunk| {
            for point in chunk {
                *point = F::new(rng.gen());
            }
        },
    );
    println!("Random generation took: {:?}", now.elapsed());
    result
}

struct MyAir(AirContext<F>);

impl Air for MyAir {
    type BaseField = F;
    type PublicInputs = ();

    fn new(trace_info: TraceInfo, _pub_inputs: Self::PublicInputs, options: ProofOptions) -> Self {
        assert_eq!(1, trace_info.width());
        let degrees = vec![TransitionConstraintDegree::new(1)];
        let num_assertions = 1;
        Self(AirContext::new(
            trace_info,
            degrees,
            num_assertions,
            options,
        ))
    }

    fn context(&self) -> &winterfell::AirContext<Self::BaseField> {
        &self.0
    }

    fn evaluate_transition<E: winterfell::math::FieldElement<BaseField = Self::BaseField>>(
        &self,
        frame: &winterfell::EvaluationFrame<E>,
        periodic_values: &[E],
        result: &mut [E],
    ) {
        unimplemented!()
    }

    fn get_assertions(&self) -> Vec<winterfell::Assertion<Self::BaseField>> {
        unimplemented!()
    }
}

struct MyProver(ProofOptions);

impl Prover for MyProver {
    type BaseField = F;
    type Air = MyAir;
    type Trace = TraceTable<F>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> () {
        todo!()
    }

    fn options(&self) -> &ProofOptions {
        &self.0
    }
}

fn bench(input: &[F]) -> f64 {
    let mut count = 0;
    let mut duration = 0.0;

    let my_prover = MyProver(ProofOptions::new(
        32, // number of queries
        2,  // blowup factor
        0,  // grinding factor
        HashFunction::Blake3_256,
        FieldExtension::None,
        8,   // FRI folding factor
        128, // FRI max remainder length
    ));
    let my_air = MyAir::new(TraceInfo::new(1, input.len()), (), my_prover.options().clone());
    let domain = StarkDomain::new(&my_air);

    let trace = Matrix::new(vec![input.to_vec()]);

    loop {
        count += 1;
        let now = Instant::now();

        let _ = my_prover.build_trace_commitment::<_, Hasher>(&trace, &domain);

        duration += now.elapsed().as_secs_f64();
        if duration > 5.0 {
            break;
        }
    }
    duration / count as f64
}

pub fn run(max_exponent: usize) {
    let max_size = 1 << max_exponent;
    println!("Preparing input...");
    let input = rand_vec(max_size);

    println!("size,duration,throughput");

    for i in 10..=max_exponent {
        let size = 1_usize << i;
        let duration = bench(&input[..size]);
        let throughput = size as f64 / duration;
        println!("{size},{duration},{throughput}");
    }
}
