## BFMOPS (non-widening)

BFloat16 outer product, subtracting

This instruction works with a 16-bit element ZA tile.

This instruction generates an outer product of the first source vector and the second source vector. The first source is SVL<sub>H</sub>×1 vector and the second source is 1×SVL<sub>H</sub> vector.

Each source vector is independently predicated by a corresponding governing predicate. When either source vector element is Inactive the corresponding destination tile element remains unmodified.

The resulting outer product, SVL<sub>H</sub>×SVL<sub>H</sub>, is then destructively subtracted from the destination tile. This is equivalent to performing a single multiply-subtract from each of the destination tile elements.

This instruction follows SME2 ZA-targeting non-widening BFloat16 numerical behaviors.

ID_AA64SMFR0_EL1.B16B16 indicates whether this instruction is implemented.

### <span id="iclass_sme2"></span>SME2<span style="font-size:smaller;">\
(FEAT_SME_B16B16) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="3" class="lr">Pm</td>
<td colspan="3" class="lr">Pn</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">ZAda</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">S</td>
<td colspan="2"></td>
<td></td>
<td></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="bfmops_za_pp_zz_16"></span>

BFMOPS [\<ZAda\>](#ZAda__3 "For the "Half-precision" variant: is the name of the ZA tile ZA0-ZA1, encoded in the "ZAda" field.").H, [\<Pn\>](#Pn "Is the name of the first governing scalable predicate register P0-P7, encoded in the "Pn" field.")/M, [\<Pm\>](#Pm "Is the name of the second governing scalable predicate register P0-P7, encoded in the "Pm" field.")/M, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").H, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").H

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_B16B16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let a : integer = UInt(Pn); let b : integer = UInt(Pm); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(ZAda);

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

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let dim : integer{} = VL DIV 16; let mask1 : bits(PL) = P{}(a); let mask2 : bits(PL) = P{}(b); let op1 : bits(VL) = Z{}(n); let op2 : bits(VL) = Z{}(m); let op3 : bits(dim\*dim\*16) = ZAtile{}(da, 16); var result : bits(dim\*dim\*16); for row = 0 to dim-1 do for col = 0 to dim-1 do let elem2 : bits(16) = op2\[col\*:16\]; let elem3 : bits(16) = op3\[(row\*dim+col)\*:16\]; if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask1, row, 16) && [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask2, col, 16) then let elem1 : bits(16) = [BFNeg](shared_pseudocode.html#func_BFNeg_1)(op1\[row\*:16\]); result\[(row\*dim+col)\*:16\] = [BFMulAdd_ZA](shared_pseudocode.html#func_BFMulAdd_ZA_4)(elem3, elem1, elem2, FPCR()); else result\[(row\*dim+col)\*:16\] = elem3; end; end; end; [ZAtile](shared_pseudocode.html#accessor_ZAtile_3){dim\*dim\*16}(da, 16) = result;

</div>
