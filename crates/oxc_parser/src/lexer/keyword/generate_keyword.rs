use proc_macro2::Ident;
use proc_macro2::Punct;
use proc_macro2::Spacing;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::LitByteStr;

use super::keyword_list;
use super::perfect_hash;
use crate::lexer::Kind;

type Value = &'static keyword_list::KeywordEntry;

/// Structure for generating a perfect hash table for Javascript Keywords.
#[derive(Debug)]
struct KeywordGenHashTable {
    gen: i32, // Current generation
    seed: u64,
    /// (Value, generation) pair. if generation < self.gen, then this entry is available
    /// at current iteration.
    entries: Vec<(Value, i32)>,
}

impl KeywordGenHashTable {
    pub fn new(size: usize) -> Self {
        Self {
            gen: 0,
            seed: 0x70ABCA, /*Random Start */
            entries: vec![(&keyword_list::EMPTY, -1); size],
        }
    }

    pub fn try_fill(&mut self, keys: &[Value]) -> bool {
        self.gen += 1;
        self.seed += 1;
        for &key in keys {
            let slice = key.0.as_bytes();
            let selection = unsafe { perfect_hash::select(slice) };
            let hash = perfect_hash::mix(selection, self.seed);
            let idx = hash as usize % self.entries.len();
            let old_entry = self.entries[idx];
            if old_entry.1 < self.gen {
                // no collision, take this slot
                self.entries[idx] = (key, self.gen);
            } else {
                // collision, try another seed maybe
                return false;
            }
        }
        true
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn into_entries(self) -> impl Iterator<Item = Option<Value>> {
        let target_gen = self.gen;
        self.entries.into_iter().map(move |(s, gen)| if gen == target_gen { Some(s) } else { None })
    }
}

/// Pack all the bytes of the keywords together compactly in a single byte array.
#[derive(Debug)]
struct KeywordGenStringTable {
    pub bytes: Vec<u8>,
}

impl KeywordGenStringTable {
    pub fn new() -> Self {
        Self { bytes: vec![] }
    }

