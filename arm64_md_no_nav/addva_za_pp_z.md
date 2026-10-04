## ADDVA

Add vertically vector elements to ZA tile

This instruction adds each element of the source vector to the corresponding Active element of each vertical slice of a ZA tile. The tile elements are predicated by a pair of governing predicates. An element of a vertical slice is considered active if its corresponding element in the first governing predicate is TRUE and the element corresponding to its vertical slice number in the second governing predicate is TRUE. Inactive elements in the destination tile remain unmodified.

ID_AA64SMFR0_EL1.I16I64 indicates whether the 64-bit integer variant is implemented.

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
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
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
<td></td>
<td></td>
<td class="droppedname">op</td>
<td colspan="3"></td>
<td colspan="2"></td>
<td class="droppedname">V</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td></td>
<td></td>
<td></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="addva_za_pp_z_32"></span>

ADDVA [\<ZAda\>](#ZAda "For the "32-bit" variant: is the name of the ZA tile ZA0-ZA3, encoded in the "ZAda" field.").S, [\<Pn\>](#Pn "Is the name of the first governing scalable predicate register P0-P7, encoded in the "Pn" field.")/M, [\<Pm\>](#Pm "Is the name of the second governing scalable predicate register P0-P7, encoded in the "Pm" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").S

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let a : integer = UInt(Pn); let b : integer = UInt(Pm); let n : integer = UInt(Zn); let da : integer = UInt(ZAda);

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
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="3" class="lr">Pm</td>
<td colspan="3" class="lr">Pn</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">ZAda</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td></td>
<td class="droppedname">op</td>
<td colspan="3"></td>
<td colspan="2"></td>
<td class="droppedname">V</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td></td>
<td></td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="addva_za_pp_z_64"></span>

ADDVA [\<ZAda\>](#ZAda__2 "For the "64-bit" variant: is the name of the ZA tile ZA0-ZA7, encoded in the "ZAda" field.").D, [\<Pn\>](#Pn "Is the name of the first governing scalable predicate register P0-P7, encoded in the "Pn" field.")/M, [\<Pm\>](#Pm "Is the name of the second governing scalable predicate register P0-P7, encoded in the "Pm" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_I16I64) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let a : integer = UInt(Pn); let b : integer = UInt(Pm); let n : integer = UInt(Zn); let da : integer = UInt(ZAda);

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

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let dim : integer{} = VL DIV esize; let mask1 : bits(PL) = P{}(a); let mask2 : bits(PL) = P{}(b); let operand_src : bits(VL) = Z{}(n); let operand_acc : bits(dim\*dim\*esize) = ZAtile{}(da, esize); var result : bits(dim\*dim\*esize); for row = 0 to dim-1 do let element : bits(esize) = operand_src\[row\*:esize\]; for col = 0 to dim-1 do var res : bits(esize) = operand_acc\[(row\*dim+col)\*:esize\]; if ([ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask1, row, esize) && [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask2, col, esize)) then res = res + element; end; result\[(row\*dim+col)\*:esize\] = res; end; end; [ZAtile](shared_pseudocode.html#accessor_ZAtile_3){dim\*dim\*esize}(da, esize) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
