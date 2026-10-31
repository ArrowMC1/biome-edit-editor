fn main() {
    cc::Build::new()
        .compiler(r"D:\LLVM\bin\clang-cl.exe")

        .warnings(false)

        .include("src/cubiomes")
        .include("src/cubiomes/tables")
        .include("src/cubiomes")

        .file("src/cubiomes/noise.c")
        .file("src/cubiomes/biomenoise.c")
        .file("src/cubiomes/terrainnoise.c")
        .file("src/cubiomes/finders.c")
        .file("src/cubiomes/generator.c")
        .file("src/cubiomes/biomes.c")
        .file("src/cubiomes/layers.c")
        .file("src/cubiomes/quadbase.c")
        .file("src/cubiomes/util.c")

        .file("src/cubiomes/wrapper.c")

        .compile("cubiomes");


    println!("cargo::rerun-if-changed=src/cubiomes");
}