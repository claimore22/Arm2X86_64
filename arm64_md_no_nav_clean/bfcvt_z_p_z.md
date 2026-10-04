## BFCVT

Single-precision convert to BFloat16 (predicated) This instruction converts to BFloat16 from single-precision in each active floating-point element of the source vector, and places the results in the corresponding elements of the destination vector. Inactive elements in the destination vector register remain unmodified or are set to zero, depending on whether merging or zeroing predication is selected. Since the result type is smaller than the input type, the results are zero-extended to fill each destination element. ID_AA64ZFR0_EL1.BF16 indicates whether this instruction is implemented. It has encodings from 2 classes: [Merging](#iclass_merging) and [Zeroing](#iclass_zeroing)

#### Decode

if !IsFeatureImplemented(FEAT_SVE2p2) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let merging : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV 32; let mask : bits(PL) = P{}(g); let operand : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, 32) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var result : bits(VL) = if merging then [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) else Zeros{VL}; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, 32) then let element : bits(32) = operand[e\*:32]; result[(2\*e)\*:16] = [FPConvertBF](shared_pseudocode.html#func_FPConvertBF_2)(element, FPCR()); result[(2\*e+1)\*:16] = Zeros{16}; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
