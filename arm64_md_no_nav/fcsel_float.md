## FCSEL

Floating-point conditional select (scalar)

This instruction allows the SIMD&FP destination register to take the value from either one or the other of two SIMD&FP source registers. If the condition passes, the first SIMD&FP source register value is taken, otherwise the second SIMD&FP source register value is taken.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="2" class="lr">ftype</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td colspan="4" class="lr">cond</td>
<td class="l">1</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">M</td>
<td></td>
<td class="droppedname">S</td>
<td colspan="5"></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Half-precision variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="FCSEL_H_floatsel"></span> Applies when <span class="bitdiff"> (ftype == 11)</span>

FCSEL [\<Hd\>](#Hd "Is the 16-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Hn\>](#Hn "For the "Half-precision" variant: is the 16-bit name of the first SIMD&FP source register, encoded in the "Rn" field."), [\<Hm\>](#Hm "Is the 16-bit name of the second SIMD&FP source register, encoded in the "Rm" field."), [\<cond\>](#cond_option "Is one of the standard conditions, encoded in the standard way, and ")

</div>

<div class="encoding">

#### Encoding for the Single-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FCSEL_S_floatsel"></span> Applies when <span class="bitdiff"> (ftype == 00)</span>

FCSEL [\<Sd\>](#Sd "Is the 32-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Sn\>](#Sn__3 "For the "Single-precision" variant: is the 32-bit name of the first SIMD&FP source register, encoded in the "Rn" field."), [\<Sm\>](#Sm "Is the 32-bit name of the second SIMD&FP source register, encoded in the "Rm" field."), [\<cond\>](#cond_option "Is one of the standard conditions, encoded in the standard way, and ")

</div>

<div class="encoding">

#### Encoding for the Double-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FCSEL_D_floatsel"></span> Applies when <span class="bitdiff"> (ftype == 01)</span>

FCSEL [\<Dd\>](#Dd "Is the 64-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Dn\>](#Dn__2 "For the "Double-precision" variant: is the 64-bit name of the first SIMD&FP source register, encoded in the "Rn" field."), [\<Dm\>](#Dm "Is the 64-bit name of the second SIMD&FP source register, encoded in the "Rm" field."), [\<cond\>](#cond_option "Is one of the standard conditions, encoded in the standard way, and ")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 8 \<\< UInt(ftype XOR '10'); let condition : bits(4) = cond;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Hd&gt;</td>
<td><span id="Hd"></span>
<p>Is the 16-bit name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Hn&gt;</td>
<td><span id="Hn"></span>
<p>Is the 16-bit name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Hm&gt;</td>
<td><span id="Hm"></span>
<p>Is the 16-bit name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;cond&gt;</td>
<td><span id="cond_option"></span>
<p>Is one of the standard conditions, encoded in the standard way, and encoded in “cond”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">cond</th>
<th class="symbol">&lt;cond&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0000</td>
<td class="symbol">EQ</td>
</tr>
<tr>
<td class="bitfield">0001</td>
<td class="symbol">NE</td>
</tr>
<tr>
<td class="bitfield">0010</td>
<td class="symbol">CS</td>
</tr>
<tr>
<td class="bitfield">0011</td>
<td class="symbol">CC</td>
</tr>
<tr>
<td class="bitfield">0100</td>
<td class="symbol">MI</td>
</tr>
<tr>
<td class="bitfield">0101</td>
<td class="symbol">PL</td>
</tr>
<tr>
<td class="bitfield">0110</td>
<td class="symbol">VS</td>
</tr>
<tr>
<td class="bitfield">0111</td>
<td class="symbol">VC</td>
</tr>
<tr>
<td class="bitfield">1000</td>
<td class="symbol">HI</td>
</tr>
<tr>
<td class="bitfield">1001</td>
<td class="symbol">LS</td>
</tr>
<tr>
<td class="bitfield">1010</td>
<td class="symbol">GE</td>
</tr>
<tr>
<td class="bitfield">1011</td>
<td class="symbol">LT</td>
</tr>
<tr>
<td class="bitfield">1100</td>
<td class="symbol">GT</td>
</tr>
<tr>
<td class="bitfield">1101</td>
<td class="symbol">LE</td>
</tr>
<tr>
<td class="bitfield">1110</td>
<td class="symbol">AL</td>
</tr>
<tr>
<td class="bitfield">1111</td>
<td class="symbol">NV</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sd&gt;</td>
<td><span id="Sd"></span>
<p>Is the 32-bit name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sn&gt;</td>
<td><span id="Sn__3"></span>
<p>Is the 32-bit name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sm&gt;</td>
<td><span id="Sm"></span>
<p>Is the 32-bit name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dd&gt;</td>
<td><span id="Dd"></span>
<p>Is the 64-bit name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dn&gt;</td>
<td><span id="Dn__2"></span>
<p>Is the 64-bit name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dm&gt;</td>
<td><span id="Dm"></span>
<p>Is the 64-bit name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); var result : bits(datasize); let condition_holds : boolean = [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition); result = if condition_holds then [V](shared_pseudocode.html#accessor_V_2){datasize}(n) else [V](shared_pseudocode.html#accessor_V_2){datasize}(m); [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
