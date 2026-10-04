## BFM

Bitfield move This instruction is usually accessed via one of its aliases, which are always preferred for disassembly. If <imms> is greater than or equal to <immr>, this copies a bitfield of (<imms>-<immr>+1) bits starting from bit position <immr> in the source register to the least significant bits of the destination register. If <imms> is less than <immr>, this copies a bitfield of (<imms>+1) bits from the least significant bits of the source register to bit position (regsize-<immr>) of the destination register, where regsize is the destination register size of 32 or 64 bits. In both cases, the other bits of the destination register remain unchanged. This instruction is used by the aliases [BFC](bfc_bfm.html "Bitfield clear"), [BFI](bfi_bfm.html "Bitfield insert"), and [BFXIL](bfxil_bfm.html "Bitfield extract and insert at low end").

#### Decode

if sf == '1' && N != '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sf == '0' && (N != '0' \|\| immr[5] != '0' \|\| imms[5] != '0') then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 << UInt(sf); let s : integer = UInt(imms); let r : integer = UInt(immr); var wmask : bits(datasize); var tmask : bits(datasize); (wmask, tmask) = [DecodeBitMasks](shared_pseudocode.html#func_DecodeBitMasks_5){datasize}(N, imms, immr, FALSE);

### Operation

let dst : bits(datasize) = X{}(d); let src : bits(datasize) = X{}(n); // Perform bitfield move on low bits let bot : bits(datasize) = (dst AND NOT(wmask)) OR (ROR(src, r) AND wmask); // Combine extension bits and result bits [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = (dst AND NOT(tmask)) OR (bot AND tmask);
