## PACIASPPC

Pointer Authentication Code for return address, using key A This instruction calculates and inserts a Pointer Authentication Code for an instruction address, using two modifiers and key A. The address is in X30. The first modifier is in SP. The second modifier is the value of PC. A PACIASPPC instruction has an implicit BTI instruction. The implicit BTI instruction of a PACIASPPC instruction is always compatible with PSTATE.BTYPE == 0b01 and PSTATE.BTYPE == 0b10. Controls in SCTLR_ELx configure whether the implicit BTI instruction of a PACIASPPC instruction is compatible with PSTATE.BTYPE == 0b11. For more information, see PSTATE.BTYPE.

#### Decode

if !IsFeatureImplemented(FEAT_PAuth_LR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = 30; if IsFeatureImplemented(FEAT_BTI) then // Check for branch target compatibility between PSTATE.BTYPE // and implicit branch target of PACIxSPPC instruction. let pacinst : [PACInstType](shared_pseudocode.html#type_PACInstType) = [PACIxSPPC](shared_pseudocode.html#enum_PACIxSPPC); [SetBTypeCompatible](shared_pseudocode.html#func_SetBTypeCompatible_1)([BTypeCompatible_PAC](shared_pseudocode.html#func_BTypeCompatible_PAC_1)(pacinst)); end;

### Operation

[X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACIA2](shared_pseudocode.html#func_AddPACIA2_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}(), [PC64](shared_pseudocode.html#func_PC64_0)());
