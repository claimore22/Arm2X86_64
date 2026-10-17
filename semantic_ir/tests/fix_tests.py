import re
import os

test_dir = r"C:\Users\claimoar\Documents\Rust\Arm2X86_64\semantic_ir\tests"

files_to_fix = [
    'uminv_sve.rs',
    'uminv_sve_predicated.rs',
    'ummla_vector.rs',
    'umov.rs',
    'umov_sve.rs',
    'umsubl.rs',
    'umulh.rs',
    'umulh_sve_unpredicated.rs',
    'umull.rs',
    'umull_sve.rs',
    'uneg_sve_predicated.rs',
    'uqadd.rs',
    'uqadd_sve.rs',
    'uqrshl.rs',
    'uqrshrn_uqrshrn2.rs',
    'uqshl_immediate.rs',
    'uqshl_register.rs',
    'uqshl_sve.rs',
    'uqshrn_uqshrn2.rs',
    'uqshrnb_sve.rs',
    'uqshrnt_sve.rs',
    'uqsub.rs',
    'uqsub_sve.rs',
    'uqxtn_uqxtn2.rs',
    'urecpe.rs',
    'urecpe_sve.rs',
    'urhadd.rs',
    'urhadd_sve.rs',
    'urshl.rs',
    'urshl_sve.rs',
    'ursqrte.rs',
    'ursqrte_sve.rs',
    'ursra.rs',
    'ushll_ushll2.rs',
    'ushrn_ushrn2.rs',
    'usmmla_vector.rs',
    'usqadd.rs',
    'usqadd_sve.rs',
    'usra.rs',
    'usubl_usubl2.rs',
    'usubw_usubw2.rs',
    'uzp1.rs',
    'uzp1_sve.rs',
    'uzp2.rs',
    'uzp2_sve.rs',
    'vcgt_vector.rs',
    'vcgt_scalar.rs',
    'vcge_vector.rs',
    'vcge_scalar.rs',
    'vcvt_f32_f64.rs',
    'vcvt_int_fp.rs',
    'vcvtn_fp_int.rs',
    'vcvtx_fp_int.rs',
    'vcvta_fp_int.rs',
    'vcvtm_fp_int.rs',
    'vcvt_f16_f32.rs',
    'vcvt_f32_f16.rs',
    'vdiv_vector.rs',
    'vdiv_scalar.rs',
    'velvt_sve.rs',
    'vext.rs',
    'vfma_vector.rs',
    'vfma_scalar.rs',
    'vfms_vector.rs',
    'vfms_scalar.rs',
    'vfnma_vector.rs',
    'vfnma_scalar.rs',
    'vfnms_vector.rs',
    'vfnms_scalar.rs',
    'vmaxnm_vector.rs',
    'vmaxnm_scalar.rs',
    'vminnm_vector.rs',
    'vminnm_scalar.rs',
    'vmov_vector.rs',
    'vmov_scalar.rs',
    'vmsr_sve.rs',
    'vmul_vector.rs',
    'vmul_scalar.rs',
    'vneg_vector.rs',
    'vneg_scalar.rs',
    'vneg_sve.rs',
    'vorr_vector.rs',
    'vrinta_fp.rs',
    'vrintm_fp.rs',
    'vrintn_fp.rs',
    'vrintp_fp.rs',
    'vrintx_fp.rs',
    'vrintz_fp.rs',
    'vrshl.rs',
    'vrsqrte.rs',
    'vrsqrte_sve.rs',
    'vrsra.rs',
    'vshll_vshll2.rs',
    'vshrn_vshrn2.rs',
    'vsqrt_vector.rs',
    'vsqrt_scalar.rs',
    'vsqrt_sve.rs',
    'vsub_vector.rs',
    'vsub_scalar.rs',
    'vsub_sve.rs',
    'vtrn1.rs',
    'vtrn1_sve.rs',
    'vtrn2.rs',
    'vtrn2_sve.rs',
    'vuzp1.rs',
    'vuzp1_sve.rs',
    'vuzp2.rs',
    'vuzp2_sve.rs',
    'wacc_vector.rs',
    'xar.rs',
    'xar_sve.rs',
    'zip1.rs',
    'zip1_sve.rs',
    'zip2.rs',
    'zip2_sve.rs',
    # SME test files
    'sme_add_za_array.rs',
    'sme_add_za_mult_single.rs',
    'sme_add_za_mult_vectors.rs',
    'sme_add_za_mult_vector.rs',
    'sme_addha.rs',
    'sme_addspl.rs',
    'sme_addsvl.rs',
    'sme_addva.rs',
    'sme_bf1cvt_bf2cvt.rs',
    'sme_bf1cvtl_bf2cvtl.rs',
    'sme_bfadd.rs',
    'sme_bfclamp.rs',
    'sme_bfcvt_bf16_f8.rs',
    'sme_bfcvt_f32_bf16.rs',
    'sme_bfcvtn.rs',
    'sme_bfdot_mult_idx.rs',
    'sme_bfdot_mult_single.rs',
    'sme_bfdot_mult_vectors.rs',
    'sme_bfmax_mult_single.rs',
    'sme_bfmax_mult_vectors.rs',
    'sme_bfmaxnm_mult_single.rs',
    'sme_bfmaxnm_mult_vectors.rs',
    'sme_bfmin_mult_single.rs',
    'sme_bfmin_mult_vectors.rs',
    'sme_bfminnm_mult_single.rs',
    'sme_bfminnm_mult_vectors.rs',
    'sme_bfmla_mult_idx.rs',
    'sme_bfmla_mult_single.rs',
    'sme_bfmla_mult_vectors.rs',
    'sme_bfmlal_mult_idx.rs',
    'sme_bfmlal_mult_single.rs',
    'sme_bfmlal_mult_vectors.rs',
    'sme_bfmls_mult_idx.rs',
    'sme_bfmls_mult_single.rs',
    'sme_bfmls_mult_vectors.rs',
    'sme_bfmlsl_mult_idx.rs',
    'sme_bfmlsl_mult_single.rs',
    'sme_bfmlsl_mult_vectors.rs',
    'sme_bfmop4a_non_widening.rs',
    'sme_bfmop4a_widening.rs',
    'sme_bfmop4s_non_widening.rs',
    'sme_bfmop4s_widening.rs',
    'sme_bfmopa_non_widening.rs',
    'sme_bfmopa_widening.rs',
    'sme_bfmops_non_widening.rs',
    'sme_bfmops_widening.rs',
    'sme_bfmul_mult_single.rs',
    'sme_bfmul_mult_vectors.rs',
    'sme_bfscale_mult_single.rs',
    'sme_bfscale_mult_vectors.rs',
    'sme_cadd_mult_vectors.rs',
    'sme_cadd_mult_single.rs',
    'sme_cadd_za_array.rs',
    'sme_cadd_za_mult_single.rs',
    'sme_cadd_za_mult_vectors.rs',
    'sme_cadd_za_mult_vector.rs',
    'sme_cdiv_mult_vectors.rs',
    'sme_cdiv_mult_single.rs',
    'sme_cdiv_za_array.rs',
    'sme_cdiv_za_mult_single.rs',
    'sme_cdiv_za_mult_vectors.rs',
    'sme_cdiv_za_mult_vector.rs',
    'sme_cmla_mult_idx.rs',
    'sme_cmla_mult_single.rs',
    'sme_cmla_mult_vectors.rs',
    'sme_cmla_za_array.rs',
    'sme_cmla_za_mult_single.rs',
    'sme_cmla_za_mult_vectors.rs',
    'sme_cmla_za_mult_vector.rs',
    'sme_cmls_mult_idx.rs',
    'sme_cmls_mult_single.rs',
    'sme_cmls_mult_vectors.rs',
    'sme_cmls_za_array.rs',
    'sme_cmls_za_mult_single.rs',
    'sme_cmls_za_mult_vectors.rs',
    'sme_cmls_za_mult_vector.rs',
    'sme_cmul_mult_vectors.rs',
    'sme_cmul_mult_single.rs',
    'sme_cmul_za_array.rs',
    'sme_cmul_za_mult_single.rs',
    'sme_cmul_za_mult_vectors.rs',
    'sme_cmul_za_mult_vector.rs',
    'sme_csub_mult_vectors.rs',
    'sme_csub_mult_single.rs',
    'sme_csub_za_array.rs',
    'sme_csub_za_mult_single.rs',
    'sme_csub_za_mult_vectors.rs',
    'sme_csub_za_mult_vector.rs',
    'sme_fadd_mult_vectors.rs',
    'sme_fadd_mult_single.rs',
    'sme_fadd_za_array.rs',
    'sme_fadd_za_mult_single.rs',
    'sme_fadd_za_mult_vectors.rs',
    'sme_fadd_za_mult_vector.rs',
    'sme_fdiv_mult_vectors.rs',
    'sme_fdiv_mult_single.rs',
    'sme_fdiv_za_array.rs',
    'sme_fdiv_za_mult_single.rs',
    'sme_fdiv_za_mult_vectors.rs',
    'sme_fdiv_za_mult_vector.rs',
    'sme_fmax_mult_vectors.rs',
    'sme_fmax_mult_single.rs',
    'sme_fmax_za_array.rs',
    'sme_fmax_za_mult_single.rs',
    'sme_fmax_za_mult_vectors.rs',
    'sme_fmax_za_mult_vector.rs',
    'sme_fmin_mult_vectors.rs',
    'sme_fmin_mult_single.rs',
    'sme_fmin_za_array.rs',
    'sme_fmin_za_mult_single.rs',
    'sme_fmin_za_mult_vectors.rs',
    'sme_fmin_za_mult_vector.rs',
    'sme_fmul_mult_vectors.rs',
    'sme_fmul_mult_single.rs',
    'sme_fmul_za_array.rs',
    'sme_fmul_za_mult_single.rs',
    'sme_fmul_za_mult_vectors.rs',
    'sme_fmul_za_mult_vector.rs',
    'sme_fmadd_mult_idx.rs',
    'sme_fmadd_mult_single.rs',
    'sme_fmadd_mult_vectors.rs',
    'sme_fmadd_za_array.rs',
    'sme_fmadd_za_mult_single.rs',
    'sme_fmadd_za_mult_vectors.rs',
    'sme_fmadd_za_mult_vector.rs',
    'sme_fmsub_mult_idx.rs',
    'sme_fmsub_mult_single.rs',
    'sme_fmsub_mult_vectors.rs',
    'sme_fmsub_za_array.rs',
    'sme_fmsub_za_mult_single.rs',
    'sme_fmsub_za_mult_vectors.rs',
    'sme_fmsub_za_mult_vector.rs',
    'sme_fnmsub_mult_idx.rs',
    'sme_fnmsub_mult_single.rs',
    'sme_fnmsub_mult_vectors.rs',
    'sme_fnmsub_za_array.rs',
    'sme_fnmsub_za_mult_single.rs',
    'sme_fnmsub_za_mult_vectors.rs',
    'sme_fnmsub_za_mult_vector.rs',
    'sme_fsub_mult_vectors.rs',
    'sme_fsub_mult_single.rs',
    'sme_fsub_za_array.rs',
    'sme_fsub_za_mult_single.rs',
    'sme_fsub_za_mult_vectors.rs',
    'sme_fsub_za_mult_vector.rs',
    'sme_ld1_mult_vectors.rs',
    'sme_ld1_mult_single.rs',
    'sme_ld1_za_array.rs',
    'sme_ld1_za_mult_single.rs',
    'sme_ld1_za_mult_vectors.rs',
    'sme_ld1_za_mult_vector.rs',
    'sme_st1_mult_vectors.rs',
    'sme_st1_mult_single.rs',
    'sme_st1_za_array.rs',
    'sme_st1_za_mult_single.rs',
    'sme_st1_za_mult_vectors.rs',
    'sme_st1_za_mult_vector.rs',
]

