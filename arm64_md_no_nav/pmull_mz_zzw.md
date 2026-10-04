## PMULL

Multi-vector polynomial multiply long

This instruction performs a polynomial multiplication over \[0, 1\] of the even-numbered elements of the first source vector by the corresponding elements of the second source vector, and places the results in the overlapping double-width elements of the first destination vector. The same operation is performed with odd-numbered elements of the source vectors, writing to the second destination vector. This instruction is unpredicated.

This instruction is legal when executed in Streaming SVE mode if both FEAT_SSVE_AES and FEAT_SVE_AES2 are implemented.

### <span id="iclass_sve2"></span>SVE2<span style="font-size:smaller;">\
(FEAT_SVE_AES2) </span>

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
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="4" class="lr">Zd</td>
<td class="lr">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="4"></td>
<td></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmull_mz_zzw_1x2"></span>

PMULL { [\<Zd1\>](#Zd1 "Is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 2.").Q-[\<Zd2\>](#Zd2 "Is the name of the second scalable vector register of the destination multi-vector group, encoded as "Zd" times 2 plus 1.").Q }, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").D, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE_AES2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd::'0');

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd1&gt;</td>
<td><span id="Zd1"></span>
<p>Is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 2.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zd2&gt;</td>
<td><span id="Zd2"></span>
<p>Is the name of the second scalable vector register of the destination multi-vector group, encoded as "Zd" times 2 plus 1.</p></td>
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

if IsFeatureImplemented(FEAT_SSVE_AES) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result_lo : bits(VL); var result_hi : bits(VL); for e = 0 to elements-1 do let element1_lo : bits(esize DIV 2) = operand1\[(2\*e + 0)\*:(esize DIV 2)\]; let element2_lo : bits(esize DIV 2) = operand2\[(2\*e + 0)\*:(esize DIV 2)\]; let element1_hi : bits(esize DIV 2) = operand1\[(2\*e + 1)\*:(esize DIV 2)\]; let element2_hi : bits(esize DIV 2) = operand2\[(2\*e + 1)\*:(esize DIV 2)\]; result_lo\[e\*:esize\] = [PolynomialMult](shared_pseudocode.html#func_PolynomialMult_4){esize DIV 2, esize DIV 2}(element1_lo, element2_lo); result_hi\[e\*:esize\] = [PolynomialMult](shared_pseudocode.html#func_PolynomialMult_4){esize DIV 2, esize DIV 2}(element1_hi, element2_hi); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d + 0) = result_lo; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d + 1) = result_hi;

</div>
