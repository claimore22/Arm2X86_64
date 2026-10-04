## INS (element)

Insert vector element from another vector element

This instruction copies the vector element of the source SIMD&FP register to the specified vector element of the destination SIMD&FP register.

This instruction can insert data into individual elements within a SIMD&FP register without clearing the remaining bits to zero.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

This instruction is used by the alias [MOV (element)](mov_ins_advsimd_elt.html "Move vector element to another vector element").

### <span id="iclass_advanced_simd"></span>Advanced SIMD<span style="font-size:smaller;">\
(FEAT_AdvSIMD) </span>

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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">imm5</td>
<td class="lr">0</td>
<td colspan="4" class="lr">imm4</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td class="droppedname">Q</td>
<td class="droppedname">op</td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td></td>
<td colspan="4"></td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="INS_asimdins_IV_v"></span>

INS [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<Ts\>](#Ts_option "Is an element size specifier, ")\[[\<index1\>](#imm5_index__5 "Is the destination element index ")\], [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").[\<Ts\>](#Ts_option "Is an element size specifier, ")\[[\<index2\>](#imm5_index__6 "Is the source element index ")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'x0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = LowestSetBitNZ(imm5\[3:0\]); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let dst_index : integer = UInt(imm5\[4:size+1\]); let src_index : integer = UInt(imm4\[3:size\]); let idxdsize : integer{} = 64 \<\< UInt(imm4\[3\]); // imm4\<size-1:0\> is IGNORED let esize : integer{} = 8 \<\< size;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Vd&gt;</td>
<td><span id="Vd"></span>
<p>Is the name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Ts&gt;</td>
<td><span id="Ts_option"></span>
<p>Is an element size specifier, encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;Ts&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">x0000</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">xxxx1</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">xxx10</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">xx100</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">x1000</td>
<td class="symbol">D</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index1&gt;</td>
<td><span id="imm5_index__5"></span>
<p>Is the destination element index encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;index1&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">x0000</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">xxxx1</td>
<td class="symbol">UInt(imm5[4:1])</td>
</tr>
<tr>
<td class="bitfield">xxx10</td>
<td class="symbol">UInt(imm5[4:2])</td>
</tr>
<tr>
<td class="bitfield">xx100</td>
<td class="symbol">UInt(imm5[4:3])</td>
</tr>
<tr>
<td class="bitfield">x1000</td>
<td class="symbol">UInt(imm5[4])</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vn&gt;</td>
<td><span id="Vn"></span>
<p>Is the name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index2&gt;</td>
<td><span id="imm5_index__6"></span>
<p>Is the source element index encoded in “(imm5 :: imm4)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;index2&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">x0000</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">xxxx1</td>
<td class="symbol">UInt(imm4)</td>
</tr>
<tr>
<td class="bitfield">xxx10</td>
<td class="symbol">UInt(imm4[3:1])</td>
</tr>
<tr>
<td class="bitfield">xx100</td>
<td class="symbol">UInt(imm4[3:2])</td>
</tr>
<tr>
<td class="bitfield">x1000</td>
<td class="symbol">UInt(imm4[3])</td>
</tr>
</tbody>
</table>
Unspecified bits in "imm4" are ignored but should be set to zero by an assembler.</td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(idxdsize) = V{}(n); var result : bits(128); result = [V](shared_pseudocode.html#accessor_V_2){128}(d); result\[dst_index\*:esize\] = operand\[src_index\*:esize\]; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
