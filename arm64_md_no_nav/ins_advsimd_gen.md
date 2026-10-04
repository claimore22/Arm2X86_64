## INS (general)

Insert vector element from general-purpose register

This instruction copies the contents of the source general-purpose register to the specified vector element in the destination SIMD&FP register.

This instruction can insert data into individual elements within a SIMD&FP register without clearing the remaining bits to zero.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

This instruction is used by the alias [MOV (from general)](mov_ins_advsimd_gen.html "Move general-purpose register to a vector element").

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
<td class="lr">0</td>
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
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
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
<td colspan="4" class="droppedname">imm4</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="INS_asimdins_IR_r"></span>

INS [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<Ts\>](#Ts_option "Is an element size specifier, ")\[[\<index\>](#imm5_index "Is the element index ")\], [\<R\>](#R_option__4 "Is the width specifier for the general-purpose source register, ")[\<n\>](#Rn_option__2 "Is the number [0-30] of the general-purpose source register or ZR (31), encoded in the "Rn" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'x0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = LowestSetBitNZ(imm5\[3:0\]); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let index : integer = UInt(imm5\[4:size+1\]); let esize : integer{} = 8 \<\< size;

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
<td>&lt;index&gt;</td>
<td><span id="imm5_index"></span>
<p>Is the element index encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;index&gt;</th>
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
<td>&lt;R&gt;</td>
<td><span id="R_option__4"></span>
<p>Is the width specifier for the general-purpose source register, encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;R&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">x0000</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">xxxx1</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">xxx10</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">xx100</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">x1000</td>
<td class="symbol">X</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;n&gt;</td>
<td><span id="Rn_option__2"></span>
<p>Is the number [0-30] of the general-purpose source register or ZR (31), encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let element : bits(esize) = X{}(n); var result : bits(128) = V{}(d); result\[index\*:esize\] = element; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
