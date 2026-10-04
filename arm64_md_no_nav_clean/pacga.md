## PACGA

Pointer Authentication Code, using generic key This instruction calculates the Pointer Authentication Code for a 64-bit value in the first source register, using a modifier in the second source register, and the generic key. The calculated Pointer Authentication Code is written to the most significant 32 bits of the destination register, and the least significant 32 bits of the destination register are set to zero.

#### Decode

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; var source_is_sp : boolean = FALSE; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let m : integer = UInt(Rm); if m == 31 then source_is_sp = TRUE; end;

### Operation

if source_is_sp then [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACGA](shared_pseudocode.html#func_AddPACGA_2)([X](shared_pseudocode.html#accessor_X_2){64}(n), [SP](shared_pseudocode.html#accessor_SP_1){64}()); else [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACGA](shared_pseudocode.html#func_AddPACGA_2)([X](shared_pseudocode.html#accessor_X_2){64}(n), [X](shared_pseudocode.html#accessor_X_2){64}(m)); end;
