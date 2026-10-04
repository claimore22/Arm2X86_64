## SQCVTU (two registers)

Multi-vector signed 32-bit integer saturating extract narrow to unsigned 16-bit integer

This instruction saturates the signed 32-bit integer value in each element of the two source vectors to unsigned integer value that is half the original source element width, and places the 16-bit results in the half-width destination elements.

This instruction is unpredicated.

### <span id="iclass_sme2"></span>SME2<span style="font-size:smaller;">\
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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">1</td>
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
<td class="droppedname">U</td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="sqcvtu_z_mz2_"></span>

SQCVTU [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").H, { [\<Zn1\>](#Zn1__4 "For the "Two registers" variant: is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 2.").S-[\<Zn2\>](#Zn2__3 "Is the name of the second scalable vector register of the source multi-vector group, encoded as "Zn" times 2 plus 1.").S }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16; let n : integer = UInt(Zn::'0'); let d : integer = UInt(Zd);

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

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (2 \* esize); var result : bits(VL); for r = 0 to 1 do let operand : bits(VL) = Z{}(n+r); for e = 0 to elements-1 do let element : integer = SInt(operand\[e\*:(2 \* esize)\]); result\[(r\*elements + e)\*:esize\] = [UnsignedSat](shared_pseudocode.html#func_UnsignedSat_2){esize}(element); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>
