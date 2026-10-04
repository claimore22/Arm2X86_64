## SM4E

SM4 encryption and decryption

This instruction reads 16 bytes of input data from each 128-bit segment of the first source vector, together with four iterations of 32-bit round keys from the corresponding 128-bit segments of the second source vector. Each block of data is encrypted by four rounds in accordance with the SM4 standard, and destructively placed in the corresponding segments of the first source vector. This instruction is unpredicated.

ID_AA64ZFR0_EL1.SM4 indicates whether this instruction is implemented.

This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

### <span id="iclass_sve2"></span>SVE2<span style="font-size:smaller;">\
(FEAT_SVE_SM4) </span>

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
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="5" class="lr">Zdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">op</td>
<td colspan="3"></td>
<td colspan="2"></td>
<td class="droppedname">o2</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="sm4e_z_zz_"></span>

SM4E [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").S, [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").S, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").S

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE_SM4) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let m : integer = UInt(Zm); let dn : integer = UInt(Zdn);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zdn&gt;</td>
<td><span id="Zdn"></span>
<p>Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.</p></td>
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

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for s = 0 to segments-1 do let key : bits(128) = operand2\[s\*:128\]; var intval : bits(32); var roundresult : bits(128) = operand1\[s\*:128\]; var roundkey : bits(32); for index = 0 to 3 do roundkey = key\[index\*:32\]; intval = roundresult\[127:96\] XOR roundresult\[95:64\] XOR roundresult\[63:32\] XOR roundkey; for i = 0 to 3 do intval\[i\*:8\] = [Sbox](shared_pseudocode.html#func_Sbox_1)(intval\[i\*:8\]); end; intval = (intval XOR ROL(intval, 2) XOR ROL(intval, 10) XOR ROL(intval, 18) XOR ROL(intval, 24)); intval = intval XOR roundresult\[31:0\]; roundresult\[31:0\] = roundresult\[63:32\]; roundresult\[63:32\] = roundresult\[95:64\]; roundresult\[95:64\] = roundresult\[127:96\]; roundresult\[127:96\] = intval; end; result\[s\*:128\] = roundresult; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