def fix_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    
    # Fix let statements ending with . -> ;
    # Match lines that start with optional whitespace, then "let " and end with .
    content = re.sub(r'(?m)^(\s*let\s+.+)\.$', r'\1;', content)
    
    # Fix let instruction = fn(). -> let instruction = fn();
    content = re.sub(r'let instruction = (\w+_instruction)\\(\\)\\.', r'let instruction = \1();', content)
    
    # Fix validate_instruction calls ending with .
    content = re.sub(r'validate_instruction\(&instruction\)\.expect\("([^"]+)"\)\.', r'validate_instruction(&instruction).expect("\1");', content)
    
    # Fix panic! calls ending with .
    content = re.sub(r'panic!\("([^"]+)"\)\.', r'panic!("\1");', content)
    
    # Fix assert_eq! calls ending with .
    content = re.sub(r'assert_eq!\(([^)]+)\)\.', r'assert_eq!(\1);', content)
    
    # Fix assert! calls ending with .
    content = re.sub(r'assert!\(([^)]+)\)\.', r'assert!(\1);', content)
    
    # Fix .to_string(). -> .to_string();
    content = re.sub(r'\.to_string\(\)\.', r'.to_string();', content)
    
    # Fix Vec::new(). -> Vec::new();
    content = re.sub(r'Vec::new\(\)\.', r'Vec::new();', content)
    
    # Fix struct initialization fields ending with . -> ,
    content = re.sub(r'(?m)^(\s+\w+:\s+.+)\.$', r'\1,', content)
    
    # Fix vec![...]. -> vec![...];
    content = re.sub(r'(vec!\[.+)\.\s*$', r'\1;', content)
    
    # Fix method calls like .as_ref(). -> .as_ref();
    content = re.sub(r'\.as_ref\(\)\.', r'.as_ref();', content)
    
    # Fix .clone(). -> .clone();
    content = re.sub(r'\.clone\(\)\.', r'.clone();', content)
    
    # Fix .to_string(). -> .to_string(); (already handled)
    # Fix .expect(). -> .expect(); (already handled via validate_instruction)
    
    # Fix match arm results ending with . -> ,
    # This is for match arms like: Some(x) => expr.
    content = re.sub(r'(?m)^(\s+[^=]+\s*=>\s+.+)\.$', r'\1,', content)
    
    # Fix Expr::Variant { ... }. -> Expr::Variant { ... };
    content = re.sub(r'(Expr::\w+\s*\{[^}]+\})\.\s*$', r'\1;', content, flags=re.MULTILINE | re.DOTALL)
    
    # Fix Stmt::Variant { ... }. -> Stmt::Variant { ... };
    content = re.sub(r'(Stmt::\w+\s*\{[^}]+\})\.\s*$', r'\1;', content, flags=re.MULTILINE | re.DOTALL)
    
    # Fix . else { -> . else { (shouldn't need fixing)
    # But fix lines ending with . before }
    content = re.sub(r'(?m)^(\s*}\.)', r'};', content)
    
    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Fixed: {filepath}")
        return True
    return False

fixed_count = 0
for file in files_to_fix:
    filepath = os.path.join(test_dir, file)
    if os.path.exists(filepath):
        if fix_file(filepath):
            fixed_count += 1
    else:
        print(f"Not found: {file}")

print(f"Total fixed: {fixed_count}")