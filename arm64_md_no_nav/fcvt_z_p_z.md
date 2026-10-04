## FCVT

Floating-point convert (predicated)

This instruction converts the size and precision of each active floating-point element of the source vector, and places the results in the corresponding elements of the destination vector. Inactive elements in the destination vector register remain unmodified or are set to zero, depending on whether merging or zeroing predication is selected.

Since the input and result types have a different size the smaller type is held unpacked in the least significant bits of elements of the larger size. When the input is the smaller type the upper bits of each source element are ignored. When the result is the smaller type the results are zero-extended to fill each destination element.

It has encodings from 12 classes: [Half-precision to single-precision, merging](#iclass_half_precision_to_single_precision_merging) , [Half-precision to single-precision, zeroing](#iclass_half_precision_to_single_precision_zeroing) , [Half-precision to double-precision, merging](#iclass_half_precision_to_double_precision_merging) , [Half-precision to double-precision, zeroing](#iclass_half_precision_to_double_precision_zeroing) , [Single-precision to half-precision, merging](#iclass_single_precision_to_half_precision_merging) , [Single-precision to half-precision, zeroing](#iclass_single_precision_to_half_precision_zeroing) , [Single-precision to double-precision, merging](#iclass_single_precision_to_double_precision_merging) , [Single-precision to double-precision, zeroing](#iclass_single_precision_to_double_precision_zeroing) , [Double-precision to half-precision, merging](#iclass_double_precision_to_half_precision_merging) , [Double-precision to half-precision, zeroing](#iclass_double_precision_to_half_precision_zeroing) , [Double-precision to single-precision, merging](#iclass_double_precision_to_single_precision_merging) and [Double-precision to single-precision, zeroing](#iclass_double_precision_to_single_precision_zeroing)

### <span id="iclass_half_precision_to_single_precision_merging"></span>Half-precision to single-precision, merging<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_h2s"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").S, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").H

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 16; let d_esize : integer{} = 32; let merging : boolean = TRUE;

### <span id="iclass_half_precision_to_single_precision_zeroing"></span>Half-precision to single-precision, zeroing<span style="font-size:smaller;">\
(FEAT_SVE2p2 \|\| FEAT_SME2p2) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_h2sz"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").S, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").H

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p2) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 16; let d_esize : integer{} = 32; let merging : boolean = FALSE;

### <span id="iclass_half_precision_to_double_precision_merging"></span>Half-precision to double-precision, merging<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_h2d"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").D, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").H

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 16; let d_esize : integer{} = 64; let merging : boolean = TRUE;

### <span id="iclass_half_precision_to_double_precision_zeroing"></span>Half-precision to double-precision, zeroing<span style="font-size:smaller;">\
(FEAT_SVE2p2 \|\| FEAT_SME2p2) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_h2dz"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").D, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").H

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p2) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 16; let d_esize : integer{} = 64; let merging : boolean = FALSE;

### <span id="iclass_single_precision_to_half_precision_merging"></span>Single-precision to half-precision, merging<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_s2h"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").H, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").S

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 32; let d_esize : integer{} = 16; let merging : boolean = TRUE;

### <span id="iclass_single_precision_to_half_precision_zeroing"></span>Single-precision to half-precision, zeroing<span style="font-size:smaller;">\
(FEAT_SVE2p2 \|\| FEAT_SME2p2) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_s2hz"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").H, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").S

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p2) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 32; let d_esize : integer{} = 16; let merging : boolean = FALSE;

### <span id="iclass_single_precision_to_double_precision_merging"></span>Single-precision to double-precision, merging<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_s2d"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").D, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").S

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 32; let d_esize : integer{} = 64; let merging : boolean = TRUE;

### <span id="iclass_single_precision_to_double_precision_zeroing"></span>Single-precision to double-precision, zeroing<span style="font-size:smaller;">\
(FEAT_SVE2p2 \|\| FEAT_SME2p2) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_s2dz"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").D, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").S

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p2) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 32; let d_esize : integer{} = 64; let merging : boolean = FALSE;

### <span id="iclass_double_precision_to_half_precision_merging"></span>Double-precision to half-precision, merging<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_d2h"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").H, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 64; let d_esize : integer{} = 16; let merging : boolean = TRUE;

### <span id="iclass_double_precision_to_half_precision_zeroing"></span>Double-precision to half-precision, zeroing<span style="font-size:smaller;">\
(FEAT_SVE2p2 \|\| FEAT_SME2p2) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_d2hz"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").H, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p2) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 64; let d_esize : integer{} = 16; let merging : boolean = FALSE;

### <span id="iclass_double_precision_to_single_precision_merging"></span>Double-precision to single-precision, merging<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_d2s"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").S, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 64; let d_esize : integer{} = 32; let merging : boolean = TRUE;

### <span id="iclass_double_precision_to_single_precision_zeroing"></span>Double-precision to single-precision, zeroing<span style="font-size:smaller;">\
(FEAT_SVE2p2 \|\| FEAT_SME2p2) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">opc2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcvt_z_p_z_d2sz"></span>

FCVT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").S, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p2) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let s_esize : integer{} = 64; let d_esize : integer{} = 32; let merging : boolean = FALSE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd&gt;</td>
<td><span id="Zd"></span>
<p>Is the name of the destination scalable vector register, encoded in the "Zd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Pg&gt;</td>
<td><span id="Pg"></span>
<p>Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn&gt;</td>
<td><span id="Zn"></span>
<p>Is the name of the source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var result : bits(VL) = if merging then [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) else Zeros{VL}; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let element : bits(esize) = operand\[e\*:esize\]; let res : bits(d_esize) = [FPConvertSVE](shared_pseudocode.html#func_FPConvertSVE_4){d_esize, s_esize}(element\[s_esize-1:0\], FPCR()); result\[e\*:esize\] = ZeroExtend{esize}(res); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>

### Operational information

For the "Half-precision to single-precision, merging" , "Half-precision to double-precision, merging" , "Single-precision to half-precision, merging" , "Single-precision to double-precision, merging" , "Double-precision to half-precision, merging" , "Double-precision to single-precision, merging" variants:

The merging variant of this instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and the merging variant of this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> can be predicated or unpredicated.
- A predicated <span class="asm-code">MOVPRFX</span> must use the same governing predicate register as the merging variant this instruction.
- A predicated <span class="asm-code">MOVPRFX</span> must use the larger of the destination element size and first source element size in the preferred disassembly of the merging variant of this instruction.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as the merging variant of this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of the merging variant of this instruction.
