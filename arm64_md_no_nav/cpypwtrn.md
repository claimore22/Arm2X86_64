## CPYPWTRN, CPYMWTRN, CPYEWTRN

Memory copy, writes unprivileged, reads non-temporal

These instructions copy a requested number of bytes in memory from a source address to a destination address. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWTRN, then CPYMWTRN, and then CPYEWTRN.

CPYPWTRN performs some preconditioning of the arguments suitable for using the CPYMWTRN instruction, and copies an <span class="arm-defined-word">IMPLEMENTATION DEFINED</span> portion of the requested number of bytes. CPYMWTRN copies a further <span class="arm-defined-word">IMPLEMENTATION DEFINED</span> portion of the remaining bytes. CPYEWTRN copies any final remaining bytes.

<div class="note">

------------------------------------------------------------------------

#### Note

The ability to copy an <span class="arm-defined-word">IMPLEMENTATION DEFINED</span> number of bytes allows an implementation to optimize how the bytes being copied are divided between the different instructions.

------------------------------------------------------------------------

</div>

For more information on exceptions specific to memory copy instructions, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.memcpy_and_memset_exceptions">Memory Copy and Memory Set exceptions</span>.

The architecture supports two algorithms for the memory copy: option A and option B. Which algorithm is used is <span class="arm-defined-word">IMPLEMENTATION DEFINED</span>.

<div class="note">

------------------------------------------------------------------------

#### Note

Portable software should not assume that the choice of algorithm is constant.

------------------------------------------------------------------------

</div>

For CPYPWTRN:

- If Xn\[63:55\] != '000000000', the copy size Xn is saturated to 0x007FFFFFFFFFFFFF.

- After saturation is performed, the direction of the memory copy is based on the following:

  If (Xs\[55:0\] \> Xd\[55:0\]) and (Xd\[55:0\] + saturated copy size) \> Xs\[55:0\], then the direction is forward.

  If (Xs\[55:0\] \< Xd\[55:0\]) and (Xs\[55:0\] + saturated copy size) \> Xd\[55:0\], then the direction is backward.

  Otherwise, the direction is an <span class="arm-defined-word">IMPLEMENTATION DEFINED</span> choice between forward and backward.

On completion of CPYPWTRN, option A:

- PSTATE.{N,Z,C,V} are set to {0,0,0,0}.
- If the copy is in the forward direction, then:
  - Xn holds -1 times the number of bytes in the saturated copy size remaining to be copied.
  - Xs holds the original Xs + saturated copy size.
  - Xd holds the original Xd + saturated copy size.
- If the copy is in the backward direction, then:
  - Xn holds the number of bytes in the saturated copy size remaining to be copied.
  - Xs and Xd are unchanged.

On completion of CPYPWTRN, option B:

- If the copy is in the forward direction, then:
  - Xn holds the number of bytes in the saturated copy size remaining to be copied.
  - Xs holds the lowest address that has not been copied from.
  - Xd holds the lowest address that has not been copied to.
  - PSTATE.{N,Z,C,V} are set to {0,0,1,0}.
- If the copy is in the backward direction, then:
  - Xn holds the number of bytes in the saturated copy size remaining to be copied.
  - Xs holds the highest address that has not been copied from + 1.
  - Xd holds the highest address that has not been copied to + 1.
  - PSTATE.{N,Z,C,V} are set to {1,0,1,0}.

For CPYMWTRN, option A, when PSTATE.C = 0:

- Xn holds a signed 64-bit integer.
- If the copy is in the forward direction (Xn holds a negative number), then:
  - Xn holds -1 times the number of bytes remaining to be copied.
  - Xs holds the lowest address to be copied from - Xn.
  - Xd holds the lowest address to be copied to - Xn.
  - On completion of the instruction, Xn holds -1 times the number of bytes remaining to be copied.
- If the copy is in the backward direction (Xn holds a positive number), then:
  - Xn holds the number of bytes remaining to be copied.
  - Xs holds the highest address to be copied from + 1 - Xn.
  - Xd holds the highest address to be copied to + 1 - Xn.
  - On completion of the instruction, Xn holds the number of bytes remaining to be copied.

For CPYMWTRN, option B, when PSTATE.C = 1:

- Xn holds the number of bytes remaining to be copied.
- If the copy is in the forward direction (PSTATE.N == 0), then:
  - Xs holds the lowest address to be copied from.
  - Xd holds the lowest address to be copied to.
  - On completion of the instruction:
    - Xn holds the number of bytes remaining to be copied.
    - Xs holds the lowest address that has not been copied from.
    - Xd holds the lowest address that has not been copied to.
- If the copy is in the backward direction (PSTATE.N == 1), then:
  - Xs holds the highest address to be copied from + 1.
  - Xd holds the highest address to be copied to + 1.
  - On completion of the instruction:
    - Xn holds the number of bytes remaining to be copied.
    - Xs holds the highest address that has not been copied from + 1.
    - Xd holds the highest address that has not been copied to + 1.

For CPYEWTRN, option A, when PSTATE.C = 0:

- Xn holds a signed 64-bit integer.

- If the copy is in the forward direction (Xn holds a negative number), then:
  - Xn holds -1 times the number of bytes remaining to be copied.
  - Xs holds the lowest address to be copied from - Xn.
  - Xd holds the lowest address to be copied to - Xn.

- If the copy is in the backward direction (Xn holds a positive number), then:

  - Xn holds the number of bytes remaining to be copied.
  - Xs holds the highest address to be copied from + 1 - Xn.
  - Xd holds the highest address to be copied to + 1 - Xn.

- On completion of the instruction:

  - Xn holds 0.
  - Xs holds an <span class="arm-defined-word">IMPLEMENTATION SPECIFIC</span> value in the range with bounds:
    - The lowest address that has been copied from.
    - The highest address that has been copied from + 1.
  - Xd holds an <span class="arm-defined-word">IMPLEMENTATION SPECIFIC</span> value in the range with bounds:
    - The lowest address that has been copied to.
    - The highest address that has not been copied to + 1.

For CPYEWTRN, option B, when PSTATE.C = 1:

- Xn holds the number of bytes remaining to be copied.

- If the copy is in the forward direction (PSTATE.N == 0), then:
  - Xs holds the lowest address to be copied from.
  - Xd holds the lowest address to be copied to.

- If the copy is in the backward direction (PSTATE.N == 1), then:

  - Xs holds the highest address to be copied from + 1.
  - Xd holds the highest address to be copied to + 1.

- On completion of the instruction:

  - Xn holds 0.
  - Xs holds an <span class="arm-defined-word">IMPLEMENTATION SPECIFIC</span> value in the range with bounds:
    - The lowest address that has been copied from.
    - The highest address that has been copied from + 1.
  - Xd holds an <span class="arm-defined-word">IMPLEMENTATION SPECIFIC</span> value in the range with bounds:
    - The lowest address that has been copied to.
    - The highest address that has not been copied to + 1.

Explicit Memory Write effects produced by the instruction behave as if the instruction was executed at EL0 if the <span class="armarm-xref" data-linkend="ARMARM_Effective_value">Effective value</span> of PSTATE.UAO is 0 and either:

- The instruction is executed at EL1.
- The instruction is executed at EL2 when the <span class="armarm-xref" data-linkend="ARMARM_Effective_value">Effective value</span> of HCR_EL2().{E2H, TGE} is '11'.

Otherwise, the Explicit Memory Write effects operate with the restrictions determined by the Exception level at which the instruction is executed.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_MOPS) </span>

<div class="regdiagram-32">

<table class="regdiagram">
<thead>
<tr>
<th>31</th>
<th>30</th>
<th>29</th>
<th>28</th>
<th>27</th>
<th>26</th>
<th>25</th>
<th>24</th>
<th>23</th>
<th>22</th>
<th>21</th>
<th>20</th>
<th>19</th>
<th>18</th>
<th>17</th>
<th>16</th>
<th>15</th>
<th>14</th>
<th>13</th>
<th>12</th>
<th>11</th>
<th>10</th>
<th>9</th>
<th>8</th>
<th>7</th>
<th>6</th>
<th>5</th>
<th>4</th>
<th>3</th>
<th>2</th>
<th>1</th>
<th>0</th>
</tr>
</thead>
<tbody>
<tr class="firstrow">
<td colspan="2" class="lr">sz</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="2" class="lr">op1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rs</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
<td colspan="3"></td>
<td class="droppedname">o0</td>
<td colspan="2"></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="4" class="droppedname">op2</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Prologue variant

