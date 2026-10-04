## SUBG

Subtract with tag

This instruction subtracts an immediate value scaled by the Tag Granule from the address in the source register, modifies the Logical Address Tag of the address using an immediate value, and writes the result to the destination register. Tags specified in GCR_EL1.Exclude are excluded from the possible outputs when modifying the Logical Address Tag.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_MTE) </span>

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
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="6" class="lr">imm6</td>
<td class="l">(0)</td>
<td class="r">(0)</td>
<td colspan="4" class="lr">imm4</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">sf</td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="6"></td>
<td colspan="2" class="droppedname">op3</td>
<td colspan="4"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SUBG_64_addsub_immtags"></span>

SUBG [\<Xd\|SP\>](#XdSP_option "Is the 64-bit name of the destination general-purpose register or stack pointer, encoded in the "Rd" field."), [\<Xn\|SP\>](#XnSP_option__3 "Is the 64-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field."), \#[\<uimm6\>](#uimm6 "Is an unsigned immediate, a multiple of 16 in the range 0 to 1008, encoded in the "imm6" field."), \#[\<uimm4\>](#uimm4 "Is an unsigned immediate, in the range 0 to 15, encoded in the "imm4" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let tag_offset : bits(4) = imm4; let offset : bits(64) = LSL(ZeroExtend{64}(imm6), [LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE));

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xd|SP&gt;</td>
<td><span id="XdSP_option"></span>
<p>Is the 64-bit name of the destination general-purpose register or stack pointer, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option__3"></span>
<p>Is the 64-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;uimm6&gt;</td>
<td><span id="uimm6"></span>
<p>Is an unsigned immediate, a multiple of 16 in the range 0 to 1008, encoded in the "imm6" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;uimm4&gt;</td>
<td><span id="uimm4"></span>
<p>Is an unsigned immediate, in the range 0 to 15, encoded in the "imm4" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let operand1 : bits(64) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(n); let start_tag : bits(4) = [AArch64_AllocationTagFromAddress](shared_pseudocode.html#func_AArch64_AllocationTagFromAddress_1)(operand1); let exclude : bits(16) = GCR_EL1().Exclude; let rtag : bits(4) = [AArch64_ChooseNonExcludedTagOrZero](shared_pseudocode.html#func_AArch64_ChooseNonExcludedTagOrZero_3)(start_tag, tag_offset, exclude); var result : bits(64); (result, -) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){64}(operand1, NOT(offset), '1'); result = [AArch64_AddressWithAllocationTag](shared_pseudocode.html#func_AArch64_AddressWithAllocationTag_2)(result, rtag); if d == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = result; else [X](shared_pseudocode.html#accessor_X_2){64}(d) = result; end;

</div>
