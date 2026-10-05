## WHILERW

While free of read-after-write conflicts This instruction checks two addresses for a conflict or overlap between address ranges of the form [addr,addr+VL÷8), where VL is the accessible vector length in bits, that could result in a loop-carried dependency through memory due to the use of these addresses by contiguous load and store instructions within the same iteration of a loop. Generate a predicate whose elements are true while the addresses cannot conflict within the same iteration, and false thereafter. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Rn); let m : integer = UInt(Rm); let d : integer = UInt(Pd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = Ones{}; let src1 : bits(64) = X{}(n); let src2 : bits(64) = X{}(m); let operand1 : integer = UInt(src1); let operand2 : integer = UInt(src2); var result : bits(PL); let psize : integer{} = esize DIV 8; let diff : integer = Abs(operand2 - operand1) DIVRM (esize DIV 8); for e = 0 to elements-1 do if diff == 0 \|\| e < diff then result[e\*:psize] = ZeroExtend{psize}('1'); else result[e\*:psize] = ZeroExtend{psize}('0'); end; end; PSTATE.[N,Z,C,V] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(mask, result, esize); [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
