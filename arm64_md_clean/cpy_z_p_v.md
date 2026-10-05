## CPY (SIMD&FP scalar)

Copy SIMD&FP scalar register to vector elements (predicated) This instruction copies the SIMD & floating-point scalar source register to each Active element in the destination vector. Inactive elements in the destination vector register remain unmodified. This instruction is used by the alias [MOV (SIMD&FP scalar, predicated)](mov_cpy_z_p_v.html "Move SIMD&FP scalar register to vector elements (predicated)").

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Vn); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(esize) = (if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [V](shared_pseudocode.html#accessor_V_2){esize}(n) else Zeros{esize}); var result : bits(VL) = Z{}(d); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then result[e\*:esize] = operand1; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
