## UZP (four registers)

Concatenate elements from four vectors

This instruction concatenates every fourth element from each of the four source vectors and places them in the corresponding elements of the four destination vectors.

This instruction is unpredicated.

It has encodings from 2 classes: [8-bit to 64-bit elements](#iclass_8_bit_to_64_bit_elements) and [128-bit element](#iclass_128_bit_element)

### <span id="iclass_8_bit_to_64_bit_elements"></span>8-bit to 64-bit elements<span style="font-size:smaller;">\
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
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="3" class="lr">Zn</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="3" class="lr">Zd</td>
<td class="lr">1</td>
<td class="lr">0</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="6"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td class="droppedname">op</td>
<td></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="uzp_mz_z_4"></span>

UZP { [\<Zd1\>](#Zd1__2 "For the "Four registers" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 4.").[\<T\>](#T__102 "Is the size specifier, ")-[\<Zd4\>](#Zd4 "For the "Consecutive" variant: is the name of the fourth scalable vector register of the destination multi-vector group, encoded as "Zd" times 4 plus 3.").[\<T\>](#T__102 "Is the size specifier, ") }, { [\<Zn1\>](#Zn1__6 "For the "Four registers" variant: is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 4.").[\<T\>](#T__102 "Is the size specifier, ")-[\<Zn4\>](#Zn4__3 "Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zn" times 4 plus 3.").[\<T\>](#T__102 "Is the size specifier, ") }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' && [MaxImplementedSVL](shared_pseudocode.html#func_MaxImplementedSVL_0)() \< 256 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let n : integer = UInt(Zn::'00'); let d : integer = UInt(Zd::'00');

### <span id="iclass_128_bit_element"></span>128-bit element<span style="font-size:smaller;">\
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
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="3" class="lr">Zn</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="3" class="lr">Zd</td>
<td class="lr">1</td>
<td class="lr">0</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="6"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td class="droppedname">op</td>
<td></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="uzp_mz_z_4q"></span>

UZP { [\<Zd1\>](#Zd1__2 "For the "Four registers" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 4.").Q-[\<Zd4\>](#Zd4 "For the "Consecutive" variant: is the name of the fourth scalable vector register of the destination multi-vector group, encoded as "Zd" times 4 plus 3.").Q }, { [\<Zn1\>](#Zn1__6 "For the "Four registers" variant: is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 4.").Q-[\<Zn4\>](#Zn4__3 "Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zn" times 4 plus 3.").Q }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if [MaxImplementedSVL](shared_pseudocode.html#func_MaxImplementedSVL_0)() \< 512 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn::'00'); let d : integer = UInt(Zd::'00');

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd1&gt;</td>
<td><span id="Zd1__2"></span>
<p>Is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__102"></span>
<p>Is the size specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">11</td>
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
<td>&lt;Zd4&gt;</td>
<td><span id="Zd4"></span>
<p>Is the name of the fourth scalable vector register of the destination multi-vector group, encoded as "Zd" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__6"></span>
<p>Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn4&gt;</td>
<td><span id="Zn4__3"></span>
<p>Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zn" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if VL \< esize \* 4 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let quads : integer = VL DIV (esize \* 4); var result0 : bits(VL); var result1 : bits(VL); var result2 : bits(VL); var result3 : bits(VL); for r = 0 to 3 do let operand : bits(VL) = Z{}(n+r); let base : integer = r \* quads; for q = 0 to quads-1 do result0\[(base+q)\*:esize\] = operand\[(4\*q+0)\*:esize\]; result1\[(base+q)\*:esize\] = operand\[(4\*q+1)\*:esize\]; result2\[(base+q)\*:esize\] = operand\[(4\*q+2)\*:esize\]; result3\[(base+q)\*:esize\] = operand\[(4\*q+3)\*:esize\]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+0) = result0; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+1) = result1; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+2) = result2; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+3) = result3;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
