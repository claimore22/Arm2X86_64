## EXT

Extract vector from pair of vectors

This instruction extracts the lowest vector elements from the second source SIMD&FP register and the highest vector elements from the first source SIMD&FP register, concatenates the results into a vector, and writes the vector to the destination SIMD&FP register vector. The index value specifies the lowest vector element to extract from the first source register, and consecutive elements are extracted from the first, then second, source registers until the destination vector is filled.

The following figure shows an example of the operation of EXT doubleword operation for Q = 0 and imm4\<2:0\> = 3.\
![EXT doubleword operation for Q = 0 and imm4\<2:0\> = 3](A64.ext_doubleword_operation_for_imm3.svg)

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

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
<td class="lr">Q</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">0</td>
<td colspan="4" class="lr">imm4</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">op2</td>
<td></td>
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

<span id="EXT_asimdext_only"></span>

EXT [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__7 "Is an arrangement specifier, "), [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").[\<T\>](#T_option__7 "Is an arrangement specifier, "), [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").[\<T\>](#T_option__7 "Is an arrangement specifier, "), \#[\<index\>](#imm420 "Is the lowest numbered byte element to be extracted, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Q == '0' && imm4\[3\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 64 \<\< UInt(Q); let position : integer{} = 8 \* UInt(imm4);

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
<td>&lt;T&gt;</td>
<td><span id="T_option__7"></span>
<p>Is an arrangement specifier, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">8B</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">16B</td>
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
<td><span id="Vn__2"></span>
<p>Is the name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vm&gt;</td>
<td><span id="Vm"></span>
<p>Is the name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index&gt;</td>
<td><span id="imm420"></span>
<p>Is the lowest numbered byte element to be extracted, encoded in “(Q :: imm4)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="bitfield">imm4[3]</th>
<th class="symbol">&lt;index&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="bitfield">0</td>
<td class="symbol">UInt(imm4[2:0])</td>
</tr>
<tr>
<td class="bitfield">0</td>
<td class="bitfield">1</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="bitfield">x</td>
<td class="symbol">UInt(imm4)</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let hi : bits(datasize) = V{}(m); let lo : bits(datasize) = V{}(n); let concat : bits(datasize\*2) = hi :: lo; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = concat\[(position+datasize)-1:position\];

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
