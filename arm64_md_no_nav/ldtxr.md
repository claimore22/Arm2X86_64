## LDTXR

Load unprivileged exclusive register

This instruction derives an address from a base register value, loads a 32-bit word or a 64-bit doubleword from memory, and writes it to a register. The memory access is atomic. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See <span class="armarm-xref" data-linkend="ARMARM_Chdcgdja">Synchronization and semaphores</span>.

Explicit Memory effects produced by the instruction behave as if the instruction was executed at EL0 if the <span class="armarm-xref" data-linkend="ARMARM_Effective_value">Effective value</span> of PSTATE.UAO is 0 and either:

- The instruction is executed at EL1.
- The instruction is executed at EL2 when the <span class="armarm-xref" data-linkend="ARMARM_Effective_value">Effective value</span> of HCR_EL2().{E2H, TGE} is '11'.

Otherwise, the Explicit Memory effects operate with the restrictions determined by the Exception level at which the instruction is executed.

<div class="note">

------------------------------------------------------------------------

#### Note

For the purposes of the Exclusives monitors, and the forward progress guarantees for Load-Exclusive and Store-Exclusive loops, <span class="asm-code">LDTXR</span> is equivalent to <span class="asm-code">LDXR</span>.

------------------------------------------------------------------------

</div>

For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

### <span id="iclass_no_offset"></span>No offset<span style="font-size:smaller;">\
(FEAT_LSUI) </span>

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
<td class="lr">1</td>
<td class="lr">sz</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">(1)</td>
<td>(1)</td>
<td>(1)</td>
<td>(1)</td>
<td class="r">(1)</td>
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
<td></td>
<td></td>
<td colspan="7"></td>
<td class="droppedname">L</td>
<td></td>
<td colspan="5" class="droppedname">Rs</td>
<td class="droppedname">o0</td>
<td colspan="5" class="droppedname">Rt2</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDTXR_LR32_ldstexclr_unpriv"></span> Applies when <span class="bitdiff"> (sz == 0)</span>

LDTXR [\<Wt\>](#WtOrWZR__4 "Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDTXR_LR64_ldstexclr_unpriv"></span> Applies when <span class="bitdiff"> (sz == 1)</span>

LDTXR [\<Xt\>](#XtOrXZR__11 "Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_LSUI) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let elsize : integer{} = 32 \<\< UInt(sz); let regsize : integer{} = if elsize == 64 then 64 else 32; let acqrel : boolean = FALSE; let tagchecked : boolean = n != 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

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

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__11"></span>
<p>Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

var address : bits(64); var data : bits(elsize); let dbytes : integer{} = elsize DIV 8; let privileged : boolean = [AArch64_IsUnprivAccessPriv](shared_pseudocode.html#func_AArch64_IsUnprivAccessPriv_0)(); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescExLDST](shared_pseudocode.html#func_CreateAccDescExLDST_5)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), acqrel, tagchecked, privileged, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; [AArch64_SetExclusiveMonitors](shared_pseudocode.html#func_AArch64_SetExclusiveMonitors_3)(address, dbytes, accdesc); data = [Mem](shared_pseudocode.html#accessor_Mem_3){elsize}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){regsize}(t) = ZeroExtend{regsize}(data);

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
