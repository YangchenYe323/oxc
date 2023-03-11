use oxc_ast::Atom;

use crate::Kind;

pub const MIN_JS_KEYWORD_LENGTH: usize = 2;
pub const MAX_JS_KEYWORD_LENGTH: usize = 11;

pub type KeywordEntry = (Atom, Kind);

pub static EMPTY: KeywordEntry = (Atom::new_inline(""), Kind::Ident);
pub static KW_IS: KeywordEntry = (Atom::new_inline("is"), Kind::Is);
pub static KW_AS: KeywordEntry = (Atom::new_inline("as"), Kind::As);
pub static KW_DO: KeywordEntry = (Atom::new_inline("do"), Kind::Do);
pub static KW_IF: KeywordEntry = (Atom::new_inline("if"), Kind::If);
pub static KW_IN: KeywordEntry = (Atom::new_inline("in"), Kind::In);
pub static KW_OF: KeywordEntry = (Atom::new_inline("of"), Kind::Of);
pub static KW_ANY: KeywordEntry = (Atom::new_inline("any"), Kind::Any);
pub static KW_FOR: KeywordEntry = (Atom::new_inline("for"), Kind::For);
pub static KW_GET: KeywordEntry = (Atom::new_inline("get"), Kind::Get);
pub static KW_LET: KeywordEntry = (Atom::new_inline("let"), Kind::Let);
pub static KW_NEW: KeywordEntry = (Atom::new_inline("new"), Kind::New);
pub static KW_OUT: KeywordEntry = (Atom::new_inline("out"), Kind::Out);
pub static KW_SET: KeywordEntry = (Atom::new_inline("set"), Kind::Set);
pub static KW_TRY: KeywordEntry = (Atom::new_inline("try"), Kind::Try);
pub static KW_VAR: KeywordEntry = (Atom::new_inline("var"), Kind::Var);
pub static KW_CASE: KeywordEntry = (Atom::new_inline("case"), Kind::Case);
pub static KW_ELSE: KeywordEntry = (Atom::new_inline("else"), Kind::Else);
pub static KW_ENUM: KeywordEntry = (Atom::new_inline("enum"), Kind::Enum);
pub static KW_FROM: KeywordEntry = (Atom::new_inline("from"), Kind::From);
pub static KW_META: KeywordEntry = (Atom::new_inline("meta"), Kind::Meta);
pub static KW_NULL: KeywordEntry = (Atom::new_inline("null"), Kind::Null);
pub static KW_THIS: KeywordEntry = (Atom::new_inline("this"), Kind::This);
pub static KW_TRUE: KeywordEntry = (Atom::new_inline("true"), Kind::True);
pub static KW_TYPE: KeywordEntry = (Atom::new_inline("type"), Kind::Type);
pub static KW_VOID: KeywordEntry = (Atom::new_inline("void"), Kind::Void);
pub static KW_WITH: KeywordEntry = (Atom::new_inline("with"), Kind::With);
pub static KW_ASYNC: KeywordEntry = (Atom::new_inline("async"), Kind::Async);
pub static KW_AWAIT: KeywordEntry = (Atom::new_inline("await"), Kind::Await);
pub static KW_BREAK: KeywordEntry = (Atom::new_inline("break"), Kind::Break);
pub static KW_CATCH: KeywordEntry = (Atom::new_inline("catch"), Kind::Catch);
pub static KW_CLASS: KeywordEntry = (Atom::new_inline("class"), Kind::Class);
pub static KW_CONST: KeywordEntry = (Atom::new_inline("const"), Kind::Const);
pub static KW_FALSE: KeywordEntry = (Atom::new_inline("false"), Kind::False);
pub static KW_INFER: KeywordEntry = (Atom::new_inline("infer"), Kind::Infer);
pub static KW_KEYOF: KeywordEntry = (Atom::new_inline("keyof"), Kind::KeyOf);
pub static KW_NEVER: KeywordEntry = (Atom::new_inline("never"), Kind::Never);
pub static KW_SUPER: KeywordEntry = (Atom::new_inline("super"), Kind::Super);
pub static KW_THROW: KeywordEntry = (Atom::new_inline("throw"), Kind::Throw);
pub static KW_WHILE: KeywordEntry = (Atom::new_inline("while"), Kind::While);
pub static KW_YIELD: KeywordEntry = (Atom::new_inline("yield"), Kind::Yield);
pub static KW_ASSERT: KeywordEntry = (Atom::new_inline("assert"), Kind::Assert);
pub static KW_BIGINT: KeywordEntry = (Atom::new_inline("bigint"), Kind::BigInt);
pub static KW_DELETE: KeywordEntry = (Atom::new_inline("delete"), Kind::Delete);
pub static KW_EXPORT: KeywordEntry = (Atom::new_inline("export"), Kind::Export);
pub static KW_GLOBAL: KeywordEntry = (Atom::new_inline("global"), Kind::Global);
pub static KW_IMPORT: KeywordEntry = (Atom::new_inline("import"), Kind::Import);
pub static KW_MODULE: KeywordEntry = (Atom::new_inline("module"), Kind::Module);
pub static KW_NUMBER: KeywordEntry = (Atom::new_inline("number"), Kind::Number);
pub static KW_OBJECT: KeywordEntry = (Atom::new_inline("object"), Kind::Object);
pub static KW_PUBLIC: KeywordEntry = (Atom::new_inline("public"), Kind::Public);
pub static KW_RETURN: KeywordEntry = (Atom::new_inline("return"), Kind::Return);
pub static KW_STATIC: KeywordEntry = (Atom::new_inline("static"), Kind::Static);
pub static KW_STRING: KeywordEntry = (Atom::new_inline("string"), Kind::String);
pub static KW_SWITCH: KeywordEntry = (Atom::new_inline("switch"), Kind::Switch);
pub static KW_SYMBOL: KeywordEntry = (Atom::new_inline("symbol"), Kind::Symbol);
pub static KW_TARGET: KeywordEntry = (Atom::new_inline("target"), Kind::Target);
pub static KW_TYPEOF: KeywordEntry = (Atom::new_inline("typeof"), Kind::Typeof);
pub static KW_UNIQUE: KeywordEntry = (Atom::new_inline("unique"), Kind::Unique);
pub static KW_ASSERTS: KeywordEntry = (Atom::new_inline("asserts"), Kind::Asserts);
pub static KW_BOOLEAN: KeywordEntry = (Atom::new_inline("boolean"), Kind::Boolean);
pub static KW_DECLARE: KeywordEntry = (Atom::new_inline("declare"), Kind::Declare);
pub static KW_DEFAULT: KeywordEntry = (Atom::new_inline("default"), Kind::Default);
pub static KW_EXTENDS: KeywordEntry = (Atom::new_inline("extends"), Kind::Extends);
pub static KW_FINALLY: KeywordEntry = (Atom::new_inline("finally"), Kind::Finally);
pub static KW_PACKAGE: KeywordEntry = (Atom::new_inline("package"), Kind::Package);
pub static KW_PRIVATE: KeywordEntry = (Atom::new_inline("private"), Kind::Private);
pub static KW_REQUIRE: KeywordEntry = (Atom::new_inline("require"), Kind::Require);
pub static KW_UNKNOWN: KeywordEntry = (Atom::new_inline("unknown"), Kind::Unknown);
pub static KW_ABSTRACT: KeywordEntry = (Atom::new_inline("abstract"), Kind::Abstract);
pub static KW_ACCESSOR: KeywordEntry = (Atom::new_inline("accessor"), Kind::Accessor);
pub static KW_CONTINUE: KeywordEntry = (Atom::new_inline("continue"), Kind::Continue);
pub static KW_DEBUGGER: KeywordEntry = (Atom::new_inline("debugger"), Kind::Debugger);
pub static KW_FUNCTION: KeywordEntry = (Atom::new_inline("function"), Kind::Function);
pub static KW_OVERRIDE: KeywordEntry = (Atom::new_inline("override"), Kind::Override);
pub static KW_READONLY: KeywordEntry = (Atom::new_inline("readonly"), Kind::Readonly);
pub static KW_INTERFACE: KeywordEntry = (Atom::new_inline("interface"), Kind::Interface);
pub static KW_INTRINSIC: KeywordEntry = (Atom::new_inline("intrinsic"), Kind::Intrinsic);
pub static KW_NAMESPACE: KeywordEntry = (Atom::new_inline("namespace"), Kind::Namespace);
pub static KW_PROTECTED: KeywordEntry = (Atom::new_inline("protected"), Kind::Protected);
pub static KW_SATISFIES: KeywordEntry = (Atom::new_inline("satisfies"), Kind::Satisfies);
pub static KW_UNDEFINED: KeywordEntry = (Atom::new_inline("undefined"), Kind::Undefined);
pub static KW_IMPLEMENTS: KeywordEntry = (Atom::new_inline("implements"), Kind::Implements);
pub static KW_INSTANCEOF: KeywordEntry = (Atom::new_inline("instanceof"), Kind::Instanceof);
pub static KW_CONSTRUCTOR: KeywordEntry = (Atom::new_inline("constructor"), Kind::Constructor);

