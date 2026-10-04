## ST64BV0

Single-copy atomic 64-byte EL0 store with status result

This instruction stores eight 64-bit doublewords from consecutive registers to a memory location, with the bottom 32 bits taken from ACCDATA_EL1, and writes the status result of the store to a register. The store starts at register Xt, with the data being formed as Data\[511:0\] = X(t+7)::X(t+6)::X(t+5)::X(t+4)::X(t+3)::X(t+2)::X(t+1)::Xt\[63:32\]::ACCDATA_EL1()\[31:0\]. The data is stored atomically and is required to be 64-byte aligned.

It is <span class="arm-defined-word">IMPLEMENTATION DEFINED</span> which memory locations support this instruction. A memory location that supports <span class="asm-code">ST64BV0</span> also supports <span class="asm-code">ST64BV</span>.

For more information, including about the memory types accessible and how the accesses are performed, see <span class="armarm-xref" data-linkend="ARMARM_CJACAFAH">Single-copy atomic 64-byte load/store</span>.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_LS64_ACCDATA) </span>

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
<td class="l">1</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rs</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="2"></td>
<td></td>
<td class="droppedname">VR</td>
<td></td>
<td></td>
<td class="droppedname">A</td>
<td class="droppedname">R</td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">o3</td>
<td colspan="3" class="droppedname">opc</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ST64BV0_64_memop"></span>

ST64BV0 [\<Xs\>](#XsOrXZR__3 "Is the 64-bit name of the general-purpose register into which the status result of this instruction is written, encoded in the "Rs" field.  The value returned is:   0xFFFFFFFF_FFFFFFFF : If the memory location accessed does not support this instruction. In this case, the value at the memory location is UNKNOWN.  != 0xFFFFFFFF_FFFFFFFF : If the memory location accessed does support this instruction. In this case, the peripheral that provides the response defines the returned value and provides information on the state of the memory update at the memory location.   If XZR is used, then the return value is ignored."), [\<Xt\>](#XtOrXZR__9 "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_LS64_ACCDATA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Rt\[4:3\] == '11' \|\| Rt\[0\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let withstatus : boolean = TRUE; let s : integer{} = UInt(Rs); let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let tagchecked : boolean = n != 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xs&gt;</td>
<td><span id="XsOrXZR__3"></span>
<p>Is the 64-bit name of the general-purpose register into which the status result of this instruction is written, encoded in the "Rs" field.</p>
<p>The value returned is:</p>
<dl>
<dt>0xFFFFFFFF_FFFFFFFF</dt>
<dd>
If the memory location accessed does not support this instruction. In this case, the value at the memory location is <span class="arm-defined-word">UNKNOWN</span>.
</dd>
<dt>!= 0xFFFFFFFF_FFFFFFFF</dt>
<dd>
If the memory location accessed does support this instruction. In this case, the peripheral that provides the response defines the returned value and provides information on the state of the memory update at the memory location.
</dd>
</dl>
<p>If XZR is used, then the return value is ignored.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__9"></span>
<p>Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option"></span>
<p>Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckST64BV0Enabled](shared_pseudocode.html#func_CheckST64BV0Enabled_0)(); var data : bits(512); var address : bits(64); var value : bits(64); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLS64](shared_pseudocode.html#func_CreateAccDescLS64_3)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), withstatus, tagchecked); let Xt : bits(64) = X{}(t); value\[31:0\] = ACCDATA_EL1()\[31:0\]; value\[63:32\] = Xt\[63:32\]; if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then value = [BigEndianReverse](shared_pseudocode.html#func_BigEndianReverse_2){64}(value); end; data\[63:0\] = value; for i = 1 to 7 do value = [X](shared_pseudocode.html#accessor_X_2){64}(t+i); if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then value = [BigEndianReverse](shared_pseudocode.html#func_BigEndianReverse_2){64}(value); end; data\[63+64\*i : 64\*i\] = value; end; if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let status : bits(64) = [MemStore64BWithRet](shared_pseudocode.html#func_MemStore64BWithRet_3)(address, data, accdesc); if s != 31 then [X](shared_pseudocode.html#accessor_X_2){64}(s) = status; end;

</div>
