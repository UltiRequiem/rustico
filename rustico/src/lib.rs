use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "núcleo" => "core",
        "estd" | "estándar" => "std",
        "colecciones" => "collections",
        "e8" => "i8",
        "e16" => "i16",
        "e32" => "i32",
        "e64" => "i64",
        "e128" => "i128",
        "etam" | "etalla" | "etamaño" => "isize",
        // Esto es muy repetitivo.
        // Y todavía falta `NonZero`!
        "n8" => "u8",
        "n16" => "u16",
        "n32" => "u32",
        "n64" => "u64",
        "n128" => "u128",
        "ntam" | "ntalla" | "ntamaño" => "usize",
        "Error" => "Err",
        "Bien" => "Ok",
        "Cadena" => "String",
        "cad" => "str",
        "como_cadena" => "to_string",
        "como_cad" => "to_str",
        "Hilo" => "Thread",
        "hilo" => "thread",
        "crear" => "spawn",         // debería ser "aparecer"?
        "Diccionario" => "HashMap", // no hay traducción satisfactoria para "hash"
        "Conjunto" => "HashSet",
        "Defecto" => "Default",
        "defecto" => "default",
        "Errorsaso" => "Error", // errorsasasaso!!!
        "Opcion" | "Opción" => "Option",
        "Alguno" => "Some",  // Algo == !Nada
        "Ninguno" => "None", // Nada = Nothing
        "Resultado" => "Result",
        "Yo" | "Mismo" => "Self",
        "yo" | "mismo" => "self",
        "imprimeln" => "println", // línea
        "imprime" => "print",     // no hay línea
        "rompe" | "romper" => "break",
        "proceso" => "process",
        "Comando" => "Command",
        "salir" => "exit",
        "asinc" | "asínc" => "async",
        "espera" | "esperar" => "await",
        "bucle" | "ciclo" | "cíclate" | "ciclar" => "loop",
        "muevete" | "muévete" | "mover" => "move",
        "jaula" => "crate",
        "Caja" => "Box",
        "codigo_inalcanzable" | "código_inalcanzable" => "unreachable_code",
        "como" => "as", // no confundir con "¿Cómo?"
        "constante" => "const",
        "rasgo" => "trait", // sobrecarga = overload, overloads no existen
        "inseguro" => "unsafe",
        "de" => "in",
        "desde" => "from",
        "din" | "dinamico" | "dinámico" => "dyn",
        "pelar" | "desenvolver" | "destapar" => "unwrap",
        "pelar_o" | "destapar_o" | "desenvolver_o" => "unwrap_or",
        "pelar_o_sino" | "destapar_o_sino" | "desenvolver_o_sino" => "unwrap_or_else",
        "como_referencia" => "as_ref",
        "es" => "io",
        "externa" | "externo" => "extern",
        "falso" => "false",
        "verdad" => "true",
        "bul" | "bulin" | "bulín" | "boleano" => "bool",
        "funcion" | "función" => "fn",
        "súper" | "piola" => "super",
        "insertar" => "insert",
        "consigue" | "conseguir" | "obtén" | "obtener" => "get",
        "permite" | "permitir" => "allow",
        "advierta" | "advertir" => "warn",
        "deniega" | "denegar" => "deny",
        "prohíbe" | "prohibir" => "forbid",
        "chales" | "ups" => "panic",
        "mód" | "módulo" => "mod",
        "mutable" => "mut",
        "nuevo" => "new",
        "donde" => "where", // no confundir con "¿Dónde?"
        "para" | "por" => "for",
        "obten_o_inserta_con" | "obtén_o_inserta_con" => "get_or_insert_with",
        "principal" => "main",
        "púb" | "publico" | "público" => "pub",
        "Difuso" => None?,
        "retorna" | "retornar" | "devuelve" | "devolver" => "return",
        "implementa" => "impl",
        "referencia" => "ref",
        "machea" | "encaja" => "match",
        "si" => "if",
        "sino" => "else",
        "deja" | "sea" => "let",
        "estatico" | "estático" => "static",
        "estructura" => "struct",
        "confia" | "confía" | "confiar" | "asume" | "asumir" => "expect",
        "mientras" => "while",
        "usando" | "usa" | "usar" => "use",
        "dentro_de" => "into",
        "enumerado" | "enumeración" => "enum",
        "Rasgo" => "Trait",
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
