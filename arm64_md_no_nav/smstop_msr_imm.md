## SMSTOP

Disable access to Streaming SVE mode and SME architectural state

This instruction disables access to Streaming SVE mode and SME architectural state.

SMSTOP exits Streaming SVE mode, and disables the SME ZA storage.

SMSTOP SM exits Streaming SVE mode, but does not disable the SME ZA storage.

SMSTOP ZA disables the SME ZA storage, but does not cause an exit from Streaming SVE mode.

This is an alias of [MSR (immediate)](msr_imm.html). This means:

- The encodings in this description are named to match the encodings of [MSR (immediate)](msr_imm.html).
- The description of [MSR (immediate)](msr_imm.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>x</td>
<td>x</td>
<td class="r">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="7"></td>
<td colspan="3" class="droppedname">op1</td>
<td colspan="4"></td>
<td colspan="4" class="droppedname">CRm</td>
<td colspan="3" class="droppedname">op2</td>
<td colspan="5" class="droppedname">Rt</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SMSTOP_MSR_SI_pstate"></span>

SMSTOP {[\<option\>](#pstatefield_option__3 "Is an optional mode, ")}

is equivalent to

[MSR](msr_imm.html#MSR_SI_pstate) [\<pstatefield\>](msr_imm.html#pstatefield_option), \#0

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;option&gt;</td>
<td><span id="pstatefield_option__3"></span>
<p>Is an optional mode, encoded in “CRm[2:1]”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">CRm[2:1]</th>
<th class="symbol">&lt;option&gt;</th>
<th class="symbol">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">RESERVED</td>
<td class="description"></td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">SM</td>
<td class="description"><p>Maps to &lt;pstatefield&gt; SVCRSM.</p></td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">ZA</td>
<td class="description"><p>Maps to &lt;pstatefield&gt; SVCRZA.</p></td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">[absent]</td>
<td class="description"><p>Maps to &lt;pstatefield&gt; SVCRSMZA.</p></td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [MSR (immediate)](msr_imm.html) gives the operational pseudocode for this instruction.

</div>
