## SHA1SU1

SHA1 schedule update 1

SHA1 schedule update 1.

### <span id="iclass_advanced_simd"></span>Advanced SIMD<span style="font-size:smaller;">\
(FEAT_SHA1) </span>

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
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="4"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="5"></td>
<td colspan="5" class="droppedname">opcode</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SHA1SU1_VV_cryptosha2"></span>

SHA1SU1 [\<Vd\>](#Vd__4 "Is the name of the SIMD&FP source and destination register, encoded in the "Rd" field.").4S, [\<Vn\>](#Vn__6 "Is the name of the second SIMD&FP source register, encoded in the "Rn" field.").4S

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SHA1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Vd&gt;</td>
<td><span id="Vd__4"></span>
<p>Is the name of the SIMD&amp;FP source and destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vn&gt;</td>
<td><span id="Vn__6"></span>
<p>Is the name of the second SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(d); let operand2 : bits(128) = V{}(n); let T : bits(128) = operand1 XOR LSR(operand2, 32); var result : bits(128); result\[31:0\] = ROL(T\[31:0\], 1); result\[63:32\] = ROL(T\[63:32\], 1); result\[95:64\] = ROL(T\[95:64\], 1); result\[127:96\] = ROL(T\[127:96\], 1) XOR ROL(T\[31:0\], 2); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
