use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Error" => "Err",
        "Bien" => "Ok",
        "Texto" => "String",
        "Diccionario" => "HashMap",
        "Defecto" => "Default",
        "Errorsaso" => "Error",
        "Opcion" => "Option",
        "Algunos" => "Some",
        "Ninguno" => "None",
        "Resultado" => "Result",
        "Yo" => "Self",
        "imprime" => "println",
        "rompe" => "break",
        "asyncrona" => "async",
        "espera" => "await",
        "loopea" => "loop",
        "muevete" => "move",
        "caja" => "crate",
        "codigo_inusado" => "unreachable_code",
        "como" => "as",
        "constante" => "const",
        "sobrecarga" => "trait",
        "inseguro" => "unsafe",
        "de" => "in",
        "desde" => "from",
        "dinamico" => "dyn",
        "pelar" => "unwrap",
        "defecto" => "default",
        "como_referencia" => "as_ref",
        "es" => "io",
        "externa" => "extern",
        "falso" => "false",
        "funcion" => "fn",
        "piola" => "super",
        "insérer" => "insert",
        "consigue" => "get",
        "permite" => "allow",
        "chales" | "ups" => "panic",
        "modulo" => "mod",
        "mutable" => "mut",
        "nuevo" => "new",
        "donde" => "where",
        "para" => "for",
        "obten_o_inserta_con" => "get_or_insert_with",
        "principal" => "main",
        "publico" => "pub",
        "Difuso" => None?,
        "retorna" => "return",
        "implementa" => "impl",
        "referencia" => "ref",
        "machea" => "match",
        "si" => "if",
        "sino" => "else",
        "yo" => "self",
        "deja" => "let",
        "statico" => "static",
        "structura" => "struct",
        "confia" => "expect",
        "mientras" => "while",
        "usando" => "use",
        "dentro_de" => "into",
        "verdad" => "true",
        "enumerado" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rustico(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
