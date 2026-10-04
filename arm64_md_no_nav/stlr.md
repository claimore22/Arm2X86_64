## STLR

Store-release register

This instruction stores a 32-bit word or a 64-bit doubleword to a memory location, from a register. The instruction also has memory ordering semantics as described in <span class="armarm-xref" data-linkend="ARMARM_BEIHCHEF">Load-Acquire, Store-Release</span>. For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

It has encodings from 2 classes: [No offset](#iclass_no_offset) and [Pre-index](#iclass_pre_index)

### <span id="iclass_no_offset"></span>No offset

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
<td>0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">(1)</td>
<td>(1)</td>
<td>(1)</td>
<td>(1)</td>
<td class="r">(1)</td>
<td class="lr">1</td>
<td class="l">(1)</td>
<td>(1)</td>
<td>(1)</td>
<td>(1)</td>
<td class="r">(1)</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="7"></td>
<td class="droppedname">L</td>
<td></td>
<td colspan="5" class="droppedname">Rs</td>
<td class="droppedname">o0</td>
<td colspan="5" class="droppedname">Rt2</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="STLR_SL32_ldstord"></span> Applies when <span class="bitdiff"> (size == 10)</span>

STLR [\<Wt\>](#WtOrWZR__4 "Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="STLR_SL64_ldstord"></span> Applies when <span class="bitdiff"> (size == 11)</span>

STLR [\<Xt\>](#XtOrXZR__11 "Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

#### Decode for all variants of this encoding

let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let wback : boolean = FALSE; let offset : integer = 0; let rt_unknown : boolean = FALSE; let elsize : integer{} = 8 \<\< UInt(size); let datasize : integer{} = elsize; let acquire : boolean = FALSE; let tagchecked : boolean = n != 31;

### <span id="iclass_pre_index"></span>Pre-index<span style="font-size:smaller;">\
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
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="7"></td>
<td class="droppedname">L</td>
<td colspan="12"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="STLR_32S_ldapstl_writeback"></span> Applies when <span class="bitdiff"> (size == 10)</span>

STLR [\<Wt\>](#WtOrWZR__4 "Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#-4\]!

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="STLR_64S_ldapstl_writeback"></span> Applies when <span class="bitdiff"> (size == 11)</span>

STLR [\<Xt\>](#XtOrXZR__11 "Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#-8\]!

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_LRCPC3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let wback : boolean = TRUE; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let datasize : integer{} = 8 \<\< UInt(size); let offset : integer = -1 \* (1 \<\< UInt(size)); let acquire : boolean = FALSE; let tagchecked : boolean = TRUE; var rt_unknown : boolean = FALSE; if n == t && n != 31 then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_WBOVERLAPST](shared_pseudocode.html#enum_Unpredictable_WBOVERLAPST)); assert c IN {[Constraint_NONE](shared_pseudocode.html#enum_Constraint_NONE), [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_NONE](shared_pseudocode.html#enum_Constraint_NONE) =\> rt_unknown = FALSE; // value stored is original value when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> rt_unknown = TRUE; // value stored is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wt&gt;</td>
<td><span id="WtOrWZR__4"></span>
<p>Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
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
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__11"></span>
<p>Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

var address : bits(64); let dbytes : integer{} = datasize DIV 8; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescAcqRel](shared_pseudocode.html#func_CreateAccDescAcqRel_4)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), tagchecked, acquire, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); var data : bits(datasize); if rt_unknown then data = ARBITRARY : bits(datasize); else data = [X](shared_pseudocode.html#accessor_X_2){datasize}(t); end; [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc) = data; if wback then if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