<span id="CPYPWTRN_CPY_memcms"></span> Applies when <span class="bitdiff"> (op1 == 00)</span>

CPYPWTRN \[[\<Xd\>](#XdOrXZR "For the "Prologue" variant: is the 64-bit name of the general-purpose register that holds the destination address and is updated by the instruction, encoded in the "Rd" field.")\]!, \[[\<Xs\>](#XsOrXZR__5 "For the "Prologue" variant: is the 64-bit name of the general-purpose register that holds the source address and is updated by the instruction, encoded in the "Rs" field.")\]!, [\<Xn\>](#XnOrXZR__2 "For the "Prologue" variant: is the 64-bit name of the general-purpose register that holds the number of bytes to be transferred and is updated by the instruction to encode the remaining size and destination, encoded in the "Rn" field.")!

</div>

<div class="encoding">

#### Encoding for the Main variant

<span id="CPYMWTRN_CPY_memcms"></span> Applies when <span class="bitdiff"> (op1 == 01)</span>

CPYMWTRN \[[\<Xd\>](#XdOrXZR__2 "For the "Epilogue" and "Main" variants: is the 64-bit name of the general-purpose register that holds an encoding of the destination address, encoded in the "Rd" field.")\]!, \[[\<Xs\>](#XsOrXZR__6 "For the "Epilogue" and "Main" variants: is the 64-bit name of the general-purpose register that holds an encoding of the source address, encoded in the "Rs" field.")\]!, [\<Xn\>](#XnOrXZR__3 "For the "Main" variant: is the 64-bit name of the general-purpose register that holds an encoding of the number of bytes to be transferred, encoded in the "Rn" field.")!

</div>

<div class="encoding">

#### Encoding for the Epilogue variant

<span id="CPYEWTRN_CPY_memcms"></span> Applies when <span class="bitdiff"> (op1 == 10)</span>

CPYEWTRN \[[\<Xd\>](#XdOrXZR__2 "For the "Epilogue" and "Main" variants: is the 64-bit name of the general-purpose register that holds an encoding of the destination address, encoded in the "Rd" field.")\]!, \[[\<Xs\>](#XsOrXZR__6 "For the "Epilogue" and "Main" variants: is the 64-bit name of the general-purpose register that holds an encoding of the source address, encoded in the "Rs" field.")\]!, [\<Xn\>](#XnOrXZR__4 "For the "Epilogue" variant: is the 64-bit name of the general-purpose register that holds an encoding of the number of bytes to be transferred and is set to zero on completion of the instruction, encoded in the "Rn" field.")!

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_MOPS) \|\| sz != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; var memcpy : [CPYParams](shared_pseudocode.html#type_CPYParams); memcpy.d = UInt(Rd); memcpy.s = UInt(Rs); memcpy.n = UInt(Rn); let options : bits(4) = op2; let rnontemporal : boolean = options\[3\] == '1'; let wnontemporal : boolean = options\[2\] == '1'; case op1 of when '00' =\> memcpy.stage = [MOPSStage_Prologue](shared_pseudocode.html#enum_MOPSStage_Prologue); when '01' =\> memcpy.stage = [MOPSStage_Main](shared_pseudocode.html#enum_MOPSStage_Main); when '10' =\> memcpy.stage = [MOPSStage_Epilogue](shared_pseudocode.html#enum_MOPSStage_Epilogue); end;

<div class="encoding-notes">

For information about the <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span> behavior of this instruction, see <span class="armarm-xref" data-linkend="CJAEGDJC">Architectural Constraints on UNPREDICTABLE behaviors</span>, and particularly <span class="armarm-xref" data-linkend="CEGCIDDEI6">Memory Copy and Memory Set CPY\*</span> and <span class="armarm-xref" data-linkend="ARMARM_CEGCCGAE">Crossing a page boundary with different memory types or Shareability attributes</span>.

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR"></span>
<p>For the "Prologue" variant: is the 64-bit name of the general-purpose register that holds the destination address and is updated by the instruction, encoded in the "Rd" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="XdOrXZR__2"></span>
<p>For the "Epilogue" and "Main" variants: is the 64-bit name of the general-purpose register that holds an encoding of the destination address, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xs&gt;</td>
<td><span id="XsOrXZR__5"></span>
<p>For the "Prologue" variant: is the 64-bit name of the general-purpose register that holds the source address and is updated by the instruction, encoded in the "Rs" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="XsOrXZR__6"></span>
<p>For the "Epilogue" and "Main" variants: is the 64-bit name of the general-purpose register that holds an encoding of the source address, encoded in the "Rs" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR__2"></span>
<p>For the "Prologue" variant: is the 64-bit name of the general-purpose register that holds the number of bytes to be transferred and is updated by the instruction to encode the remaining size and destination, encoded in the "Rn" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="XnOrXZR__3"></span>
<p>For the "Main" variant: is the 64-bit name of the general-purpose register that holds an encoding of the number of bytes to be transferred, encoded in the "Rn" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="XnOrXZR__4"></span>
<p>For the "Epilogue" variant: is the 64-bit name of the general-purpose register that holds an encoding of the number of bytes to be transferred and is set to zero on completion of the instruction, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckMOPSEnabled](shared_pseudocode.html#func_CheckMOPSEnabled_0)(); [CheckCPYConstrainedUnpredictable](shared_pseudocode.html#func_CheckCPYConstrainedUnpredictable_3)(memcpy.n, memcpy.d, memcpy.s); memcpy.nzcv = [PSTATE](shared_pseudocode.html#global_PSTATE).\[N,Z,C,V\]; memcpy.toaddress = [X](shared_pseudocode.html#accessor_X_2){64}(memcpy.d); memcpy.fromaddress = [X](shared_pseudocode.html#accessor_X_2){64}(memcpy.s); if memcpy.stage == [MOPSStage_Prologue](shared_pseudocode.html#enum_MOPSStage_Prologue) then memcpy.cpysize = UInt([X](shared_pseudocode.html#accessor_X_2){64}(memcpy.n)); else memcpy.cpysize = SInt([X](shared_pseudocode.html#accessor_X_2){64}(memcpy.n)); end; memcpy.implements_option_a = [CPYOptionA](shared_pseudocode.html#func_CPYOptionA_0)(); let rprivileged : boolean = (if options\[1\] == '1' then [AArch64_IsUnprivAccessPriv](shared_pseudocode.html#func_AArch64_IsUnprivAccessPriv_0)() else [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0)); let wprivileged : boolean = (if options\[0\] == '1' then [AArch64_IsUnprivAccessPriv](shared_pseudocode.html#func_AArch64_IsUnprivAccessPriv_0)() else [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0)); let raccdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescMOPS](shared_pseudocode.html#func_CreateAccDescMOPS_3)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), rprivileged, rnontemporal); let waccdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescMOPS](shared_pseudocode.html#func_CreateAccDescMOPS_3)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), wprivileged, wnontemporal); if memcpy.stage == [MOPSStage_Prologue](shared_pseudocode.html#enum_MOPSStage_Prologue) then if memcpy.cpysize \> [ArchMaxMOPSCPYSize](shared_pseudocode.html#global_ArchMaxMOPSCPYSize) then memcpy.cpysize = [ArchMaxMOPSCPYSize](shared_pseudocode.html#global_ArchMaxMOPSCPYSize); end; memcpy.forward = [IsMemCpyForward](shared_pseudocode.html#func_IsMemCpyForward_1)(memcpy); if memcpy.implements_option_a then memcpy.nzcv = '0000'; if memcpy.forward then // Copy in the forward direction offsets the arguments. memcpy.toaddress = memcpy.toaddress + memcpy.cpysize; memcpy.fromaddress = memcpy.fromaddress + memcpy.cpysize; memcpy.cpysize = 0 - memcpy.cpysize; end; else if !memcpy.forward then // Copy in the reverse direction offsets the arguments. memcpy.toaddress = memcpy.toaddress + memcpy.cpysize; memcpy.fromaddress = memcpy.fromaddress + memcpy.cpysize; memcpy.nzcv = '1010'; else memcpy.nzcv = '0010'; end; end; end; memcpy.stagecpysize = [MemCpyStageSize](shared_pseudocode.html#func_MemCpyStageSize_1)(memcpy); if memcpy.stage != [MOPSStage_Prologue](shared_pseudocode.html#enum_MOPSStage_Prologue) then memcpy.forward = memcpy.cpysize \< 0 \|\| (!memcpy.implements_option_a && memcpy.nzcv\[3\] == '0'); [CheckMemCpyParams](shared_pseudocode.html#func_CheckMemCpyParams_2)(memcpy, options); end; var copied : integer; var iswrite : boolean; var memaddrdesc : [AddressDescriptor](shared_pseudocode.html#type_AddressDescriptor); var memstatus : [PhysMemRetStatus](shared_pseudocode.html#type_PhysMemRetStatus); var fault : boolean = FALSE; var B : [MOPSBlockSize](shared_pseudocode.html#type_MOPSBlockSize) = 0; if memcpy.implements_option_a then while memcpy.stagecpysize != 0 && !fault looplimit [ArchMaxMOPSCPYSize](shared_pseudocode.html#global_ArchMaxMOPSCPYSize) do // IMP DEF selection of the block size that is worked on. While many // implementations might make this constant, that is not assumed. B = [CPYSizeChoice](shared_pseudocode.html#func_CPYSizeChoice_1)(memcpy); if memcpy.forward then assert B \<= -1 \* memcpy.stagecpysize; (copied, iswrite, memaddrdesc, memstatus) = [MemCpyBytes](shared_pseudocode.html#func_MemCpyBytes_6)( memcpy.toaddress + memcpy.cpysize, memcpy.fromaddress + memcpy.cpysize, memcpy.forward, B, raccdesc, waccdesc); if copied != B then fault = TRUE; else memcpy.cpysize = memcpy.cpysize + B; memcpy.stagecpysize = memcpy.stagecpysize + B; end; else assert B \<= memcpy.stagecpysize; memcpy.cpysize = memcpy.cpysize - B; memcpy.stagecpysize = memcpy.stagecpysize - B; (copied, iswrite, memaddrdesc, memstatus) = [MemCpyBytes](shared_pseudocode.html#func_MemCpyBytes_6)( memcpy.toaddress + memcpy.cpysize, memcpy.fromaddress + memcpy.cpysize, memcpy.forward, B, raccdesc, waccdesc); if copied != B then fault = TRUE; memcpy.cpysize = memcpy.cpysize + B; memcpy.stagecpysize = memcpy.stagecpysize + B; end; end; end; else while memcpy.stagecpysize \> 0 && !fault looplimit [ArchMaxMOPSCPYSize](shared_pseudocode.html#global_ArchMaxMOPSCPYSize) do // IMP DEF selection of the block size that is worked on. While many // implementations might make this constant, that is not assumed. B = [CPYSizeChoice](shared_pseudocode.html#func_CPYSizeChoice_1)(memcpy); assert B \<= memcpy.stagecpysize; if memcpy.forward then (copied, iswrite, memaddrdesc, memstatus) = [MemCpyBytes](shared_pseudocode.html#func_MemCpyBytes_6)(memcpy.toaddress, memcpy.fromaddress, memcpy.forward, B, raccdesc, waccdesc); if copied != B then fault = TRUE; else memcpy.fromaddress = memcpy.fromaddress + B; memcpy.toaddress = memcpy.toaddress + B; end; else (copied, iswrite, memaddrdesc, memstatus) = [MemCpyBytes](shared_pseudocode.html#func_MemCpyBytes_6)(memcpy.toaddress - B, memcpy.fromaddress - B, memcpy.forward, B, raccdesc, waccdesc); if copied != B then fault = TRUE; else memcpy.fromaddress = memcpy.fromaddress - B; memcpy.toaddress = memcpy.toaddress - B; end; end; if !fault then memcpy.cpysize = memcpy.cpysize - B; memcpy.stagecpysize = memcpy.stagecpysize - B; end; end; end; [UpdateCpyRegisters](shared_pseudocode.html#func_UpdateCpyRegisters_3)(memcpy, fault, copied); if fault then if [IsFault](shared_pseudocode.html#func_IsFault_1)(memaddrdesc) then [AArch64_Abort](shared_pseudocode.html#func_AArch64_Abort_1)(memaddrdesc.fault); else let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = if iswrite then waccdesc else raccdesc; [HandleExternalAbort](shared_pseudocode.html#func_HandleExternalAbort_5)(memstatus, iswrite, memaddrdesc, B, accdesc); end; end; if memcpy.stage == [MOPSStage_Prologue](shared_pseudocode.html#enum_MOPSStage_Prologue) then PSTATE.\[N,Z,C,V\] = memcpy.nzcv; end;

</div>