pub static KEYWORDS: [&'static KeywordEntry; 84] = [
    &KW_IS,
    &KW_AS,
    &KW_DO,
    &KW_IF,
    &KW_IN,
    &KW_OF,
    &KW_ANY,
    &KW_FOR,
    &KW_GET,
    &KW_LET,
    &KW_NEW,
    &KW_OUT,
    &KW_SET,
    &KW_TRY,
    &KW_VAR,
    &KW_CASE,
    &KW_ELSE,
    &KW_ENUM,
    &KW_FROM,
    &KW_META,
    &KW_NULL,
    &KW_THIS,
    &KW_TRUE,
    &KW_TYPE,
    &KW_VOID,
    &KW_WITH,
    &KW_ASYNC,
    &KW_AWAIT,
    &KW_BREAK,
    &KW_CATCH,
    &KW_CLASS,
    &KW_CONST,
    &KW_FALSE,
    &KW_INFER,
    &KW_KEYOF,
    &KW_NEVER,
    &KW_SUPER,
    &KW_THROW,
    &KW_WHILE,
    &KW_YIELD,
    &KW_ASSERT,
    &KW_BIGINT,
    &KW_DELETE,
    &KW_EXPORT,
    &KW_GLOBAL,
    &KW_IMPORT,
    &KW_MODULE,
    &KW_NUMBER,
    &KW_OBJECT,
    &KW_PUBLIC,
    &KW_RETURN,
    &KW_STATIC,
    &KW_STRING,
    &KW_SWITCH,
    &KW_SYMBOL,
    &KW_TARGET,
    &KW_TYPEOF,
    &KW_UNIQUE,
    &KW_ASSERTS,
    &KW_BOOLEAN,
    &KW_DECLARE,
    &KW_DEFAULT,
    &KW_EXTENDS,
    &KW_FINALLY,
    &KW_PACKAGE,
    &KW_PRIVATE,
    &KW_REQUIRE,
    &KW_UNKNOWN,
    &KW_ABSTRACT,
    &KW_ACCESSOR,
    &KW_CONTINUE,
    &KW_DEBUGGER,
    &KW_FUNCTION,
    &KW_OVERRIDE,
    &KW_READONLY,
    &KW_INTERFACE,
    &KW_INTRINSIC,
    &KW_NAMESPACE,
    &KW_PROTECTED,
    &KW_SATISFIES,
    &KW_UNDEFINED,
    &KW_IMPLEMENTS,
    &KW_INSTANCEOF,
    &KW_CONSTRUCTOR,
];
