## BFMOPS (widening)

BFloat16 sum of outer products to single-precision, subtracting

This instruction works with a 32-bit element ZA tile.

This instruction multiplies the SVL<sub>S</sub>×2 sub-matrix of BFloat16 values held in the first source vector by the 2×SVL<sub>S</sub> sub-matrix of BFloat16 values in the second source vector.

Each source vector is independently predicated by a corresponding governing predicate. When a 16-bit source element is Inactive it is treated as having the value +0.0, but if both pairs of source vector elements that correspond to a 32-bit destination element contain Inactive elements, then the destination element remains unmodified.

The resulting SVL<sub>S</sub>×SVL<sub>S</sub> single-precision sum of outer products is then destructively subtracted from the single-precision destination tile. This is equivalent to performing a 2-way dot product and subtract from each of the destination tile elements.

Each 32-bit container of the first source vector holds 2 consecutive column elements of each row of a SVL<sub>S</sub>×2 sub-matrix. Similarly, each 32-bit container of the second source vector holds 2 consecutive row elements of each column of a 2×SVL<sub>S</sub> sub-matrix.

This instruction follows SME BFloat16 numerical behaviors.

### <span id="iclass_sme"></span>SME<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="3" class="lr">Pm</td>
<td colspan="3" class="lr">Pn</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="2" class="lr">ZAda</td>
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
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="bfmops_za32_pp_zz_"></span>

BFMOPS [\<ZAda\>](#ZAda "For the "Single-precision" variant: is the name of the ZA tile ZA0-ZA3, encoded in the "ZAda" field.").S, [\<Pn\>](#Pn "Is the name of the first governing scalable predicate register P0-P7, encoded in the "Pn" field.")/M, [\<Pm\>](#Pm "Is the name of the second governing scalable predicate register P0-P7, encoded in the "Pm" field.")/M, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").H, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").H

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let a : integer = UInt(Pn); let b : integer = UInt(Pm); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(ZAda);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;ZAda&gt;</td>
<td><span id="ZAda"></span>
<p>Is the name of the ZA tile ZA0-ZA3, encoded in the "ZAda" field.</p></td>
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

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let dim : integer{} = VL DIV 32; let mask1 : bits(PL) = P{}(a); let mask2 : bits(PL) = P{}(b); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(dim\*dim\*32) = ZAtile{}(da, 32); var result : bits(dim\*dim\*32); for row = 0 to dim-1 do for col = 0 to dim-1 do // determine row/col predicates let prow_0 : boolean = ([ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask1, 2\*row + 0, 16)); let prow_1 : boolean = ([ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask1, 2\*row + 1, 16)); let pcol_0 : boolean = ([ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask2, 2\*col + 0, 16)); let pcol_1 : boolean = ([ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask2, 2\*col + 1, 16)); var sum : bits(32) = operand3\[(row\*dim+col)\*:32\]; if (prow_0 && pcol_0) \|\| (prow_1 && pcol_1) then var erow_0 : bits(16) = (if prow_0 then operand1\[(2\*row + 0)\*:16\] else [FPZero](shared_pseudocode.html#func_FPZero_2){16}('0')); var erow_1 : bits(16) = (if prow_1 then operand1\[(2\*row + 1)\*:16\] else [FPZero](shared_pseudocode.html#func_FPZero_2){16}('0')); let ecol_0 : bits(16) = (if pcol_0 then operand2\[(2\*col + 0)\*:16\] else [FPZero](shared_pseudocode.html#func_FPZero_2){16}('0')); let ecol_1 : bits(16) = (if pcol_1 then operand2\[(2\*col + 1)\*:16\] else [FPZero](shared_pseudocode.html#func_FPZero_2){16}('0')); let honor_altfp : boolean = FALSE; // Alternate handling ignored if prow_0 then erow_0 = [BFNeg](shared_pseudocode.html#func_BFNeg_2)(erow_0, honor_altfp); end; if prow_1 then erow_1 = [BFNeg](shared_pseudocode.html#func_BFNeg_2)(erow_1, honor_altfp); end; sum = [BFDotAdd](shared_pseudocode.html#func_BFDotAdd_6)(sum, erow_0, erow_1, ecol_0, ecol_1, FPCR()); end; result\[(row\*dim+col)\*:32\] = sum; end; end; [ZAtile](shared_pseudocode.html#accessor_ZAtile_3){dim\*dim\*32}(da, 32) = result;

</div>
