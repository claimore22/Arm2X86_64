## LDXP

Load exclusive pair of registers

This instruction derives an address from a base register value, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers. For information on single-copy atomicity and alignment requirements, see <span class="armarm-xref" data-linkend="ARMARM_CHDDCBCC">Requirements for single-copy atomicity</span> and <span class="armarm-xref" data-linkend="ARMARM_CHDFFEGJ">Alignment of data accesses</span>. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See <span class="armarm-xref" data-linkend="ARMARM_Chdcgdja">Synchronization and semaphores</span>. For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

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
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">(1)</td>
<td>(1)</td>
<td>(1)</td>
<td>(1)</td>
<td class="r">(1)</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rt2</td>
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
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDXP_LP32_ldstexclp"></span> Applies when <span class="bitdiff"> (sz == 0)</span>

LDXP [\<Wt1\>](#Wt1OrWZR "Is the 32-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Wt2\>](#Wt2OrWZR "Is the 32-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDXP_LP64_ldstexclp"></span> Applies when <span class="bitdiff"> (sz == 1)</span>

LDXP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

#### Decode for all variants of this encoding

let t : integer{} = UInt(Rt); let t2 : integer{} = UInt(Rt2); let n : integer{} = UInt(Rn); let elsize : integer{} = 32 \<\< UInt(sz); let datasize : integer{} = elsize \* 2; let acqrel : boolean = FALSE; let tagchecked : boolean = n != 31; var rt_unknown : boolean = FALSE; if t == t2 then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_LDPOVERLAP](shared_pseudocode.html#enum_Unpredictable_LDPOVERLAP)); assert c IN {[Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> rt_unknown = TRUE; // result is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end;

<div class="encoding-notes">

For information about the <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span> behavior of this instruction, see <span class="armarm-xref" data-linkend="CJAEGDJC">Architectural Constraints on UNPREDICTABLE behaviors</span>, and particularly <span class="armarm-xref" data-linkend="CHDDEEBD">LDXP</span>.

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wt1&gt;</td>
<td><span id="Wt1OrWZR"></span>
<p>Is the 32-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wt2&gt;</td>
<td><span id="Wt2OrWZR"></span>
<p>Is the 32-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field.</p></td>
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
<td>&lt;Xt1&gt;</td>
<td><span id="Xt1OrXZR"></span>
<p>Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt2&gt;</td>
<td><span id="Xt2OrXZR"></span>
<p>Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

var address : bits(64); var data : bits(datasize); let dbytes : integer{} = datasize DIV 8; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let ispair : boolean = elsize == 64; // When elsize is 32, the access is single-copy atomic let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescExLDST](shared_pseudocode.html#func_CreateAccDescExLDST_7)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), acqrel, tagchecked, privileged, ispair, t, t2); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; [AArch64_SetExclusiveMonitors](shared_pseudocode.html#func_AArch64_SetExclusiveMonitors_3)(address, dbytes, accdesc); data = [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc); if rt_unknown then // ConstrainedUNPREDICTABLE case [X](shared_pseudocode.html#accessor_X_2){datasize}(t) = ARBITRARY : bits(datasize); // In this case t = t2 elsif [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then [X](shared_pseudocode.html#accessor_X_2){elsize}(t) = data\[elsize +: elsize\]; [X](shared_pseudocode.html#accessor_X_2){elsize}(t2) = data\[0 +: elsize\]; else [X](shared_pseudocode.html#accessor_X_2){elsize}(t) = data\[0 +: elsize\]; [X](shared_pseudocode.html#accessor_X_2){elsize}(t2) = data\[elsize +: elsize\]; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
