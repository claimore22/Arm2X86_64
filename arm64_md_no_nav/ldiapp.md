## LDIAPP

Load-Acquire RCpc ordered pair of registers

This instruction calculates an address from a base register value and an optional offset, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers. For information on single-copy atomicity and alignment requirements, see <span class="armarm-xref" data-linkend="ARMARM_CHDDCBCC">Requirements for single-copy atomicity</span> and <span class="armarm-xref" data-linkend="ARMARM_CHDFFEGJ">Alignment of data accesses</span>. The instruction also has memory ordering semantics, as described in <span class="armarm-xref" data-linkend="ARMARM_BEIHCHEF">Load-Acquire, Load-AcquirePC, and Store-Release</span>, except that:

- The Memory effects associated with Xt1/Wt1 are Ordered-before the Memory effects associated with Xt2/Wt2.
- If the destination registers are not both WZR or not both XZR, <span class="asm-code">LDIAPP</span> loads from memory with Acquire semantics.
- There is no ordering requirement, separate from the requirements of a Load-AcquirePC or a Store-Release, created by having a Store-Release followed by a Load-AcquirePC instruction.
- The reading of a value written by a Store-Release by a Load-AcquirePC instruction by the same observer does not make the write of the Store-Release globally observed.

For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_LRCPC3) </span>

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
<td class="r">x</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rt2</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">x</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="7"></td>
<td class="droppedname">L</td>
<td></td>
<td colspan="5"></td>
<td colspan="4" class="droppedname">opc2</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit post-index variant

<span id="LDIAPP_32LE_ldiappstilp"></span> Applies when <span class="bitdiff"> (size == 10 && opc2 == 0000)</span>

LDIAPP [\<Wt1\>](#Wt1OrWZR "Is the 32-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Wt2\>](#Wt2OrWZR "Is the 32-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#8

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDIAPP_32L_ldiappstilp"></span> Applies when <span class="bitdiff"> (size == 10 && opc2 == 0001)</span>

LDIAPP [\<Wt1\>](#Wt1OrWZR "Is the 32-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Wt2\>](#Wt2OrWZR "Is the 32-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the 64-bit post-index variant

<span id="LDIAPP_64LS_ldiappstilp"></span> Applies when <span class="bitdiff"> (size == 11 && opc2 == 0000)</span>

LDIAPP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#16

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDIAPP_64L_ldiappstilp"></span> Applies when <span class="bitdiff"> (size == 11 && opc2 == 0001)</span>

LDIAPP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_LRCPC3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let ispair : boolean = TRUE; let postindex : boolean = opc2\[0\] == '0'; var wback : boolean = opc2\[0\] == '0';

<div class="encoding-notes">

<span class="asm-code">LDIAPP</span> has the same <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span> behavior as <span class="asm-code">LDP</span>. For information about this <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span> behavior, see <span class="armarm-xref" data-linkend="CJAEGDJC">Architectural Constraints on UNPREDICTABLE behaviors</span>, and particularly <span class="armarm-xref" data-linkend="CHDBBBDG">LDP and LDIAPP</span>.

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wt1&gt;</td>
<td><span id="Wt1OrWZR"></span>
<p>Is the 32-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wt2&gt;</td>
<td><span id="Wt2OrWZR"></span>
<p>Is the 32-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field.</p></td>
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

<table>
<tbody>
<tr>
<td>&lt;Xt1&gt;</td>
<td><span id="Xt1OrXZR"></span>
<p>Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt2&gt;</td>
<td><span id="Xt2OrXZR"></span>
<p>Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Shared Decode

let t : integer{} = UInt(Rt); let t2 : integer{} = UInt(Rt2); let n : integer{} = UInt(Rn); let scale : integer{} = 2 + UInt(size\[0\]); let datasize : integer{} = 8 \<\< scale; let offset : integer = if opc2\[0\] == '0' then (2 \<\< scale) else 0; let acqrel : boolean = TRUE; let tagchecked : boolean = wback \|\| n != 31; var rt_unknown : boolean = FALSE; var wb_unknown : boolean = FALSE; if wback && (t == n \|\| t2 == n) && n != 31 then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_WBOVERLAPLD](shared_pseudocode.html#enum_Unpredictable_WBOVERLAPLD)); assert c IN {[Constraint_WBSUPPRESS](shared_pseudocode.html#enum_Constraint_WBSUPPRESS), [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_WBSUPPRESS](shared_pseudocode.html#enum_Constraint_WBSUPPRESS) =\> wback = FALSE; // writeback is suppressed when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> wb_unknown = TRUE; // writeback is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end; if t == t2 then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_LDPOVERLAP](shared_pseudocode.html#enum_Unpredictable_LDPOVERLAP)); assert c IN {[Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> rt_unknown = TRUE; // result is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end;

</div>

<div class="ps">

### Operation

var address : bits(64); var data1 : bits(datasize); var data2 : bits(datasize); let dbytes : integer{} = datasize DIV 8; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLDAcqPC](shared_pseudocode.html#func_CreateAccDescLDAcqPC_5)(tagchecked, ispair, acqrel, t, t2); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; if !postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; var full_data : bits(2\*datasize); full_data = [Mem](shared_pseudocode.html#accessor_Mem_3){2\*datasize}(address, accdesc); if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then data2 = full_data\[(datasize-1):0\]; data1 = full_data\[(2\*datasize-1):datasize\]; else data1 = full_data\[(datasize-1):0\]; data2 = full_data\[(2\*datasize-1):datasize\]; end; if rt_unknown then data1 = ARBITRARY : bits(datasize); data2 = ARBITRARY : bits(datasize); end; [X](shared_pseudocode.html#accessor_X_2){datasize}(t) = data1; [X](shared_pseudocode.html#accessor_X_2){datasize}(t2) = data2; if wback then if wb_unknown then address = ARBITRARY : bits(64); elsif postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
