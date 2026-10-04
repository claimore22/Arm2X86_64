## LUTI2

Lookup table read with 2-bit indices

This instruction copies indexed 8-bit or 16-bit elements from the table vector to the destination vector using packed 2-bit indices from a segment of the source vector. A segment corresponds to a portion of the source vector that is consumed in order to fill the destination vector. The segment is selected by the vector segment index.

### <span id="iclass_advanced_simd"></span>Advanced SIMD<span style="font-size:smaller;">\
(FEAT_AdvSIMD && FEAT_LUT) </span>

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
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">x</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">0</td>
<td colspan="2" class="lr">len</td>
<td class="lr">op</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td class="droppedname">Q</td>
<td colspan="6"></td>
<td colspan="2" class="droppedname">op2</td>
<td></td>
<td colspan="5"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Byte variant

<span id="LUTI2_asimdtbl_L5"></span> Applies when <span class="bitdiff"> (op2 == 10 && op == 1)</span>

LUTI2 [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").16B, { [\<Vn\>](#Vn__3 "For the "Single register table" variant: is the name of the SIMD&FP table register, encoded in the "Rn" field.").16B }, [\<Vm\>](#Vm__4 "Is the name of the SIMD&FP source register, encoded in the "Rm" field.")\[[\<index\>](#index__9 "For the "Byte" variant: is the vector segment index, in the range 0 to 3, encoded in the "len" field.")\]

</div>

<div class="encoding">

#### Encoding for the Halfword variant

<span id="LUTI2_asimdtbl_L6"></span> Applies when <span class="bitdiff"> (op2 == 11)</span>

LUTI2 [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").8H, { [\<Vn\>](#Vn__3 "For the "Single register table" variant: is the name of the SIMD&FP table register, encoded in the "Rn" field.").8H }, [\<Vm\>](#Vm__4 "Is the name of the SIMD&FP source register, encoded in the "Rm" field.")\[[\<index\>](#len_op "For the "Halfword" variant: is the vector segment index, in the range 0 to 7, encoded in the "len:op" fields.")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) \|\| !IsFeatureImplemented(FEAT_LUT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if op2 == '10' && op == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let isize : integer{} = 2; let esize : integer{} = if op2 == '10' then 8 else 16; let part : integer = if op2 == '10' then UInt(len) else UInt(len::op);

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
<td>&lt;Vn&gt;</td>
<td><span id="Vn__3"></span>
<p>Is the name of the SIMD&amp;FP table register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vm&gt;</td>
<td><span id="Vm__4"></span>
<p>Is the name of the SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index&gt;</td>
<td><span id="index__9"></span>
<p>For the "Byte" variant: is the vector segment index, in the range 0 to 3, encoded in the "len" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="len_op"></span>
<p>For the "Halfword" variant: is the vector segment index, in the range 0 to 7, encoded in the "len:op" fields.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let elements : integer = 128 DIV esize; let ibase : integer = elements \* part; let indices : bits(128) = V{}(m); let table : bits(128) = V{}(n); var result : bits(128); for e = 0 to elements-1 do let index : integer = UInt(indices\[(ibase + e)\*:isize\]); result\[e\*:esize\] = table\[index\*:esize\]; end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
