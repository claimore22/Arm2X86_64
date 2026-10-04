## ADDP (scalar)

Add pair of elements (scalar) This instruction adds two vector elements in the source SIMD&FP register and writes the scalar result into the destination SIMD&FP register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 64; let datasize : integer{} = 128;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); [V](shared_pseudocode.html#accessor_V_2){esize}(d) = [IntReduce](shared_pseudocode.html#func_IntReduce_4){esize, datasize}([ReduceOp_ADD](shared_pseudocode.html#enum_ReduceOp_ADD), operand);
