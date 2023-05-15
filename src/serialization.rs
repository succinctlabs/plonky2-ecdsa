use alloc::vec::Vec;
use plonky2::field::types::Field;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use plonky2_u32::serialization::{ReadU32, WriteU32};

use crate::gadgets::biguint::BigUintTarget;
use crate::gadgets::nonnative::NonNativeTarget;

pub trait WriteBigUint {
    fn write_target_biguint(&mut self, x: BigUintTarget) -> IoResult<()>;
}

impl WriteBigUint for Vec<u8> {
    #[inline]
    fn write_target_biguint(&mut self, x: BigUintTarget) -> IoResult<()> {
        self.write_usize(x.num_limbs())?;
        for limb in x.limbs.iter() {
            self.write_target_u32(*limb)?;
        }

        Ok(())
    }
}

pub trait ReadBigUint {
    fn read_target_biguint(&mut self) -> IoResult<BigUintTarget>;
}

impl ReadBigUint for Buffer {
    #[inline]
    fn read_target_biguint(&mut self) -> IoResult<BigUintTarget> {
        let length = self.read_usize()?;
        let limbs = (0..length)
            .map(|_| self.read_target_u32())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(BigUintTarget{ limbs: limbs })
    }
}



pub trait WriteNonNativeTarget {
    fn write_target_nonnative<FF:Field>(&mut self, x: NonNativeTarget<FF>) -> IoResult<()>;
}

impl WriteNonNativeTarget for Vec<u8> {
    #[inline]
    fn write_target_nonnative<FF:Field>(&mut self, x: NonNativeTarget<FF>) -> IoResult<()> {
        self.write_target_biguint(x.value)
    }
}

pub trait ReadNonNativeTarget {
    fn read_target_nonnative<FF:Field>(&mut self) -> IoResult<NonNativeTarget<FF>>;
}

impl ReadNonNativeTarget for Buffer {
    #[inline]
    fn read_target_nonnative<FF:Field>(&mut self) -> IoResult<NonNativeTarget<FF>> {
        let value = self.read_target_biguint()?;
        Ok(NonNativeTarget{ value: value, _phantom: core::marker::PhantomData })
    }
}
