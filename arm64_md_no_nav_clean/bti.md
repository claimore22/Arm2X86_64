## BTI

Branch target identification This instruction is used to guard against the execution of instructions that are not the intended target of a branch. Outside of a guarded memory region, a BTI instruction executes as a NOP. Within a guarded memory region, while PSTATE.BTYPE != 0b00, a BTI instruction compatible with the current value of PSTATE.BTYPE will not generate a Branch Target Exception and will allow execution of subsequent instructions within the memory region. For more information, see PSTATE.BTYPE. The operand <targets> passed to a BTI instruction determines the values of PSTATE.BTYPE that the BTI instruction is compatible with.

#### Decode

if !IsFeatureImplemented(FEAT_BTI) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; // Check branch target compatibility between BTI instruction and PSTATE.BTYPE [SetBTypeCompatible](shared_pseudocode.html#func_SetBTypeCompatible_1)([BTypeCompatible_BTI](shared_pseudocode.html#func_BTypeCompatible_BTI_1)(op2[2:1]));

### Operation

[SetBTypeNext](shared_pseudocode.html#func_SetBTypeNext_1)('00');
