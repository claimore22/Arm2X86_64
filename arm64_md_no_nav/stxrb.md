## STXRB

Store exclusive register byte

This instruction stores a byte from a register to memory if the PE has exclusive access to the memory address, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See <span class="armarm-xref" data-linkend="ARMARM_Chdcgdja">Synchronization and semaphores</span>. The memory access is atomic.

For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

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
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rs</td>
<td class="lr">0</td>
<td class="l">(1)</td>
<td>(1)</td>
<td>(1)</td>
<td>(1)</td>
<td class="r">(1)</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="2"></td>
<td></td>
<td></td>
<td></td>
<td colspan="2"></td>
<td class="droppedname">L</td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">o0</td>
<td colspan="5" class="droppedname">Rt2</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="STXRB_SR32_ldstexclr"></span>

STXRB [\<Ws\>](#WsOrWZR__4 "Is the 32-bit name of the general-purpose register into which the status result of the store exclusive is written, encoded in the "Rs" field. The value returned is:   0 : If the operation updates memory.  1 : If the operation fails to update memory."), [\<Wt\>](#WtOrWZR__4 "Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

#### Decode for this encoding

let s : integer{} = UInt(Rs); let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let acqrel : boolean = FALSE; let tagchecked : boolean = n != 31; var rt_unknown : boolean = FALSE; var rn_unknown : boolean = FALSE; if s == t then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_DATAOVERLAP](shared_pseudocode.html#enum_Unpredictable_DATAOVERLAP)); assert c IN {[Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> rt_unknown = TRUE; // store UNKNOWN value when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end; if s == n && n != 31 then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_BASEOVERLAP](shared_pseudocode.html#enum_Unpredictable_BASEOVERLAP)); assert c IN {[Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> rn_unknown = TRUE; // address is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end;

<div class="encoding-notes">

For information about the <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span> behavior of this instruction, see <span class="armarm-xref" data-linkend="CJAEGDJC">Architectural Constraints on UNPREDICTABLE behaviors</span>, and particularly <span class="armarm-xref" data-linkend="CHDDGIAF">STXRB</span>.

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Ws&gt;</td>
<td><span id="WsOrWZR__4"></span>
<p>Is the 32-bit name of the general-purpose register into which the status result of the store exclusive is written, encoded in the "Rs" field. The value returned is:</p>
<dl>
<dt>0</dt>
<dd>
If the operation updates memory.
</dd>
<dt>1</dt>
<dd>
If the operation fails to update memory.
</dd>
</dl></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wt&gt;</td>
<td><span id="WtOrWZR__4"></span>
<p>Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
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

</div>

<div class="syntax-notes">

Aborts

If a synchronous Data Abort exception is generated by the execution of this instruction:

- Memory is not updated.
- \<Ws\> is not updated.

If AArch64_ExclusiveMonitorsPass() returns FALSE and the memory address, if accessed, would generate a synchronous Data Abort exception, it is <span class="arm-defined-word">IMPLEMENTATION DEFINED</span> whether the exception is generated.

</div>

<div class="ps">

### Operation

var address : bits(64); var data : bits(8); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescExLDST](shared_pseudocode.html#func_CreateAccDescExLDST_5)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), acqrel, tagchecked, privileged, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); elsif rn_unknown then address = ARBITRARY : bits(64); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; if rt_unknown then data = ARBITRARY : bits(8); else data = [X](shared_pseudocode.html#accessor_X_2){8}(t); end; var status : bit = '1'; // Check whether the Exclusives monitors are set to include the // physical memory locations corresponding to virtual address // range \[address, address+dbytes-1\]. // If AArch64_ExclusiveMonitorsPass() returns FALSE and the memory address, // if accessed, would generate a synchronous Data Abort exception, it is // IMPLEMENTATION DEFINED whether the exception is generated. // It is a limitation of this model that synchronous Data Aborts are never // generated in this case, as Mem is not called. // If FEAT_SPE is implemented, it is also IMPLEMENTATION DEFINED whether or not the // physical address packet is output when permitted and when // AArch64_ExclusiveMonitorPass() returns FALSE for a Store Exclusive instruction. // This behavior is not reflected here due to the previously stated limitation. if [AArch64_ExclusiveMonitorsPass](shared_pseudocode.html#func_AArch64_ExclusiveMonitorsPass_3)(address, 1, accdesc) then // This atomic write will be rejected if it does not refer // to the same physical locations after address translation. [Mem](shared_pseudocode.html#accessor_Mem_3){8}(address, accdesc) = data; status = [ExclusiveMonitorsStatus](shared_pseudocode.html#func_ExclusiveMonitorsStatus_0)(); end; [X](shared_pseudocode.html#accessor_X_2){32}(s) = ZeroExtend{32}(status);

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