    pub fn find_word_start(&self, key: &str) -> Option<usize> {
        let len = key.len();
        self.bytes.windows(len).position(|slice| slice == key.as_bytes())
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

fn make_hash_table(keys: &[Value]) -> KeywordGenHashTable {
    const ATTEMPTS_PER_TABLE_SIZE: i32 = 50_000;
    const MAX_TABLE_SIZE: usize = 1024;

    let mut table_size = keys.len().next_power_of_two();
    loop {
        let mut hash_table = KeywordGenHashTable::new(table_size as usize);
        for _ in 0..ATTEMPTS_PER_TABLE_SIZE {
            if hash_table.try_fill(keys) {
                return hash_table;
            }
            // try a different seed
        }

        // increase table size and retry
        let next_table_size = table_size * 2;
        if next_table_size > MAX_TABLE_SIZE {
            panic!("Failed to generate hash table of size {}", table_size);
        }
        table_size = next_table_size;
    }
}

fn make_string_table(keys: &[Value]) -> KeywordGenStringTable {
    let mut string_table = KeywordGenStringTable::new();
    for key in keys {
        let slice = key.0.as_bytes();
        let idx = string_table.bytes.windows(slice.len()).position(|s| s == slice);
        // Only insert new string patterns to save space
        if idx.is_none() {
            string_table.bytes.extend_from_slice(slice);
        }
    }

    string_table
}

impl ToTokens for Kind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Ident::new("Kind", Span::call_site()));
        tokens.append(Punct::new(':', Spacing::Joint));
        tokens.append(Punct::new(':', Spacing::Alone));
        let kind = match self {
            Kind::Undetermined => "Undetermined",
            Kind::Eof => "Eof",
            Kind::WhiteSpace => "WhiteSpace",
            Kind::NewLine => "NewLine",
            Kind::Comment => "Comment",
            Kind::MultiLineComment => "MultiLineComment",
            Kind::Ident => "Ident",
            Kind::Await => "Await",
            Kind::Break => "Break",
            Kind::Case => "Case",
            Kind::Catch => "Catch",
            Kind::Class => "Class",
            Kind::Const => "Const",
            Kind::Continue => "Continue",
            Kind::Debugger => "Debugger",
            Kind::Default => "Default",
            Kind::Delete => "Delete",
            Kind::Do => "Do",
            Kind::Else => "Else",
            Kind::Enum => "Enum",
            Kind::Export => "Export",
            Kind::Extends => "Extends",
            Kind::Finally => "Finally",
            Kind::For => "For",
            Kind::Function => "Function",
            Kind::If => "If",
            Kind::Import => "Import",
            Kind::In => "In",
            Kind::Instanceof => "Instanceof",
            Kind::New => "New",
            Kind::Return => "Return",
            Kind::Super => "Super",
            Kind::Switch => "Switch",
            Kind::This => "This",
            Kind::Throw => "Throw",
            Kind::Try => "Try",
            Kind::Typeof => "Typeof",
            Kind::Var => "Var",
            Kind::Void => "Void",
            Kind::While => "While",
            Kind::With => "With",
            Kind::Async => "Async",
            Kind::From => "From",
            Kind::Get => "Get",
            Kind::Meta => "Meta",
            Kind::Of => "Of",
            Kind::Set => "Set",
            Kind::Target => "Target",
            Kind::Accessor => "Accessor",
            Kind::Abstract => "Abstract",
            Kind::As => "As",
            Kind::Asserts => "Asserts",
            Kind::Assert => "Assert",
            Kind::Any => "Any",
            Kind::Boolean => "Boolean",
            Kind::Constructor => "Constructor",
            Kind::Declare => "Declare",
            Kind::Infer => "Infer",
            Kind::Intrinsic => "Intrinsic",
            Kind::Is => "Is",
            Kind::KeyOf => "KeyOf",
            Kind::Module => "Module",
            Kind::Namespace => "Namespace",
            Kind::Never => "Never",
            Kind::Out => "Out",
            Kind::Readonly => "Readonly",
            Kind::Require => "Require",
            Kind::Number => "Number",
            Kind::Object => "Object",
            Kind::Satisfies => "Satisfies",
            Kind::String => "String",
            Kind::Symbol => "Symbol",
            Kind::Type => "Type",
            Kind::Undefined => "Undefined",
            Kind::Unique => "Unique",
            Kind::Unknown => "Unknown",
            Kind::Global => "Global",
            Kind::BigInt => "BigInt",
            Kind::Override => "Override",
            Kind::Implements => "Implements",
            Kind::Interface => "Interface",
            Kind::Let => "Let",
            Kind::Package => "Package",
            Kind::Private => "Private",
            Kind::Protected => "Protected",
            Kind::Public => "Public",
            Kind::Static => "Static",
            Kind::Yield => "Yield",
            Kind::Amp => "Amp",
            Kind::Amp2 => "Amp2",
            Kind::Amp2Eq => "Amp2Eq",
            Kind::AmpEq => "AmpEq",
            Kind::Bang => "Bang",
            Kind::Caret => "Caret",
            Kind::CaretEq => "CaretEq",
            Kind::Colon => "Colon",
            Kind::Comma => "Comma",
            Kind::Dot => "Dot",
            Kind::Dot3 => "Dot3",
            Kind::Eq => "Eq",
            Kind::Eq2 => "Eq2",
            Kind::Eq3 => "Eq3",
            Kind::GtEq => "GtEq",
            Kind::LAngle => "LAngle",
            Kind::LBrack => "LBrack",
            Kind::LCurly => "LCurly",
            Kind::LParen => "LParen",
            Kind::LtEq => "LtEq",
            Kind::Minus => "Minus",
            Kind::Minus2 => "Minus2",
            Kind::MinusEq => "MinusEq",
            Kind::Neq => "Neq",
            Kind::Neq2 => "Neq2",
            Kind::Percent => "Percent",
            Kind::PercentEq => "PercentEq",
            Kind::Pipe => "Pipe",
            Kind::Pipe2 => "Pipe2",
            Kind::Pipe2Eq => "Pipe2Eq",
            Kind::PipeEq => "PipeEq",
            Kind::Plus => "Plus",
            Kind::Plus2 => "Plus2",
            Kind::PlusEq => "PlusEq",
            Kind::Question => "Question",
            Kind::Question2 => "Question2",
            Kind::Question2Eq => "Question2Eq",
            Kind::QuestionDot => "QuestionDot",
            Kind::RAngle => "RAngle",
            Kind::RBrack => "RBrack",
            Kind::RCurly => "RCurly",
            Kind::RParen => "RParen",
            Kind::Semicolon => "Semicolon",
            Kind::ShiftLeft => "ShiftLeft",
            Kind::ShiftLeftEq => "ShiftLeftEq",
            Kind::ShiftRight => "ShiftRight",
            Kind::ShiftRight3 => "ShiftRight3",
            Kind::ShiftRight3Eq => "ShiftRight3Eq",
            Kind::ShiftRightEq => "ShiftRightEq",
            Kind::Slash => "Slash",
            Kind::SlashEq => "SlashEq",
            Kind::Star => "Star",
            Kind::Star2 => "Star2",
            Kind::Star2Eq => "Star2Eq",
            Kind::StarEq => "StarEq",
            Kind::Tilde => "Tilde",
            Kind::Arrow => "Arrow",
            Kind::Null => "Null",
            Kind::True => "True",
            Kind::False => "False",
            Kind::Decimal => "Decimal",
            Kind::Float => "Float",
            Kind::Binary => "Binary",
            Kind::Octal => "Octal",
            Kind::Hex => "Hex",
            Kind::Str => "Str",
            Kind::RegExp => "RegExp",
            Kind::NoSubstitutionTemplate => "NoSubstitutionTemplate",
            Kind::TemplateHead => "TemplateHead",
            Kind::TemplateMiddle => "TemplateMiddle",
            Kind::TemplateTail => "TemplateTail",
            Kind::PrivateIdentifier => "PrivateIdentifier",
            Kind::JSXText => "JSXText",
            Kind::At => "At",
        };
        tokens.append(Ident::new(kind, Span::call_site()));
    }
}

