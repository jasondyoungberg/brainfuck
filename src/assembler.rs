use std::{fmt, fs, io, path::Path, process::Command};

pub enum AsmError {
    FileSystem(io::Error),
    Command(&'static str, io::Error),
    NasmErr(String),
    LinkerErr(String),
}
impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmError::FileSystem(err) => write!(f, "filesystem error:\n{err}"),
            AsmError::Command(cmd, err) => write!(f, "error executing {cmd}:\n{err}"),
            AsmError::NasmErr(msg) => write!(f, "nasm failed with:\n{msg}"),
            AsmError::LinkerErr(msg) => write!(f, "linker failed with:\n{msg}"),
        }
    }
}

pub fn assemble(asm: &str, out_file: &Path) -> Result<(), AsmError> {
    const ASM_FILE: &str = ".tmp.asm";
    const ELF_FILE: &str = ".tmp.elf";

    fs::write(ASM_FILE, asm).map_err(AsmError::FileSystem)?;

    let nasm_output = Command::new("nasm")
        .arg("-felf64")
        .arg(ASM_FILE)
        .arg("-o")
        .arg(ELF_FILE)
        .output()
        .map_err(|err| {
            let _ = fs::remove_file(ASM_FILE).map_err(AsmError::FileSystem);
            AsmError::Command("nasm", err)
        })?;
    if !nasm_output.status.success() {
        let _ = fs::remove_file(ASM_FILE).map_err(AsmError::FileSystem);
        let _ = fs::remove_file(ELF_FILE).map_err(AsmError::FileSystem);

        let stdout = String::from_utf8_lossy(&nasm_output.stdout);
        let stderr = String::from_utf8_lossy(&nasm_output.stderr);
        return Err(AsmError::NasmErr(format!("{stdout}{stderr}")));
    }

    fs::remove_file(ASM_FILE).map_err(AsmError::FileSystem)?;

    let linker_output = Command::new("cc")
        .arg("-no-pie")
        .arg(ELF_FILE)
        .arg("-o")
        .arg(out_file)
        .output()
        .map_err(|err| {
            let _ = fs::remove_file(ELF_FILE).map_err(AsmError::FileSystem);
            AsmError::Command("cc", err)
        })?;
    if !linker_output.status.success() {
        let _ = fs::remove_file(ELF_FILE).map_err(AsmError::FileSystem);
        let _ = fs::remove_file(out_file).map_err(AsmError::FileSystem);

        let stdout = String::from_utf8_lossy(&linker_output.stdout);
        let stderr = String::from_utf8_lossy(&linker_output.stderr);
        return Err(AsmError::LinkerErr(format!("{stdout}{stderr}")));
    }

    fs::remove_file(ELF_FILE).map_err(AsmError::FileSystem)?;

    Ok(())
}
