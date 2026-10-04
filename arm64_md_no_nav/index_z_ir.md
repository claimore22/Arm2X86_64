## INDEX (immediate, scalar)

Create index starting from immediate and incremented by general-purpose register

This instruction populates the destination vector by setting the first element to the first signed immediate integer operand and monotonically incrementing the value by the second signed scalar integer operand for each subsequent element. The scalar source operand is a general-purpose register in which only the least significant bits corresponding to the vector element size are used and any remaining bits are ignored. This instruction is unpredicated.

### <span id="iclass_sve"></span>SVE<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">imm5</td>
<td colspan="5" class="lr">Zd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="index_z_ir_"></span>

INDEX [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__56 "Is the size specifier, "), \#[\<imm\>](#imm__43 "Is the signed immediate operand, in the range -16 to 15, encoded in the "imm5" field."), [\<R\>](#R__5 "Is a width specifier, ")[\<m\>](#m__3 "Is the number [0-30] of the source general-purpose register or the name ZR (31), encoded in the "Rm" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let m : integer = UInt(Rm); let d : integer = UInt(Zd); let imm : integer = SInt(imm5);

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
<td><span id="T__56"></span>
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
<td class="bitfield">00</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">11</td>
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
<td>&lt;imm&gt;</td>
<td><span id="imm__43"></span>
<p>Is the signed immediate operand, in the range -16 to 15, encoded in the "imm5" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;R&gt;</td>
<td><span id="R__5"></span>
<p>Is a width specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;R&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">X</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;m&gt;</td>
<td><span id="m__3"></span>
<p>Is the number [0-30] of the source general-purpose register or the name ZR (31), encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand2 : bits(esize) = X{}(m); let element2 : integer = SInt(operand2); var result : bits(VL); for e = 0 to elements-1 do let index : integer = imm + e \* element2; result\[e\*:esize\] = index\[esize-1:0\]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>
