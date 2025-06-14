// Compile dsa_emu2413 for the VRC7 mapper
//

fn main() {
    cc::Build::new()
        .file("src/mapper/vrc7_audio/dsa_emu2413.c")
        .include("src/mapper/vrc7_audio")
        .compile("dsa_emu2413");

    println!("cargo:rustc-link-lib=static=dsa_emu2413");
    println!("cargo:rerun-if-changed=src/mapper/vrc7_audio/dsa_emu2413.c");
}
