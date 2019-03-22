use std::marker::PhantomData;

use bellman::{
    Circuit,
    ConstraintSystem,
    SynthesisError
};
use bellman::pairing::Engine;
use bellman::pairing::ff:: {
    Field,
    PrimeField,
};
use crate::helpers::engines::*;

pub(crate) struct DefaultCircuit<E: Engine> {
    pub(crate) a: Option<bool>,
    pub(crate) b: Option<bool>,
    pub(crate) _marker: PhantomData<E>
}

impl<E: Engine> Circuit<E> for DefaultCircuit<E>
    where E: bellman::pairing::Engine {
    fn synthesize<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        let a_var = cs.alloc(|| "a", || {
            if self.a.is_some() {
                if self.a.unwrap() {
                    Ok(E::Fr::one())
                } else {
                    Ok(E::Fr::zero())
                }
            } else {
                Err(SynthesisError::AssignmentMissing)
            }
        })?;

        cs.enforce(
            || "a_boolean_constraint",
            |lc| lc + CS::one() - a_var,
            |lc| lc + a_var,
            |lc| lc
        );

        let b_var = cs.alloc(|| "b", || {
            if self.b.is_some() {
                if self.b.unwrap() {
                    Ok(E::Fr::one())
                } else {
                    Ok(E::Fr::zero())
                }
            } else {
                Err(SynthesisError::AssignmentMissing)
            }
        })?;

        cs.enforce(
            || "b_boolean_constraint",
            |lc| lc + CS::one() - b_var,
            |lc| lc + b_var,
            |lc| lc
        );

        let c_var = cs.alloc_input(|| "c", || {
            if self.a.is_some() && self.b.is_some() {
                if self.a.unwrap() ^ self.b.unwrap() {
                    Ok(E::Fr::one())
                } else {
                    Ok(E::Fr::zero())
                }
            } else {
                Err(SynthesisError::AssignmentMissing)
            }
        })?;

        cs.enforce(
            || "c_xor_constraint",
            |lc| lc + a_var + a_var,
            |lc| lc + b_var,
            |lc| lc + a_var + b_var - c_var
        );

        Ok(())
    }
}

