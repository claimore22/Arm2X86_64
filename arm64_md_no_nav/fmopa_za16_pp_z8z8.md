## FMOPA (widening, 2-way, FP8 to FP16)

8-bit floating-point sum of outer products to half-precision, accumulating

This instruction works with a 16-bit element ZA tile.

This instruction widens the SVL<sub>H</sub>×2 sub-matrix of 8-bit floating-point values held in the first source vector to half-precision values and multiplies it by the widened 2×SVL<sub>H</sub> sub-matrix of 8-bit floating-point values in the second source vector to half-precision values.

Each source vector is independently predicated by a corresponding governing predicate. When a 8-bit source element is Inactive it is treated as having the value +0.0, but if both groups of source vector elements that correspond to a 16-bit destination element contain Inactive elements, then the destination element remains unmodified.

The resulting SVL<sub>H</sub>×SVL<sub>H</sub> half-precision sum of outer products are scaled by 2<sup>-UInt(FPMR.LSCALE\[3:0\])</sup>, before being destructively added to the half-precision destination tile. This is equivalent to performing a downscaled 2-way dot product and accumulate to each of the destination tile elements.

Each 16-bit container of the first source vector holds 2 consecutive column elements of each row of a SVL<sub>H</sub>×2 sub-matrix. Similarly, each 16-bit container of the second source vector holds 2 consecutive row elements of each column of a 2×SVL<sub>H</sub> sub-matrix.

The 8-bit floating-point encoding format for the elements of the first source vector and the second source vector is selected by FPMR.F8S1 and FPMR.F8S2 respectively.

### <span id="iclass_sme2"></span>SME2<span style="font-size:smaller;">\
(FEAT_SME_F8F16) </span>

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
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="3" class="lr">Pm</td>
<td colspan="3" class="lr">Pn</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">ZAda</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmopa_za16_pp_z8z8_8"></span>

FMOPA [\<ZAda\>](#ZAda__3 "For the "Half-precision" variant: is the name of the ZA tile ZA0-ZA1, encoded in the "ZAda" field.").H, [\<Pn\>](#Pn "Is the name of the first governing scalable predicate register P0-P7, encoded in the "Pn" field.")/M, [\<Pm\>](#Pm "Is the name of the second governing scalable predicate register P0-P7, encoded in the "Pm" field.")/M, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").B, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_F8F16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let a : integer = UInt(Pn); let b : integer = UInt(Pm); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(ZAda);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;ZAda&gt;</td>
<td><span id="ZAda__3"></span>
<p>Is the name of the ZA tile ZA0-ZA1, encoded in the "ZAda" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Pn&gt;</td>
<td><span id="Pn"></span>
<p>Is the name of the first governing scalable predicate register P0-P7, encoded in the "Pn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Pm&gt;</td>
<td><span id="Pm"></span>
<p>Is the name of the second governing scalable predicate register P0-P7, encoded in the "Pm" field.</p></td>
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

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let dim : integer{} = VL DIV 16; let mask1 : bits(PL) = P{}(a); let mask2 : bits(PL) = P{}(b); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(dim\*dim\*16) = ZAtile{}(da, 16); var result : bits(dim\*dim\*16); for row = 0 to dim-1 do for col = 0 to dim-1 do var prow : array \[\[2\]\] of boolean; var pcol : array \[\[2\]\] of boolean; var any_active : boolean = FALSE; for i = 0 to 1 do prow\[\[i\]\] = [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask1, 2\*row + i, 8); pcol\[\[i\]\] = [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask2, 2\*col + i, 8); any_active = any_active \|\| (prow\[\[i\]\] && pcol\[\[i\]\]); end; if any_active then var sum : bits(16) = operand3\[(row\*dim+col)\*:16\]; var rowop : bits(16) = Zeros{}; var colop : bits(16) = Zeros{}; for i = 0 to 1 do if prow\[\[i\]\] then rowop\[i\*:8\] = operand1\[(2\*row + i)\*:8\]; end; if pcol\[\[i\]\] then colop\[i\*:8\] = operand2\[(2\*col + i)\*:8\]; end; end; sum = [FP8DotAddFP](shared_pseudocode.html#func_FP8DotAddFP_7){16, 16}(sum, rowop, colop, FPCR(), FPMR()); result\[(row\*dim+col)\*:16\] = sum; else result\[(row\*dim+col)\*:16\] = operand3\[(row\*dim+col)\*:16\]; end; end; end; [ZAtile](shared_pseudocode.html#accessor_ZAtile_3){dim\*dim\*16}(da, 16) = result;

</div>
