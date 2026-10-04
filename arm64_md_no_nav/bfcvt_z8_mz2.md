## BFCVT (BFloat16 to 8-bit floating-point)

Multi-vector BFloat16 convert to 8-bit floating-point

This instruction converts each BFloat16 element of the two source vectors to 8-bit floating-point while scaling the value by 2<sup>SInt(FPMR.NSCALE)</sup>, and places the results in the half-width elements of the destination vector. The 8-bit floating-point encoding format is selected by FPMR.F8D.

This instruction is unpredicated.

### <span id="iclass_sme2"></span>SME2<span style="font-size:smaller;">\
(FEAT_SME2 && FEAT_FP8) </span>

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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td></td>
<td class="droppedname">op</td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="6"></td>
<td colspan="4"></td>
<td></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="bfcvt_z8_mz2_"></span>

BFCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").B, { [\<Zn1\>](#Zn1__4 "For the "Two registers" variant: is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 2.").H-[\<Zn2\>](#Zn2__3 "Is the name of the second scalable vector register of the source multi-vector group, encoded as "Zn" times 2 plus 1.").H }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) \|\| !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn::'0'); let d : integer = UInt(Zd);

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
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__4"></span>
<p>Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 2.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn2&gt;</td>
<td><span id="Zn2__3"></span>
<p>Is the name of the second scalable vector register of the source multi-vector group, encoded as "Zn" times 2 plus 1.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; var result : bits(VL); let operand1 : bits(VL) = Z{}(n+0); let operand2 : bits(VL) = Z{}(n+1); for e = 0 to elements-1 do let element1 : bits(16) = operand1\[e\*:16\]; let element2 : bits(16) = operand2\[e\*:16\]; result\[(0\*elements + e)\*:8\] = [BFConvertFP8](shared_pseudocode.html#func_BFConvertFP8_3)(element1, FPCR(), FPMR()); result\[(1\*elements + e)\*:8\] = [BFConvertFP8](shared_pseudocode.html#func_BFConvertFP8_3)(element2, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>
