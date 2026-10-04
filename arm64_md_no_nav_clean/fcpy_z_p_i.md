## FCPY

Copy floating-point immediate to vector elements (predicated) This instruction copies a floating-point immediate into each Active element in the destination vector. Inactive elements in the destination vector register remain unmodified. This instruction is used by the alias [FMOV (immediate, predicated)](fmov_fcpy_z_p_i.html "Move floating-point immediate to vector elements (predicated)").

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let d : integer = UInt(Zd); let imm : bits(esize) = VFPExpandImm{}(imm8);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); var result : bits(VL) = Z{}(d); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then result[e\*:esize] = imm; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
