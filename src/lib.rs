//#![warn(missing_docs)]

use std::{
    ffi::{OsStr, OsString},
    fmt::Display,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Debug)]
pub struct Instance {
    r#as: Option<PathBuf>,
    file: PathBuf,
    format: Option<OutputFormat>,
    output: Option<PathBuf>,
    args: Vec<OsString>,
}

impl Instance {
    pub fn new<P: AsRef<Path>>(file: P) -> Instance {
        Instance {
            r#as: None,
            file: file.as_ref().to_path_buf(),
            format: None,
            output: None,
            args: Vec::new(),
        }
    }

    pub fn set_assembler<P: AsRef<Path>>(&mut self, path: P) {
        self.r#as = Some(path.as_ref().to_path_buf());
    }

    pub fn set_file<P: AsRef<Path>>(&mut self, file: P) {
        self.file = file.as_ref().to_path_buf();
    }

    pub fn set_format(&mut self, format: OutputFormat) {
        self.format = Some(format);
    }

    pub fn arg<S>(&mut self, arg: S) -> &mut Instance
    where
        S: AsRef<OsStr>,
    {
        self.args.push(arg.as_ref().to_os_string());
        self
    }

    pub fn compile(&self) -> Result<PathBuf, String> {
        let assembler = self.get_assembler()?;

        let mut cmd = Command::new(assembler);
        if let Some(format) = &self.format {
            cmd.arg("-f").arg(format.to_string());
        }

        let output_path = if let Some(path) = self.output.as_ref() {
            cmd.arg("-o").arg(path);
            path.clone()
        } else {
            println!("{}", self.file.display());
            Instance::convert_path(
                self.format.as_ref().unwrap_or(&OutputFormat::Binary),
                &self.file,
            )
        };

        cmd.arg(&self.file);

        println!("Running: {:?}", cmd);

        let output = cmd
            .spawn()
            .map_err(|err| err.to_string())?
            .wait_with_output()
            .map_err(|err| err.to_string())?;

        if !output.status.success() {
            return Err(format!(
                "Exited with code {}",
                output.status.code().unwrap_or_default()
            ));
        }

        Ok(output_path)
    }

    fn convert_path<P: AsRef<Path>>(format: &OutputFormat, path: P) -> PathBuf {
        let path = path.as_ref();
        match format {
            OutputFormat::Binary => path.with_extension(""),
            OutputFormat::Ith => path.with_extension("ith"),
            OutputFormat::SRec => path.with_extension("srec"),
            OutputFormat::Dbg => path.with_extension("dbg"),
            OutputFormat::Obj | OutputFormat::Win32 | OutputFormat::Win64 => {
                path.with_extension("obj")
            }
            OutputFormat::Coff
            | OutputFormat::Macho32
            | OutputFormat::Macho64
            | OutputFormat::Elf32
            | OutputFormat::Elf64
            | OutputFormat::Elfx32
            | OutputFormat::Aout
            | OutputFormat::Aoutb
            | OutputFormat::As86 => path.with_extension("o"),
        }
    }

    fn get_assembler(&self) -> Result<PathBuf, String> {
        match &self.r#as {
            Some(p) => return Ok(p.to_owned()),
            None => {
                let path = std::env::var_os("PATH").unwrap_or_default();
                let paths: Vec<PathBuf> = std::iter::once(PathBuf::from("nasm"))
                    .chain(std::env::split_paths(&path).map(|p| p.join("nasm")))
                    .collect();

                let mut first_error = None;
                for nasm_path in paths {
                    match self.is_nasm(&nasm_path) {
                        Ok(_) => return Ok(nasm_path),
                        Err(err) => {
                            first_error.get_or_insert(err);
                        }
                    }
                }

                Err(first_error.unwrap())
            }
        }
    }

    fn is_nasm(&self, path: &Path) -> Result<(), String> {
        let output = Command::new(path)
            .arg("-v")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|err| err.to_string())?
            .wait_with_output()
            .map_err(|err| err.to_string())?;

        if !output.status.success() {
            return Err(format!(
                "nasm returned with error code {}",
                output.status.code().unwrap_or(0)
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OutputFormat {
    Binary,
    Ith,
    SRec,
    Obj,
    Win32,
    Win64,
    Coff,
    Macho32,
    Macho64,
    Elf32,
    Elf64,
    Elfx32,
    Aout,
    Aoutb,
    As86,
    Dbg,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            OutputFormat::Binary => write!(f, "bin"),
            OutputFormat::Ith => write!(f, "ith"),
            OutputFormat::SRec => write!(f, "srec"),
            OutputFormat::Obj => write!(f, "obj"),
            OutputFormat::Win32 => write!(f, "win32"),
            OutputFormat::Win64 => write!(f, "win64"),
            OutputFormat::Coff => write!(f, "coff"),
            OutputFormat::Macho32 => write!(f, "macho32"),
            OutputFormat::Macho64 => write!(f, "macho64"),
            OutputFormat::Elf32 => write!(f, "elf32"),
            OutputFormat::Elf64 => write!(f, "elf64"),
            OutputFormat::Elfx32 => write!(f, "elfx32"),
            OutputFormat::Aout => write!(f, "aout"),
            OutputFormat::Aoutb => write!(f, "aoutb"),
            OutputFormat::As86 => write!(f, "as86"),
            OutputFormat::Dbg => write!(f, "dbg"),
        }
    }
}
