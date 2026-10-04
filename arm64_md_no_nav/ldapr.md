## LDAPR

Load-acquire RCpc register

This instruction derives an address from a base register value, loads a 32-bit word or 64-bit doubleword from the derived address in memory, and writes it to a register.

If the destination register is not one of WZR or XZR, <span class="asm-code">LDAPR</span> loads from memory with AcquirePC semantics.

For more information about memory ordering semantics, see <span class="armarm-xref" data-linkend="ARMARM_BEIHCHEF">Load-Acquire, Load-AcquirePC, and Store-Release</span>.

For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

It has encodings from 2 classes: [Post-index](#iclass_post_index) and [No offset](#iclass_no_offset)

### <span id="iclass_post_index"></span>Post-index<span style="font-size:smaller;">\
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
<td class="lr">1</td>
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

<span id="LDAPR_32L_ldapstl_writeback"></span> Applies when <span class="bitdiff"> (size == 10)</span>

LDAPR [\<Wt\>](#WtOrWZR__2 "Is the 32-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#4

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDAPR_64L_ldapstl_writeback"></span> Applies when <span class="bitdiff"> (size == 11)</span>

LDAPR [\<Xt\>](#XtOrXZR__8 "Is the 64-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#8

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_LRCPC3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); var wback : boolean = TRUE; let acquirepc : boolean = TRUE; let regsize : integer{} = if size == '11' then 64 else 32; let datasize : integer{} = 8 \<\< UInt(size); let offset : integer = 1 \<\< UInt(size); let tagchecked : boolean = TRUE; var wb_unknown : boolean = FALSE; if n == t && n != 31 then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_WBOVERLAPLD](shared_pseudocode.html#enum_Unpredictable_WBOVERLAPLD)); assert c IN {[Constraint_WBSUPPRESS](shared_pseudocode.html#enum_Constraint_WBSUPPRESS), [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_WBSUPPRESS](shared_pseudocode.html#enum_Constraint_WBSUPPRESS) =\> wback = FALSE; // writeback is suppressed when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> wb_unknown = TRUE; // writeback is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end;

### <span id="iclass_no_offset"></span>No offset<span style="font-size:smaller;">\
(FEAT_LRCPC) </span>

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
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">(1)</td>
<td>(1)</td>
<td>(1)</td>
<td>(1)</td>
<td class="r">(1)</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="2"></td>
<td class="droppedname">A</td>
<td class="droppedname">R</td>
<td></td>
<td colspan="5" class="droppedname">Rs</td>
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

#### Encoding for the 32-bit variant

<span id="LDAPR_32L_memop"></span> Applies when <span class="bitdiff"> (size == 10)</span>

LDAPR [\<Wt\>](#WtOrWZR__2 "Is the 32-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.") {, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDAPR_64L_memop"></span> Applies when <span class="bitdiff"> (size == 11)</span>

LDAPR [\<Xt\>](#XtOrXZR__8 "Is the 64-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.") {, \#0}\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_LRCPC) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let wback : boolean = FALSE; let offset : integer = 0; let wb_unknown : boolean = FALSE; let elsize : integer{} = 8 \<\< UInt(size); let regsize : integer{} = if elsize == 64 then 64 else 32; let datasize : integer{} = elsize; let acquirepc : boolean = TRUE; let tagchecked : boolean = n != 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wt&gt;</td>
<td><span id="WtOrWZR__2"></span>
<p>Is the 32-bit name of the general-purpose register to be loaded, encoded in the "Rt" field.</p></td>
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
<td><span id="XtOrXZR__8"></span>
<p>Is the 64-bit name of the general-purpose register to be loaded, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

var address : bits(64); var data : bits(datasize); let dbytes : integer{} = datasize DIV 8; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLDAcqPC](shared_pseudocode.html#func_CreateAccDescLDAcqPC_3)(tagchecked, acquirepc, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; data = [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){regsize}(t) = ZeroExtend{regsize}(data); if wback then if wb_unknown then address = ARBITRARY : bits(64); else address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
