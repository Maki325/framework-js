use bincode::{Decode, Encode};
use ring::digest::{Digest, SHA256, SHA256_OUTPUT_LEN};
use std::{
  collections::HashMap,
  fs::File,
  io::{BufReader, BufWriter},
  path::Path,
};

pub type ExportName = String;

pub const DEFAULT_EXPORT_KEY: &'static str = "default";

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub enum ExportType {
  JSX,
  AsyncJSX,
  Other,
  AsyncOther,
}

impl ExportType {
  #[allow(unused)]
  pub fn is_async(self) -> bool {
    match self {
      ExportType::AsyncJSX | ExportType::AsyncOther => true,
      ExportType::JSX | ExportType::Other => false,
    }
  }

  pub fn awaited(self) -> ExportType {
    return match self {
      ExportType::JSX => ExportType::AsyncJSX,
      ExportType::Other => ExportType::AsyncOther,
      _ => self,
    };
  }

  // Biggest -> Lowest
  // Awaited JSX -> JSX -> Awaited Other -> Other
  pub fn priority(self) -> u8 {
    return match self {
      ExportType::AsyncJSX => 3,
      ExportType::JSX => 2,
      ExportType::AsyncOther => 1,
      ExportType::Other => 0,
    };
  }

  pub fn gt(self, other: ExportType) -> ExportType {
    if self.priority() > other.priority() {
      return self;
    }
    return other;
  }
}

pub type Exports = HashMap<ExportName, ExportType>;

const TYPE_INFO_VERSION: u16 = 1;

type HashSHA256 = [u8; SHA256_OUTPUT_LEN];

pub struct TypeInfo {
  pub version: u16,
  pub hash: HashSHA256,
  pub exports: Exports,
}

impl std::fmt::Debug for TypeInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("TypeInfo")
      .field("version", &self.version)
      .field_with("hash", |f| {
        write!(f, "SHA256:")?;
        for byte in self.hash {
          write!(f, "{:02x}", byte)?;
        }
        Ok(())
      })
      .field("exports", &self.exports)
      .finish()
  }
}

impl TypeInfo {
  pub fn new(hash: HashSHA256, exports: Exports) -> TypeInfo {
    return TypeInfo {
      version: TYPE_INFO_VERSION,
      hash,
      exports,
    };
  }

  pub fn of_file<P: AsRef<Path>>(
    src_path: P,
    type_path: P,
  ) -> Result<TypeInfo, Box<dyn std::error::Error>> {
    let config = bincode::config::standard().with_fixed_int_encoding();

    let type_path = type_path.as_ref();
    let src_path = src_path.as_ref();

    let input = File::open(src_path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;
    let hash: HashSHA256 = {
      let mut hash: HashSHA256 = [0; SHA256_OUTPUT_LEN];
      hash.copy_from_slice(digest.as_ref());

      hash
    };

    if type_path.exists() {
      let type_info_file = File::open(type_path)?;
      let mut type_info_reader = BufReader::new(type_info_file);
      let type_info: TypeInfo = bincode::decode_from_std_read(&mut type_info_reader, config)?;

      if type_info.version == TYPE_INFO_VERSION && type_info.hash.as_ref() == digest.as_ref() {
        return Ok(type_info);
      }
    }

    let exports = Self::get_exports(src_path)?;

    let output = File::create(type_path).unwrap();
    let mut writer = BufWriter::new(output);

    let type_info = TypeInfo::new(hash, exports);

    bincode::encode_into_std_write(&type_info, &mut writer, config).unwrap();

    return Ok(type_info);
  }

  pub fn get_exports<P: AsRef<Path>>(src_path: P) -> std::io::Result<Exports> {
    use crate::{commands::TypecheckerVisitor, utils};
    use anyhow::Context;
    use std::sync::Arc;
    use swc::try_with_handler;
    use swc_common::{SourceMap, GLOBALS};
    use swc_core::ecma::visit::{as_folder, FoldWith};
    use swc_ecma_ast::EsVersion;
    use swc_ecma_parser::{Syntax, TsConfig};

    let input_file = utils::path::make_abs_path(src_path.as_ref().to_path_buf()).unwrap();

    let cm = Arc::<SourceMap>::default();

    let c = swc::Compiler::new(cm.clone());

    let exports: Exports = GLOBALS
      .set(&Default::default(), || {
        try_with_handler(cm.clone(), Default::default(), |handler| {
          let fm = cm.load_file(&input_file).expect("failed to load file");

          let output = c
            .parse_js(
              fm,
              handler,
              EsVersion::EsNext,
              Syntax::Typescript(TsConfig {
                tsx: true,
                ..Default::default()
              }),
              swc::config::IsModule::Bool(true),
              None,
            )
            .context("failed to parse file")?;

          let mut exports = Exports::new();

          output.fold_with(&mut as_folder(TypecheckerVisitor::new(&c, &mut exports)));

          Ok(exports)
        })
      })
      .unwrap();

    return Ok(exports);
  }
}

fn sha256_digest<R: std::io::Read>(mut reader: R) -> std::io::Result<Digest> {
  let mut context = ring::digest::Context::new(&SHA256);
  let mut buffer = [0; 1024];

  loop {
    let count = reader.read(&mut buffer)?;
    if count == 0 {
      break;
    }
    context.update(&buffer[..count]);
  }

  Ok(context.finish())
}

impl Encode for TypeInfo {
  fn encode<E: bincode::enc::Encoder>(
    &self,
    encoder: &mut E,
  ) -> Result<(), bincode::error::EncodeError> {
    self.version.encode(encoder)?;
    self.hash.encode(encoder)?;

    self.exports.len().encode(encoder)?;
    for (key, value) in &self.exports {
      key.encode(encoder)?;
      value.encode(encoder)?;
    }

    return Ok(());
  }
}

impl Decode for TypeInfo {
  fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
    let version: u16 = Decode::decode(decoder)?;
    let hash = HashSHA256::decode(decoder)?;

    let exports = {
      let len = usize::decode(decoder)?;
      let mut map = Exports::with_capacity(len);
      for _ in 0..len {
        let key = String::decode(decoder)?;
        let value = ExportType::decode(decoder)?;
        map.insert(key, value);
      }

      map
    };

    return Ok(TypeInfo {
      version,
      hash,
      exports,
    });
  }
}
