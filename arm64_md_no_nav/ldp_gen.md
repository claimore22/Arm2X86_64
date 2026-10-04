## LDP

Load pair of registers

This instruction calculates an address from a base register value and an immediate offset, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers. For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

It has encodings from 3 classes: [Post-index](#iclass_post_index) , [Pre-index](#iclass_pre_index) and [Signed offset](#iclass_signed_offset)

### <span id="iclass_post_index"></span>Post-index

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
<td class="l">x</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="7" class="lr">imm7</td>
<td colspan="5" class="lr">Rt2</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="3"></td>
<td class="droppedname">L</td>
<td colspan="7"></td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDP_32_ldstpair_post"></span> Applies when <span class="bitdiff"> (opc == 00)</span>

LDP [\<Wt1\>](#Wt1OrWZR "Is the 32-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Wt2\>](#Wt2OrWZR "Is the 32-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#[\<imm\>](#imm__12 "For the "Post-index 32-bit" and "Pre-index 32-bit" variants: is the signed immediate byte offset, a multiple of 4 in the range -256 to 252, encoded in the "imm7" field as <imm>/4.")

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDP_64_ldstpair_post"></span> Applies when <span class="bitdiff"> (opc == 10)</span>

LDP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#[\<imm\>](#imm__15 "For the "Post-index 64-bit" and "Pre-index 64-bit" variants: is the signed immediate byte offset, a multiple of 8 in the range -512 to 504, encoded in the "imm7" field as <imm>/8.")

</div>

#### Decode for all variants of this encoding

var wback : boolean = TRUE; let postindex : boolean = TRUE;

### <span id="iclass_pre_index"></span>Pre-index

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
<td class="l">x</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="7" class="lr">imm7</td>
<td colspan="5" class="lr">Rt2</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="3"></td>
<td class="droppedname">L</td>
<td colspan="7"></td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDP_32_ldstpair_pre"></span> Applies when <span class="bitdiff"> (opc == 00)</span>

LDP [\<Wt1\>](#Wt1OrWZR "Is the 32-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Wt2\>](#Wt2OrWZR "Is the 32-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#[\<imm\>](#imm__12 "For the "Post-index 32-bit" and "Pre-index 32-bit" variants: is the signed immediate byte offset, a multiple of 4 in the range -256 to 252, encoded in the "imm7" field as <imm>/4.")\]!

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDP_64_ldstpair_pre"></span> Applies when <span class="bitdiff"> (opc == 10)</span>

LDP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#[\<imm\>](#imm__15 "For the "Post-index 64-bit" and "Pre-index 64-bit" variants: is the signed immediate byte offset, a multiple of 8 in the range -512 to 504, encoded in the "imm7" field as <imm>/8.")\]!

</div>

#### Decode for all variants of this encoding

var wback : boolean = TRUE; let postindex : boolean = FALSE;

### <span id="iclass_signed_offset"></span>Signed offset

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
<td class="l">x</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="7" class="lr">imm7</td>
<td colspan="5" class="lr">Rt2</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="3"></td>
<td class="droppedname">L</td>
<td colspan="7"></td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDP_32_ldstpair_off"></span> Applies when <span class="bitdiff"> (opc == 00)</span>

LDP [\<Wt1\>](#Wt1OrWZR "Is the 32-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Wt2\>](#Wt2OrWZR "Is the 32-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<imm\>](#imm__4 "For the "Signed offset 32-bit" variant: is the optional signed immediate byte offset, a multiple of 4 in the range -256 to 252, defaulting to 0 and encoded in the "imm7" field as <imm>/4.")}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDP_64_ldstpair_off"></span> Applies when <span class="bitdiff"> (opc == 10)</span>

LDP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<imm\>](#imm__5 "For the "Signed offset 64-bit" variant: is the optional signed immediate byte offset, a multiple of 8 in the range -512 to 504, defaulting to 0 and encoded in the "imm7" field as <imm>/8.")}\]

</div>

#### Decode for all variants of this encoding

var wback : boolean = FALSE; let postindex : boolean = FALSE;

<div class="encoding-notes">

For information about the <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span> behavior of this instruction, see <span class="armarm-xref" data-linkend="CJAEGDJC">Architectural Constraints on UNPREDICTABLE behaviors</span>, and particularly <span class="armarm-xref" data-linkend="CHDBBBDG">LDP</span>.

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
<td>&lt;imm&gt;</td>
<td><span id="imm__12"></span>
<p>For the "Post-index 32-bit" and "Pre-index 32-bit" variants: is the signed immediate byte offset, a multiple of 4 in the range -256 to 252, encoded in the "imm7" field as &lt;imm&gt;/4.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__15"></span>
<p>For the "Post-index 64-bit" and "Pre-index 64-bit" variants: is the signed immediate byte offset, a multiple of 8 in the range -512 to 504, encoded in the "imm7" field as &lt;imm&gt;/8.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__4"></span>
<p>For the "Signed offset 32-bit" variant: is the optional signed immediate byte offset, a multiple of 4 in the range -256 to 252, defaulting to 0 and encoded in the "imm7" field as &lt;imm&gt;/4.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__5"></span>
<p>For the "Signed offset 64-bit" variant: is the optional signed immediate byte offset, a multiple of 8 in the range -512 to 504, defaulting to 0 and encoded in the "imm7" field as &lt;imm&gt;/8.</p></td>
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

let t : integer{} = UInt(Rt); let t2 : integer{} = UInt(Rt2); let n : integer{} = UInt(Rn); let nontemporal : boolean = FALSE; let scale : integer{} = 2 + UInt(opc\[1\]); let datasize : integer{} = 8 \<\< scale; let offset : bits(64) = LSL(SignExtend{64}(imm7), scale); let tagchecked : boolean = wback \|\| n != 31; var rt_unknown : boolean = FALSE; var wb_unknown : boolean = FALSE; if wback && (t == n \|\| t2 == n) && n != 31 then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_WBOVERLAPLD](shared_pseudocode.html#enum_Unpredictable_WBOVERLAPLD)); assert c IN {[Constraint_WBSUPPRESS](shared_pseudocode.html#enum_Constraint_WBSUPPRESS), [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_WBSUPPRESS](shared_pseudocode.html#enum_Constraint_WBSUPPRESS) =\> wback = FALSE; // Writeback is suppressed when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> wb_unknown = TRUE; // Writeback is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end; if t == t2 then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_LDPOVERLAP](shared_pseudocode.html#enum_Unpredictable_LDPOVERLAP)); assert c IN {[Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> rt_unknown = TRUE; // Result is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end;

</div>

<div class="ps">

### Operation

var address : bits(64); var data1 : bits(datasize); var data2 : bits(datasize); let dbytes : integer{} = datasize DIV 8; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let ispair : boolean = TRUE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_7)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, privileged, tagchecked, ispair, t, t2); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; if !postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; let data : bits(2\*datasize) = [Mem](shared_pseudocode.html#accessor_Mem_3){2\*datasize}(address, accdesc); if rt_unknown then data1 = ARBITRARY : bits(datasize); data2 = ARBITRARY : bits(datasize); elsif [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then data2 = data\[(datasize-1):0\]; data1 = data\[(2\*datasize-1):datasize\]; else data1 = data\[(datasize-1):0\]; data2 = data\[(2\*datasize-1):datasize\]; end; [X](shared_pseudocode.html#accessor_X_2){datasize}(t) = data1; [X](shared_pseudocode.html#accessor_X_2){datasize}(t2) = data2; if wback then if wb_unknown then address = ARBITRARY : bits(64); elsif postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
