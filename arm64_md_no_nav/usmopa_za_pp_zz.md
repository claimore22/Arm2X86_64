## USMOPA (4-way)

Unsigned by signed integer sum of outer products, accumulating

The 8-bit integer variant works with a 32-bit element ZA tile.

The 16-bit integer variant works with a 64-bit element ZA tile.

This instruction multiplies the sub-matrix in the first source vector by the sub-matrix in the second source vector. In case of the 8-bit integer variant, the first source holds SVL<sub>S</sub>×4 sub-matrix of unsigned 8-bit integer values, and the second source holds 4×SVL<sub>S</sub> sub-matrix of signed 8-bit integer values. In case of the 16-bit integer variant, the first source holds SVL<sub>D</sub>×4 sub-matrix of unsigned 16-bit integer values, and the second source holds 4×SVL<sub>D</sub> sub-matrix of signed 16-bit integer values.

Each source vector is independently predicated by a corresponding governing predicate. When an 8-bit source element in case of 8-bit integer variant or a 16-bit source element in case of 16-bit integer variant is Inactive, it is treated as having the value 0.

The resulting SVL<sub>S</sub>×SVL<sub>S</sub> widened 32-bit integer or SVL<sub>D</sub>×SVL<sub>D</sub> widened 64-bit integer sum of outer products is then destructively added to the 32-bit integer or 64-bit integer destination tile, respectively for 8-bit integer and 16-bit integer instruction variants. This is equivalent to performing a 4-way dot product and accumulate to each of the destination tile elements.

In case of the 8-bit integer variant, each 32-bit container of the first source vector holds 4 consecutive column elements of each row of a SVL<sub>S</sub>×4 sub-matrix, and each 32-bit container of the second source vector holds 4 consecutive row elements of each column of a 4×SVL<sub>S</sub> sub-matrix. In case of the 16-bit integer variant, each 64-bit container of the first source vector holds 4 consecutive column elements of each row of a SVL<sub>D</sub>×4 sub-matrix, and each 64-bit container of the second source vector holds 4 consecutive row elements of each column of a 4×SVL<sub>D</sub> sub-matrix.

ID_AA64SMFR0_EL1.I16I64 indicates whether the 16-bit integer variant is implemented.

It has encodings from 2 classes: [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

### <span id="iclass_32_bit"></span>32-bit<span style="font-size:smaller;">\
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
<td class="r">1</td>
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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">ZAda</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td class="droppedname">u0</td>
<td colspan="2"></td>
<td class="droppedname">u1</td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">S</td>
<td></td>
<td></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="usmopa_za_pp_zz_32"></span>

USMOPA [\<ZAda\>](#ZAda "For the "32-bit" variant: is the name of the ZA tile ZA0-ZA3, encoded in the "ZAda" field.").S, [\<Pn\>](#Pn "Is the name of the first governing scalable predicate register P0-P7, encoded in the "Pn" field.")/M, [\<Pm\>](#Pm "Is the name of the second governing scalable predicate register P0-P7, encoded in the "Pm" field.")/M, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").B, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let a : integer = UInt(Pn); let b : integer = UInt(Pm); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(ZAda); let op1_unsigned : boolean = TRUE; let op2_unsigned : boolean = FALSE;

### <span id="iclass_64_bit"></span>64-bit<span style="font-size:smaller;">\
(FEAT_SME_I16I64) </span>

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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="3" class="lr">Pm</td>
<td colspan="3" class="lr">Pn</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">ZAda</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td></td>
<td colspan="4"></td>
<td class="droppedname">u0</td>
<td colspan="2"></td>
<td class="droppedname">u1</td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">S</td>
<td></td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="usmopa_za_pp_zz_64"></span>

USMOPA [\<ZAda\>](#ZAda__2 "For the "64-bit" variant: is the name of the ZA tile ZA0-ZA7, encoded in the "ZAda" field.").D, [\<Pn\>](#Pn "Is the name of the first governing scalable predicate register P0-P7, encoded in the "Pn" field.")/M, [\<Pm\>](#Pm "Is the name of the second governing scalable predicate register P0-P7, encoded in the "Pm" field.")/M, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").H, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").H

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_I16I64) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let a : integer = UInt(Pn); let b : integer = UInt(Pm); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(ZAda); let op1_unsigned : boolean = TRUE; let op2_unsigned : boolean = FALSE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;ZAda&gt;</td>
<td><span id="ZAda"></span>
<p>For the "32-bit" variant: is the name of the ZA tile ZA0-ZA3, encoded in the "ZAda" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="ZAda__2"></span>
<p>For the "64-bit" variant: is the name of the ZA tile ZA0-ZA7, encoded in the "ZAda" field.</p></td>
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

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let dim : integer{} = VL DIV esize; let mask1 : bits(PL) = P{}(a); let mask2 : bits(PL) = P{}(b); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(dim\*dim\*esize) = ZAtile{}(da, esize); var result : bits(dim\*dim\*esize); var prod : integer; for row = 0 to dim-1 do for col = 0 to dim-1 do var sum : bits(esize) = operand3\[(row\*dim+col)\*:esize\]; for k = 0 to 3 do if ([ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask1, 4\*row + k, esize DIV 4) && [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask2, 4\*col + k, esize DIV 4)) then let op1elt : bits(esize DIV 4) = operand1\[(4\*row + k)\*:(esize DIV 4)\]; let op2elt : bits(esize DIV 4) = operand2\[(4\*col + k)\*:(esize DIV 4)\]; let element1 : integer = if op1_unsigned then UInt(op1elt) else SInt(op1elt); let element2 : integer = if op2_unsigned then UInt(op2elt) else SInt(op2elt); prod = element1 \* element2; sum = sum + prod; end; end; result\[(row\*dim+col)\*:esize\] = sum; end; end; [ZAtile](shared_pseudocode.html#accessor_ZAtile_3){dim\*dim\*esize}(da, esize) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
