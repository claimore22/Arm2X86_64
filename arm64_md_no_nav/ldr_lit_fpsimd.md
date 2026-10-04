## LDR (literal, SIMD&FP)

Load SIMD&FP register (PC-relative literal)

This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from the PC value and an immediate offset.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

### <span id="iclass_literal"></span>Literal<span style="font-size:smaller;">\
(FEAT_FP) </span>

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
<td colspan="2" class="lr">opc</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="19" class="lr">imm19</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="2"></td>
<td colspan="19"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDR_S_loadlit"></span> Applies when <span class="bitdiff"> (opc == 00)</span>

LDR [\<St\>](#fpfar_st "Is the 32-bit name of the SIMD&FP register to be loaded, encoded in the "Rt" field."), [\<label\>](#imm19_offset__2 "Is the program label from which the data is to be loaded. Its offset from the address of this instruction, in the range +/-1MB, is encoded as "imm19" times 4.")

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDR_D_loadlit"></span> Applies when <span class="bitdiff"> (opc == 01)</span>

LDR [\<Dt\>](#fpfar_dt "Is the 64-bit name of the SIMD&FP register to be loaded, encoded in the "Rt" field."), [\<label\>](#imm19_offset__2 "Is the program label from which the data is to be loaded. Its offset from the address of this instruction, in the range +/-1MB, is encoded as "imm19" times 4.")

</div>

<div class="encoding">

#### Encoding for the 128-bit variant

<span id="LDR_Q_loadlit"></span> Applies when <span class="bitdiff"> (opc == 10)</span>

LDR [\<Qt\>](#Qt__2 "Is the 128-bit name of the SIMD&FP register to be loaded, encoded in the "Rt" field."), [\<label\>](#imm19_offset__2 "Is the program label from which the data is to be loaded. Its offset from the address of this instruction, in the range +/-1MB, is encoded as "imm19" times 4.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); if opc == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = 4 \<\< (UInt(opc) as integer{0..2}); let nontemporal : boolean = FALSE; let tagchecked : boolean = FALSE; let offset : bits(64) = SignExtend{}(imm19::'00');

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;St&gt;</td>
<td><span id="fpfar_st"></span>
<p>Is the 32-bit name of the SIMD&amp;FP register to be loaded, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;label&gt;</td>
<td><span id="imm19_offset__2"></span>
<p>Is the program label from which the data is to be loaded. Its offset from the address of this instruction, in the range +/-1MB, is encoded as "imm19" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dt&gt;</td>
<td><span id="fpfar_dt"></span>
<p>Is the 64-bit name of the SIMD&amp;FP register to be loaded, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Qt&gt;</td>
<td><span id="Qt__2"></span>
<p>Is the 128-bit name of the SIMD&amp;FP register to be loaded, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let address : bits(64) = [PC64](shared_pseudocode.html#func_PC64_0)() + offset; [AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescASIMD](shared_pseudocode.html#func_CreateAccDescASIMD_4)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, tagchecked, privileged); let data : bits(size\*8) = [Mem](shared_pseudocode.html#accessor_Mem_3){size\*8}(address, accdesc); [V](shared_pseudocode.html#accessor_V_2){size\*8}(t) = data;

</div>
