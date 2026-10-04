## STR (table)

Store ZT0 register

This instruction stores the 64-byte ZT0 register to the memory address provided in the 64-bit scalar base register. This instruction is unpredicated.

The store is performed as contiguous byte accesses, with no endian conversion and no guarantee of single-copy atomicity larger than a byte. However, if alignment is checked, then the base register must be aligned to 16 bytes.

This instruction does not require the PE to be in Streaming SVE mode, and it is expected that this instruction will not experience a significant slowdown due to contention with other PEs that are executing in Streaming SVE mode.

### <span id="iclass_sme2"></span>SME2<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="6" class="droppedname">opc</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc2</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="str_zt_br_"></span>

STR ZT0, \[[\<Xn\|SP\>](#XnSP__3 "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP__3"></span>
<p>Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSMEEnabled](shared_pseudocode.html#func_CheckSMEEnabled_0)(); [CheckSMEZT0Enabled](shared_pseudocode.html#func_CheckSMEZT0Enabled_0)(); let elements : integer = 512 DIV 8; var addr : bits(64); let table : bits(512) = ZT0{}(); let contiguous : boolean = TRUE; let nontemporal : boolean = FALSE; let tagchecked : boolean = n != 31; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescSME](shared_pseudocode.html#func_CreateAccDescSME_4)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), nontemporal, contiguous, tagchecked); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); addr = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else addr = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let aligned : boolean = IsAlignedSize(addr, 16); if !aligned && [AlignmentEnforced](shared_pseudocode.html#func_AlignmentEnforced_0)() then let fault : [FaultRecord](shared_pseudocode.html#type_FaultRecord) = [AlignmentFault](shared_pseudocode.html#func_AlignmentFault_2)(accdesc, addr); [AArch64_Abort](shared_pseudocode.html#func_AArch64_Abort_1)(fault); end; for e = 0 to elements-1 do [AArch64_MemSingle](shared_pseudocode.html#accessor_AArch64_MemSingle_4){8}(addr, accdesc, aligned) = table\[e\*:8\]; addr = [AddressIncrement](shared_pseudocode.html#func_AddressIncrement_3)(addr, 1, accdesc); end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
