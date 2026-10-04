## STTNP (SIMD&FP)

Store unprivileged pair of SIMD&FP registers, with non-temporal hint

This instruction stores a pair of SIMD&FP registers to memory, issuing a hint to the memory system that the access is non-temporal. The address used for the store is calculated from an address from a base register value and an immediate offset. For information about non-temporal pair instructions, see <span class="armarm-xref" data-linkend="ARMARM_BABJADHH">Load/Store SIMD and Floating-point non-temporal pair</span>.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

Explicit Memory effects produced by the instruction behave as if the instruction was executed at EL0 if the <span class="armarm-xref" data-linkend="ARMARM_Effective_value">Effective value</span> of PSTATE.UAO is 0 and either:

- The instruction is executed at EL1.
- The instruction is executed at EL2 when the <span class="armarm-xref" data-linkend="ARMARM_Effective_value">Effective value</span> of HCR_EL2().{E2H, TGE} is '11'.

Otherwise, the Explicit Memory effects operate with the restrictions determined by the Exception level at which the instruction is executed.

### <span id="iclass_signed_offset"></span>Signed offset<span style="font-size:smaller;">\
(FEAT_FP && FEAT_LSUI) </span>

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
<td class="l">1</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="7" class="lr">imm7</td>
<td colspan="5" class="lr">Rt2</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">opc</td>
<td colspan="2"></td>
<td></td>
<td class="droppedname">VR</td>
<td></td>
<td colspan="2"></td>
<td class="droppedname">L</td>
<td colspan="7"></td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="STTNP_Q_ldstnapair_offs"></span>

STTNP [\<Qt1\>](#Qt1 "Is the 128-bit name of the first SIMD&FP register to be transferred, encoded in the "Rt" field."), [\<Qt2\>](#Qt2 "Is the 128-bit name of the second SIMD&FP register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<imm\>](#imm__6 "For the "128-bit" variant: is the optional signed immediate byte offset, a multiple of 16 in the range -1024 to 1008, defaulting to 0 and encoded in the "imm7" field as <imm>/16.")}\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_FP) \|\| !IsFeatureImplemented(FEAT_LSUI) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let t2 : integer{} = UInt(Rt2); let n : integer{} = UInt(Rn); let nontemporal : boolean = TRUE; let datasize : integer{} = 128; let offset : bits(64) = LSL(SignExtend{64}(imm7), 4); let tagchecked : boolean = n != 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Qt1&gt;</td>
<td><span id="Qt1"></span>
<p>Is the 128-bit name of the first SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Qt2&gt;</td>
<td><span id="Qt2"></span>
<p>Is the 128-bit name of the second SIMD&amp;FP register to be transferred, encoded in the "Rt2" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option"></span>
<p>Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__6"></span>
<p>Is the optional signed immediate byte offset, a multiple of 16 in the range -1024 to 1008, defaulting to 0 and encoded in the "imm7" field as &lt;imm&gt;/16.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); var address : bits(64); let dbytes : integer{} = datasize DIV 8; let privileged : boolean = [AArch64_IsUnprivAccessPriv](shared_pseudocode.html#func_AArch64_IsUnprivAccessPriv_0)(); let ispair : boolean = TRUE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescASIMD](shared_pseudocode.html#func_CreateAccDescASIMD_5)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), nontemporal, tagchecked, privileged, ispair); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); var data : bits(2\*datasize); if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then data = [V](shared_pseudocode.html#accessor_V_2){datasize}(t) :: [V](shared_pseudocode.html#accessor_V_2){datasize}(t2); else data = [V](shared_pseudocode.html#accessor_V_2){datasize}(t2) :: [V](shared_pseudocode.html#accessor_V_2){datasize}(t); end; [Mem](shared_pseudocode.html#accessor_Mem_3){2\*datasize}(address, accdesc) = data;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
