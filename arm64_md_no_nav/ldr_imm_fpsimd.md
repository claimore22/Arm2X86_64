## LDR (immediate, SIMD&FP)

Load SIMD&FP register (immediate offset)

This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

It has encodings from 3 classes: [Post-index](#iclass_post_index) , [Pre-index](#iclass_pre_index) and [Unsigned offset](#iclass_unsigned_offset)

### <span id="iclass_post_index"></span>Post-index<span style="font-size:smaller;">\
(FEAT_FP) </span>

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
<td colspan="2" class="lr">size</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">x</td>
<td class="r">1</td>
<td class="lr">0</td>
<td colspan="9" class="lr">imm9</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
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

#### Encoding for the 8-bit variant

<span id="LDR_B_ldst_immpost"></span> Applies when <span class="bitdiff"> (size == 00 && opc == 01)</span>

LDR [\<Bt\>](#Bt "Is the 8-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")

</div>

<div class="encoding">

#### Encoding for the 16-bit variant

<span id="LDR_H_ldst_immpost"></span> Applies when <span class="bitdiff"> (size == 01 && opc == 01)</span>

LDR [\<Ht\>](#Ht "Is the 16-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDR_S_ldst_immpost"></span> Applies when <span class="bitdiff"> (size == 10 && opc == 01)</span>

LDR [\<St\>](#St "Is the 32-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDR_D_ldst_immpost"></span> Applies when <span class="bitdiff"> (size == 11 && opc == 01)</span>

LDR [\<Dt\>](#Dt "Is the 64-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")

</div>

<div class="encoding">

#### Encoding for the 128-bit variant

<span id="LDR_Q_ldst_immpost"></span> Applies when <span class="bitdiff"> (size == 00 && opc == 11)</span>

LDR [\<Qt\>](#Qt "Is the 128-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if opc\[1\] == '1' && size != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let scale : integer{} = if opc\[1\] == '1' then 4 else UInt(size); let wback : boolean = TRUE; let postindex : boolean = TRUE; let offset : bits(64) = SignExtend{}(imm9);

### <span id="iclass_pre_index"></span>Pre-index<span style="font-size:smaller;">\
(FEAT_FP) </span>

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
<td colspan="2" class="lr">size</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">x</td>
<td class="r">1</td>
<td class="lr">0</td>
<td colspan="9" class="lr">imm9</td>
<td class="l">1</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
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

#### Encoding for the 8-bit variant

<span id="LDR_B_ldst_immpre"></span> Applies when <span class="bitdiff"> (size == 00 && opc == 01)</span>

LDR [\<Bt\>](#Bt "Is the 8-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")\]!

</div>

<div class="encoding">

#### Encoding for the 16-bit variant

<span id="LDR_H_ldst_immpre"></span> Applies when <span class="bitdiff"> (size == 01 && opc == 01)</span>

LDR [\<Ht\>](#Ht "Is the 16-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")\]!

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDR_S_ldst_immpre"></span> Applies when <span class="bitdiff"> (size == 10 && opc == 01)</span>

LDR [\<St\>](#St "Is the 32-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")\]!

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDR_D_ldst_immpre"></span> Applies when <span class="bitdiff"> (size == 11 && opc == 01)</span>

LDR [\<Dt\>](#Dt "Is the 64-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")\]!

</div>

<div class="encoding">

#### Encoding for the 128-bit variant

<span id="LDR_Q_ldst_immpre"></span> Applies when <span class="bitdiff"> (size == 00 && opc == 11)</span>

LDR [\<Qt\>](#Qt "Is the 128-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#[\<simm\>](#simm__3 "Is the signed immediate byte offset, in the range -256 to 255, encoded in the "imm9" field.")\]!

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if opc\[1\] == '1' && size != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let scale : integer{} = if opc\[1\] == '1' then 4 else UInt(size); let wback : boolean = TRUE; let postindex : boolean = FALSE; let offset : bits(64) = SignExtend{}(imm9);

### <span id="iclass_unsigned_offset"></span>Unsigned offset<span style="font-size:smaller;">\
(FEAT_FP) </span>

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
<td colspan="2" class="lr">size</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">x</td>
<td class="r">1</td>
<td colspan="12" class="lr">imm12</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
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

#### Encoding for the 8-bit variant

<span id="LDR_B_ldst_pos"></span> Applies when <span class="bitdiff"> (size == 00 && opc == 01)</span>

LDR [\<Bt\>](#Bt "Is the 8-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<pimm\>](#pimm "For the "8-bit" variant: is the optional positive immediate byte offset, in the range 0 to 4095, defaulting to 0 and encoded in the "imm12" field.")}\]

</div>

<div class="encoding">

#### Encoding for the 16-bit variant

<span id="LDR_H_ldst_pos"></span> Applies when <span class="bitdiff"> (size == 01 && opc == 01)</span>

LDR [\<Ht\>](#Ht "Is the 16-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<pimm\>](#pimm__4 "For the "16-bit" variant: is the optional positive immediate byte offset, a multiple of 2 in the range 0 to 8190, defaulting to 0 and encoded in the "imm12" field as <pimm>/2.")}\]

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDR_S_ldst_pos"></span> Applies when <span class="bitdiff"> (size == 10 && opc == 01)</span>

LDR [\<St\>](#St "Is the 32-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<pimm\>](#pimm__6 "For the "32-bit" variant: is the optional positive immediate byte offset, a multiple of 4 in the range 0 to 16380, defaulting to 0 and encoded in the "imm12" field as <pimm>/4.")}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDR_D_ldst_pos"></span> Applies when <span class="bitdiff"> (size == 11 && opc == 01)</span>

LDR [\<Dt\>](#Dt "Is the 64-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<pimm\>](#pimm__8 "For the "64-bit" variant: is the optional positive immediate byte offset, a multiple of 8 in the range 0 to 32760, defaulting to 0 and encoded in the "imm12" field as <pimm>/8.")}\]

</div>

<div class="encoding">

#### Encoding for the 128-bit variant

<span id="LDR_Q_ldst_pos"></span> Applies when <span class="bitdiff"> (size == 00 && opc == 11)</span>

LDR [\<Qt\>](#Qt "Is the 128-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<pimm\>](#pimm__3 "For the "128-bit" variant: is the optional positive immediate byte offset, a multiple of 16 in the range 0 to 65520, defaulting to 0 and encoded in the "imm12" field as <pimm>/16.")}\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if opc\[1\] == '1' && size != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let scale : integer{} = if opc\[1\] == '1' then 4 else UInt(size); let wback : boolean = FALSE; let postindex : boolean = FALSE; let offset : bits(64) = LSL(ZeroExtend{64}(imm12), scale);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Bt&gt;</td>
<td><span id="Bt"></span>
<p>Is the 8-bit name of the SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
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
<td>&lt;Ht&gt;</td>
<td><span id="Ht"></span>
<p>Is the 16-bit name of the SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;St&gt;</td>
<td><span id="St"></span>
<p>Is the 32-bit name of the SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dt&gt;</td>
<td><span id="Dt"></span>
<p>Is the 64-bit name of the SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Qt&gt;</td>
<td><span id="Qt"></span>
<p>Is the 128-bit name of the SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;pimm&gt;</td>
<td><span id="pimm"></span>
<p>For the "8-bit" variant: is the optional positive immediate byte offset, in the range 0 to 4095, defaulting to 0 and encoded in the "imm12" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="pimm__4"></span>
<p>For the "16-bit" variant: is the optional positive immediate byte offset, a multiple of 2 in the range 0 to 8190, defaulting to 0 and encoded in the "imm12" field as &lt;pimm&gt;/2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="pimm__6"></span>
<p>For the "32-bit" variant: is the optional positive immediate byte offset, a multiple of 4 in the range 0 to 16380, defaulting to 0 and encoded in the "imm12" field as &lt;pimm&gt;/4.</p></td>
</tr>
<tr>
<td></td>
<td><span id="pimm__8"></span>
<p>For the "64-bit" variant: is the optional positive immediate byte offset, a multiple of 8 in the range 0 to 32760, defaulting to 0 and encoded in the "imm12" field as &lt;pimm&gt;/8.</p></td>
</tr>
<tr>
<td></td>
<td><span id="pimm__3"></span>
<p>For the "128-bit" variant: is the optional positive immediate byte offset, a multiple of 16 in the range 0 to 65520, defaulting to 0 and encoded in the "imm12" field as &lt;pimm&gt;/16.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Shared Decode

let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let datasize : integer{} = 8 \<\< scale; let nontemporal : boolean = FALSE; let tagchecked : boolean = wback \|\| n != 31;

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); var address : bits(64); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescASIMD](shared_pseudocode.html#func_CreateAccDescASIMD_4)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, tagchecked, privileged); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; if !postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(t) = [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc); if wback then if postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
