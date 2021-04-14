use self::rule::Rule;

/// CST にテキストの情報を付加したもの。
// TODO: 自己参照構造体にする。
#[derive(Debug, PartialEq, Eq)]
pub struct CstText {
    pub text: String,
    pub cst: Cst,
}

impl CstText {
    /// 与えられたパーサに基づき、与えられたテキストをパースする。
    pub fn parse<F, E: std::error::Error>(text: &str, parser: F) -> std::result::Result<Self, E>
    where
        F: Fn(&str) -> std::result::Result<Cst, E>,
        E: Send,
    {
        let cst = parser(text)?;
        Ok(CstText {
            text: text.to_owned(),
            cst,
        })
    }

    /// self.cst の子要素である Cst について、その要素に相当する text を取得する。
    pub fn get_text(&self, cst: &Cst) -> &str {
        let text = self.text.as_str();
        let (s, e) = cst.range;
        &text[s..e]
    }
}

/// Concrete syntax tree.
/// 1つの CST は構文規則、テキストの範囲、子要素からなり、全体として木構造をなす。
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cst {
    /// 構文規則。
    pub rule: Rule,
    pub range: (usize, usize),
    pub inner: Vec<Cst>,
}

pub mod rule;

impl Cst {
    /// 新たな CST を作成する。
    pub fn new(rule: Rule, range: (usize, usize), inner: Vec<Cst>) -> Self {
        Self { rule, range, inner }
    }

    /// 新たな CST を作成する。
    pub fn new_node(rule: Rule, inner: Vec<Cst>) -> Self {
        let range = inner.iter().fold((usize::MAX, 0), |acc, cst| {
            let (acc_start, acc_end) = acc;
            let (cst_start, cst_end) = cst.range;
            (acc_start.min(cst_start), acc_end.max(cst_end))
        });
        Self { rule, range, inner }
    }

    pub fn as_str<'a>(&'a self, text: &'a str) -> &'a str {
        let (s, e) = self.range;
        &text[s..e]
    }
}

#[macro_export]
macro_rules! cst {
    // - Rule name: 省略可能
    // - range: inner があるときのみ省略可能
    // - inner: 省略可能、リストの形で直接記載可能

    // 省略なし + inner リスト形式
    ($rule:ident ($s:expr, $e:expr) [$($inner:expr),*]) => {
        Cst {
            rule: Rule::$rule,
            range: ($s, $e),
            inner: vec![$($inner),*]
        }
    };
    // 省略なし
    ($rule:ident ($s:expr, $e:expr); $inner:expr) => {
        Cst {
            rule: Rule::$rule,
            range: ($s, $e),
            inner: $inner
        }
    };

    // range 省略
    ($rule:ident [$($inner:expr),*]) => {
        Cst::new_node(Rule::$rule, vec![$($inner),*])
    };
    ($rule:ident; $inner:expr) => {
        Cst::new_node(Rule::$rule, $inner)
    };

    // inner 省略
    ($rule:ident ($s:expr, $e:expr)) => {
        Cst {
            rule: Rule::$rule,
            range: ($s, $e),
            inner: vec![]
        }
    };

    // rule 省略
    (($s:expr, $e:expr) [$($inner:expr),*]) => {
        Cst {
            rule: Rule::misc,
            range: ($s, $e),
            inner: vec![$($inner),*]
        }
    };
    (($s:expr, $e:expr); $inner:expr) => {
        Cst {
            rule: Rule::misc,
            range: ($s, $e),
            inner: $inner
        }
    };

    // rule, range 省略
    ([$($inner:expr),*]) => {
        Cst::new_node(Rule::misc, vec![$($inner),*])
    };

    // rule, inner 省略
    (($s:expr, $e:expr)) => {
        Cst {
            rule: Rule::misc,
            range: ($s, $e),
            inner: vec![]
        }
    };
}
