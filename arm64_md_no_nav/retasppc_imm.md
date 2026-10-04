## RETAASPPC, RETABSPPC

Return from subroutine, with enhanced pointer authentication using an immediate offset

This instruction authenticates the address that is held in LR, using SP as the first modifier, the specified immediate subtracted from PC as the second modifier, and the specified key, and branches to the authenticated address.

Key A is used for <span class="asm-code">RETAASPPC</span>. Key B is used for <span class="asm-code">RETABSPPC</span>.

If the authentication passes, the PE continues execution at the target of the branch. For information on behavior if the authentication fails, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.Faulting_on_pointer_authentication">Faulting on pointer authentication</span>.

The authenticated address is not written back to LR.

This instruction provides a hint that this is a subroutine return.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_PAuth_LR) </span>

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
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">x</td>
<td colspan="16" class="lr">imm16</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="8"></td>
<td colspan="3" class="droppedname">opc</td>
<td colspan="16"></td>
<td colspan="5" class="droppedname">op2</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the RETAASPPC variant

<span id="RETAASPPC_only_miscbranch"></span> Applies when <span class="bitdiff"> (opc == 000)</span>

RETAASPPC [\<label\>](#imm16_offset "Is the program label whose address is to be calculated. Its negative offset from the address of this instruction, a multiple of 4 in the range -262140 to 0, is encoded as an unsigned value in the "imm16" field as <label>/4.")

</div>

<div class="encoding">

#### Encoding for the RETABSPPC variant

<span id="RETABSPPC_only_miscbranch"></span> Applies when <span class="bitdiff"> (opc == 001)</span>

RETABSPPC [\<label\>](#imm16_offset "Is the program label whose address is to be calculated. Its negative offset from the address of this instruction, a multiple of 4 in the range -262140 to 0, is encoded as an unsigned value in the "imm16" field as <label>/4.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_PAuth_LR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let use_key_a : boolean = opc\[0\] == '0'; let offset : bits(64) = ZeroExtend{}(imm16::'00'); let auth_then_branch : boolean = TRUE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;label&gt;</td>
<td><span id="imm16_offset"></span>
<p>Is the program label whose address is to be calculated. Its negative offset from the address of this instruction, a multiple of 4 in the range -262140 to 0, is encoded as an unsigned value in the "imm16" field as &lt;label&gt;/4.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

var inst_type : [GCSInstruction](shared_pseudocode.html#type_GCSInstruction); var target : bits(64) = X{}(30); let modifier : bits(64) = SP{}(); let modifier2 : bits(64) = [PC64](shared_pseudocode.html#func_PC64_0)() - offset; if use_key_a then target = [AuthIA2](shared_pseudocode.html#func_AuthIA2_4)(target, modifier, modifier2, auth_then_branch); else target = [AuthIB2](shared_pseudocode.html#func_AuthIB2_4)(target, modifier, modifier2, auth_then_branch); end; if IsFeatureImplemented(FEAT_GCS) && [GCSPCREnabled](shared_pseudocode.html#func_GCSPCREnabled_1)([PSTATE](shared_pseudocode.html#global_PSTATE).EL) then inst_type = if use_key_a then [GCSInstType_PRETAA](shared_pseudocode.html#enum_GCSInstType_PRETAA) else [GCSInstType_PRETAB](shared_pseudocode.html#enum_GCSInstType_PRETAB); target = [LoadCheckGCSRecord](shared_pseudocode.html#func_LoadCheckGCSRecord_2)(target, inst_type); [SetCurrentGCSPointer](shared_pseudocode.html#func_SetCurrentGCSPointer_1)([GetCurrentGCSPointer](shared_pseudocode.html#func_GetCurrentGCSPointer_0)() + 8); end; // Value in BTypeNext will be used to set PSTATE.BTYPE [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '00'; let branch_conditional : boolean = FALSE; [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}(target, [BranchType_RET](shared_pseudocode.html#enum_BranchType_RET), branch_conditional);

</div>
