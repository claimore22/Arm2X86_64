## STR (immediate)

Store register (immediate)

This instruction stores a word or a doubleword from a register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

It has encodings from 3 classes: [Post-index](#iclass_post_index) , [Pre-index](#iclass_pre_index) and [Unsigned offset](#iclass_unsigned_offset)

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
<td class="l">1</td>
<td class="r">x</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="9" class="lr">imm9</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="9"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="STR_32_ldst_immpost"></span> Applies when <span class="bitdiff"> (size == 10)</span>

STR [\<Wt\>](#WtOrWZR__4 "Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="STR_64_ldst_immpost"></span> Applies when <span class="bitdiff"> (size == 11)</span>

STR [\<Xt\>](#XtOrXZR__11 "Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")

</div>

#### Decode for all variants of this encoding

let wback : boolean = TRUE; let postindex : boolean = TRUE; let scale : integer{} = UInt(size); let offset : bits(64) = SignExtend{}(imm9);

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
<td class="l">1</td>
<td class="r">x</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="9" class="lr">imm9</td>
<td class="l">1</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="9"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="STR_32_ldst_immpre"></span> Applies when <span class="bitdiff"> (size == 10)</span>

STR [\<Wt\>](#WtOrWZR__4 "Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")\]!

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="STR_64_ldst_immpre"></span> Applies when <span class="bitdiff"> (size == 11)</span>

STR [\<Xt\>](#XtOrXZR__11 "Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")\]!

</div>

#### Decode for all variants of this encoding

let wback : boolean = TRUE; let postindex : boolean = FALSE; let scale : integer{} = UInt(size); let offset : bits(64) = SignExtend{}(imm9);

### <span id="iclass_unsigned_offset"></span>Unsigned offset

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
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="12" class="lr">imm12</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="12"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="STR_32_ldst_pos"></span> Applies when <span class="bitdiff"> (size == 10)</span>

STR [\<Wt\>](#WtOrWZR__4 "Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<pimm\>](#pimm__6 "For the "32-bit" variant: is the optional positive immediate byte offset, a multiple of 4 in the range 0 to 16380, defaulting to 0 and encoded in the "imm12" field as <pimm>/4.")}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="STR_64_ldst_pos"></span> Applies when <span class="bitdiff"> (size == 11)</span>

STR [\<Xt\>](#XtOrXZR__11 "Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<pimm\>](#pimm__8 "For the "64-bit" variant: is the optional positive immediate byte offset, a multiple of 8 in the range 0 to 32760, defaulting to 0 and encoded in the "imm12" field as <pimm>/8.")}\]

</div>

#### Decode for all variants of this encoding

let wback : boolean = FALSE; let postindex : boolean = FALSE; let scale : integer{} = UInt(size); let offset : bits(64) = LSL(ZeroExtend{64}(imm12), scale);

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
<td>&lt;simm&gt;</td>
<td><span id="simm__3"></span>
<p>Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.</p></td>
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

<table>
<tbody>
<tr>
<td>&lt;pimm&gt;</td>
<td><span id="pimm__6"></span>
<p>For the "32-bit" variant: is the optional positive immediate byte offset, a multiple of 4 in the range 0 to 16380, defaulting to 0 and encoded in the "imm12" field as &lt;pimm&gt;/4.</p></td>
</tr>
<tr>
<td></td>
<td><span id="pimm__8"></span>
<p>For the "64-bit" variant: is the optional positive immediate byte offset, a multiple of 8 in the range 0 to 32760, defaulting to 0 and encoded in the "imm12" field as &lt;pimm&gt;/8.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Shared Decode

let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let datasize : integer{} = 8 \<\< scale; let nontemporal : boolean = FALSE; let tagchecked : boolean = wback \|\| n != 31; var c : [Constraint](shared_pseudocode.html#type_Constraint); var rt_unknown : boolean = FALSE; if wback && n == t && n != 31 then c = ConstrainUnpredictable([Unpredictable_WBOVERLAPST](shared_pseudocode.html#enum_Unpredictable_WBOVERLAPST)); assert c IN {[Constraint_NONE](shared_pseudocode.html#enum_Constraint_NONE), [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_NONE](shared_pseudocode.html#enum_Constraint_NONE) =\> rt_unknown = FALSE; // Value stored is original value when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> rt_unknown = TRUE; // Value stored is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end;

</div>

<div class="ps">

### Operation

var address : bits(64); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_5)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), nontemporal, privileged, tagchecked, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; if !postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; var data : bits(datasize); if rt_unknown then data = ARBITRARY : bits(datasize); else data = [X](shared_pseudocode.html#accessor_X_2){datasize}(t); end; [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc) = data; if wback then if postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
