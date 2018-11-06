extern crate cc;

use std::env;

fn main() {
    let mut cfg = cc::Build::new();
    cfg.warnings(false)
       .flag("-Wno-everything") // Disable all warnings to reduce spam
       .include("lp_solve_5.5")
       .include("lp_solve_5.5/bfp")
       .include("lp_solve_5.5/bfp/bfp_LUSOL")
       .include("lp_solve_5.5/bfp/bfp_LUSOL/LUSOL")
       .include("lp_solve_5.5/colamd")
       .include("lp_solve_5.5/shared")
       .opt_level(3)
       .define("NOISNAN", None)
       .define("INVERSE_ACTIVE", Some("INVERSE_LUSOL"))
       .define("RoleIsExternalInvEngine", None)
       .define("YY_NEVER_INTERACTIVE", None)
       .define("PARSER_LP", None)
       .file("lp_solve_5.5/lp_MDO.c")
       .file("lp_solve_5.5/shared/commonlib.c")
       .file("lp_solve_5.5/colamd/colamd.c")
       .file("lp_solve_5.5/shared/mmio.c")
       .file("lp_solve_5.5/shared/myblas.c")
       .file("lp_solve_5.5/ini.c")
      //.file("lp_solve_5.5/fortify.c") // removed: causes linker warning about no symbols in fortify.o
       .file("lp_solve_5.5/lp_rlp.c")
       .file("lp_solve_5.5/lp_crash.c")
       .file("lp_solve_5.5/bfp/bfp_LUSOL/lp_LUSOL.c")
       .file("lp_solve_5.5/bfp/bfp_LUSOL/LUSOL/lusol.c")
       .file("lp_solve_5.5/lp_Hash.c")
       .file("lp_solve_5.5/lp_lib.c")
       .file("lp_solve_5.5/lp_wlp.c")
       .file("lp_solve_5.5/lp_matrix.c")
       .file("lp_solve_5.5/lp_mipbb.c")
       .file("lp_solve_5.5/lp_MPS.c")
       .file("lp_solve_5.5/lp_params.c")
       .file("lp_solve_5.5/lp_presolve.c")
       .file("lp_solve_5.5/lp_price.c")
       .file("lp_solve_5.5/lp_pricePSE.c")
       .file("lp_solve_5.5/lp_report.c")
       .file("lp_solve_5.5/lp_scale.c")
       .file("lp_solve_5.5/lp_simplex.c")
       .file("lp_solve_5.5/lp_SOS.c")
       .file("lp_solve_5.5/lp_utils.c")
       .file("lp_solve_5.5/yacc_read.c");

    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("linux") | Ok("freebsd") | Ok("openbsd") | Ok("netbsd") => {}
        Ok("android") => {}
        Ok("macos") => {}
        Ok("ios") => {}
        Ok("windows") => {
            cfg.define("WIN32", None);
        }
        tos => panic!("unknown target os {:?}!", tos)
    }

    // removed: causes many warnings about unused flags
//    println!("rerun-if-env-changed=TARGET");
//    if !std::env::var("TARGET").unwrap().contains("msvc") {
//        cfg.flag("-lm");
//        cfg.flag("-ldl");
//    }

    cfg.compile("liblpsolve.a");
}
