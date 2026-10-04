## HLT

Halt instruction This instruction can generate a Halt Instruction debug event, which causes entry into Debug state. Within a guarded memory region, while PSTATE.BTYPE != 0b00, an HLT instruction that would cause entry into Debug state will not generate a Branch Target exception and will cause entry into Debug state as normal. For more information, see PSTATE.BTYPE.

#### Decode

if EDSCR().HDE == '0' \|\| \![HaltingAllowed](shared_pseudocode.html#func_HaltingAllowed_0)() then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); elsif IsFeatureImplemented(FEAT_BTI) then [SetBTypeCompatible](shared_pseudocode.html#func_SetBTypeCompatible_1)(TRUE); end;

### Operation

let is_async : boolean = FALSE; let fault : [FaultRecord](shared_pseudocode.html#type_FaultRecord) = [NoFault](shared_pseudocode.html#func_NoFault_0)(); [Halt](shared_pseudocode.html#func_Halt_3)([DebugHalt_HaltInstruction](shared_pseudocode.html#global_DebugHalt_HaltInstruction), is_async, fault);
