## PMULLT

Polynomial multiply long (top)

This instruction performs a polynomial multiplication over \[0, 1\] of the odd-numbered elements of the first source vector by the corresponding elements of the second source vector, and places the results in the overlapping double-width elements of the destination vector. This instruction is unpredicated.

ID_AA64ZFR0_EL1.AES indicates whether the 128-bit element variant is implemented. The 128-bit element variant is legal when executed in Streaming SVE mode if one of the following is true:

- Both FEAT_SSVE_AES and FEAT_SVE_PMULL128 are implemented.
- FEAT_SME_FA64 is implemented and enabled.

It has encodings from 2 classes: [16-bit or 64-bit elements](#iclass_16_bit_or_64_bit_elements) and [128-bit element](#iclass_128_bit_element)

### <span id="iclass_16_bit_or_64_bit_elements"></span>16-bit or 64-bit elements<span style="font-size:smaller;">\
(FEAT_SVE2 \|\| FEAT_SME) </span>

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
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">!= 00</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zm</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="5"></td>
<td></td>
<td colspan="2"></td>
<td class="droppedname">op</td>
<td class="droppedname">U</td>
<td class="droppedname">T</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmullt_z_zz_"></span>

PMULLT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__68 "Is the size specifier, "), [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").[\<Tb\>](#Tb__6 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<Tb\>](#Tb__6 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size\[0\] == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### <span id="iclass_128_bit_element"></span>128-bit element<span style="font-size:smaller;">\
(FEAT_SVE_PMULL128) </span>

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
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zm</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="5"></td>
<td></td>
<td colspan="2"></td>
<td class="droppedname">op</td>
<td class="droppedname">U</td>
<td class="droppedname">T</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmullt_z_zz_q"></span>

PMULLT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").Q, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").D, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE_PMULL128) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd&gt;</td>
<td><span id="Zd"></span>
<p>Is the name of the destination scalable vector register, encoded in the "Zd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__68"></span>
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
<td class="bitfield">01</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">1x</td>
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
<td>&lt;Zn&gt;</td>
<td><span id="Zn__2"></span>
<p>Is the name of the first source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Tb&gt;</td>
<td><span id="Tb__6"></span>
<p>Is the size specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;Tb&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">01</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">1x</td>
<td class="symbol">S</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm&gt;</td>
<td><span id="Zm"></span>
<p>Is the name of the second source scalable vector register, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if esize == 128 then if IsFeatureImplemented(FEAT_SSVE_AES) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; else [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(esize DIV 2) = operand1\[(2\*e + 1)\*:(esize DIV 2)\]; let element2 : bits(esize DIV 2) = operand2\[(2\*e + 1)\*:(esize DIV 2)\]; result\[e\*:esize\] = [PolynomialMult](shared_pseudocode.html#func_PolynomialMult_4){esize DIV 2, esize DIV 2}(element1, element2); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