pub fn generate_keyword_table() -> TokenStream {
    let mut keys = Vec::from_iter(keyword_list::KEYWORDS.iter().copied());
    // Sorts in reverse order to enable merging of common prefixes, like for
    // "as", "assert", and "asserts".
    keys.sort_by_key(|entry| std::cmp::Reverse(entry.0.as_bytes()));
    let hash_table = make_hash_table(&keys);
    let string_table = make_string_table(&keys);

    // Dumping the generated code
    let hash_table_size = hash_table.len();
    let hash_table_seed = hash_table.seed();
    let bytes = string_table.bytes();
    let bytes_str = syn::Lit::ByteStr(LitByteStr::new(bytes, Span::call_site()));
    let string_table_size = bytes.len();

    // struct KeywordTableEntry(key_start, size, kind)
    let hash_table_entry_stream = hash_table.into_entries().map(|entry| match entry {
        Some((key, kind)) => {
            let key_start = string_table.find_word_start(key.as_str()).unwrap() as u32;
            let len = key.len() as u32;
            assert!(len > 0);
            quote! {
              KeywordTableEntry(#key_start, #len, #kind)
            }
        }
        None => {
            quote! {
              KeywordTableEntry(0, 0, Kind::Ident)
            }
        }
    });

    // this will be generated to "keyword/keyword_generated.rs"
    quote! {
      use super::perfect_hash;
      use super::keyword_list;
      use crate::Kind;

      // String table contains bytes of all the keywords packed together
      const STRING_TABLE_SIZE: usize = #string_table_size;
      const STRING_TABLE: &'static [u8] = #bytes_str;

      const HASH_TABLE_SIZE: usize = #hash_table_size;
      const HASH_TABLE_SEED: u64 = #hash_table_seed;

      struct KeywordTableEntry(u32/*Index into STRING_TABLE */, u32/*Length */, Kind /*Syntax Kind */);
      struct KeywordTable {
        entries: [KeywordTableEntry; HASH_TABLE_SIZE],
        bytes: [u8; STRING_TABLE_SIZE],
      }
      impl KeywordTable {
        #[inline]
        pub fn match_keyword(&self, candidate: &str) -> Kind {
          let slice = candidate.as_bytes();
          let clen = slice.len();
          // This branch is necessary for the safety requirement of `hash::select`
          if clen < keyword_list::MIN_JS_KEYWORD_LENGTH || clen > keyword_list::MAX_JS_KEYWORD_LENGTH {
            return Kind::Ident;
          }
  
          // Run the hash function
          let selection = unsafe { perfect_hash::select(slice) };
          let hash_code = perfect_hash::mix(selection, HASH_TABLE_SEED);
          let idx = hash_code as usize % HASH_TABLE_SIZE;

          let entry = &self.entries[idx];
          let klen = entry.1 as usize;

          if clen != klen {
            return Kind::Ident;
          }

          let mut result = entry.2;
          let key_start = entry.0 as usize;
          let key_end = key_start + klen;
          let key = &self.bytes[key_start..key_end];
          if slice != key {
            result = Kind::Ident;
          }

          result
        }
      }

      static KEYWORD_TABLE: KeywordTable = KeywordTable {
        entries: [#(#hash_table_entry_stream),*],
        bytes: *#bytes_str
      };

      #[inline]
      pub fn match_keyword(s: &str) -> Kind {
        KEYWORD_TABLE.match_keyword(s)
      }
    }
    .into()
}
