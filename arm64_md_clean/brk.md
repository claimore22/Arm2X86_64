## BRK

Breakpoint instruction This instruction generates a Breakpoint Instruction exception. The PE records the exception in ESR_ELx, using the EC value 0x3C, and captures the value of the immediate argument in ESR_ELx.ISS. Within a guarded memory region, while PSTATE.BTYPE != 0b00, a BRK instruction will not generate a Branch Target exception and will generate a Breakpoint Instruction exception as normal. For more information, see PSTATE.BTYPE.

#### Decode

let comment : bits(16) = imm16; if IsFeatureImplemented(FEAT_BTI) then [SetBTypeCompatible](shared_pseudocode.html#func_SetBTypeCompatible_1)(TRUE); end;

### Operation

[AArch64_SoftwareBreakpoint](shared_pseudocode.html#func_AArch64_SoftwareBreakpoint_1)(comment);
