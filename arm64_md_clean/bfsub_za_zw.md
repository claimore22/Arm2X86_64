## BFSUB

Multi-vector BFloat16 subtract from ZA array vectors This instruction destructively subtracts all elements of the two or four source vectors from the corresponding BFloat16 elements of the ZA single-vector groups. The single-vector group within each half of or each quarter of the ZA array is selected by the sum of the vector select register and offset, modulo half or quarter the number of ZA array vectors. The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA single-vector groups respectively. The vector group symbol is preferred for disassembly, but optional in assembler source code. This instruction follows SME2 ZA-targeting non-widening BFloat16 numerical behaviors. This instruction is unpredicated. ID_AA64SMFR0_EL1.B16B16 indicates whether this instruction is implemented. It has encodings from 2 classes: [Two ZA single-vectors](#iclass_two_za_single_vectors) and [Four ZA single-vectors](#iclass_four_za_single_vectors)

#### Decode

if !IsFeatureImplemented(FEAT_SME_B16B16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let m : integer = UInt(Zm::'00'); let offset : integer = UInt(off3); let nreg : integer{} = 4;

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; var result : bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = ZAvector{}(vec); let operand2 : bits(VL) = Z{}(m+r); for e = 0 to elements-1 do let element1 : bits(16) = operand1[e\*:16]; let element2 : bits(16) = operand2[e\*:16]; result[e\*:16] = [BFSub_ZA](shared_pseudocode.html#func_BFSub_ZA_3)(element1, element2, FPCR()); end; [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec) = result; vec = vec + vstride; end;
