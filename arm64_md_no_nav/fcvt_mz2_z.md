## FCVT (widening)

Multi-vector half-precision convert to single-precision

This instruction converts each element of the source vector from half-precision to single-precision floating-point, and places the results in the double-width destination elements of the destination vectors.

This instruction follows SME2 floating-point numerical behaviors corresponding to instructions that place their results in one or more SVE Z vectors.

This instruction is unpredicated.

ID_AA64SMFR0_EL1.F16F16 indicates whether this instruction is implemented.

### <span id="iclass_sme2"></span>SME2<span style="font-size:smaller;">\
(FEAT_SME_F16F16) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="4" class="lr">Zd</td>
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
<td colspan="5"></td>
<td colspan="4"></td>
<td class="droppedname">L</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_mz2_z_"></span>

FCVT { [\<Zd1\>](#Zd1 "For the "Two registers" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 2.").S-[\<Zd2\>](#Zd2 "For the "Consecutive" variant: is the name of the second scalable vector register of the destination multi-vector group, encoded as "Zd" times 2 plus 1.").S }, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").H

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_F16F16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Zd::'0');

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
<td><span id="Zn"></span>
<p>Is the name of the source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; let operand : bits(VL) = Z{}(n); var result : bits(2\*VL); for e = 0 to elements-1 do let element : bits(16) = operand\[e\*:16\]; let res : bits(32) = [FPConvertSVE](shared_pseudocode.html#func_FPConvertSVE_4){32, 16}(element, FPCR()); result\[e\*:32\] = res; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+0) = result\[VL-1:0\]; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+1) = result\[2\*VL-1:VL\];

</div>
