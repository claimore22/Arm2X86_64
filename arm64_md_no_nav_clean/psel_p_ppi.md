## PSEL

Predicate select between predicate register or all-false This instruction places the contents of the first source predicate register into the destination predicate register if the indexed element of the second source predicate is true, otherwise it sets the destination predicate to all-false. The indexed element is determined by the sum of a general-purpose index register and an immediate, modulo the number of elements. This instruction does not set the condition flags. For programmer convenience, an assembler must also accept predicate-as-counter register names for the destination predicate register and the first source predicate register.

#### Decode

if !IsFeatureImplemented(FEAT_SME) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let imm5 : bits(5) = i1::tszh::tszl; var esize : integer; var imm : integer; case tszh::tszl of when '0000' => [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when '1000' => esize = 64; imm = UInt(imm5[4]); when 'x100' => esize = 32; imm = UInt(imm5[4:3]); when 'xx10' => esize = 16; imm = UInt(imm5[4:2]); when 'xxx1' => esize = 8; imm = UInt(imm5[4:1]); end; let n : integer = UInt(Pn); let m : integer = UInt(Pm); let d : integer = UInt(Pd); let v : integer = UInt('011'::Rv);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let operand1 : bits(PL) = P{}(n); let operand2 : bits(PL) = P{}(m); let idx : bits(32) = X{}(v); let element : integer = (UInt(idx) + imm) MOD elements; var result : bits(PL); if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand2, element, esize) then result = operand1; else result = Zeros{PL}; end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
