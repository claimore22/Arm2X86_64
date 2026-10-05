## CPY (scalar)

Copy general-purpose register to vector elements (predicated) This instruction copies the general-purpose scalar source register to each Active element in the destination vector. Inactive elements in the destination vector register remain unmodified. This instruction is used by the alias [MOV (scalar, predicated)](mov_cpy_z_p_r.html "Move general-purpose register to vector elements (predicated)").

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Rn); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); var result : bits(VL) = Z{}(d); if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then var operand1 : bits(64); if n == 31 then operand1 = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else operand1 = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then result[e\*:esize] = operand1[esize-1:0]; end; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
