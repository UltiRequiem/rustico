// Test Option and Result types

#[test]
fn test_option_some() {
    rustico::rustico! {
        función retornar_alguno() -> Opcion<e32> {
            Alguno(42)
        }
    }

    assert_eq!(retornar_alguno(), Some(42));
}

#[test]
fn test_option_none() {
    rustico::rustico! {
        función retornar_ninguno() -> Opcion<e32> {
            Ninguno
        }
    }

    assert_eq!(retornar_ninguno(), None);
}

#[test]
fn test_option_unwrap() {
    rustico::rustico! {
        función pelar_opcion() -> e32 {
            sea opt = Alguno(100);
            opt.pelar()
        }
    }

    assert_eq!(pelar_opcion(), 100);
}

#[test]
fn test_option_unwrap_or() {
    rustico::rustico! {
        función pelar_o_defecto() -> e32 {
            sea opt: Opcion<e32> = Ninguno;
            opt.pelar_o(50)
        }
    }

    assert_eq!(pelar_o_defecto(), 50);
}

#[test]
fn test_result_ok() {
    rustico::rustico! {
        función retornar_bien() -> Resultado<e32, Cadena> {
            Bien(42)
        }
    }

    assert_eq!(retornar_bien(), Ok(42));
}

#[test]
fn test_result_err() {
    rustico::rustico! {
        función retornar_error() -> Resultado<e32, Cadena> {
            Error(Cadena::desde("falló"))
        }
    }

    assert_eq!(retornar_error(), Err(String::from("falló")));
}

#[test]
fn test_result_unwrap() {
    rustico::rustico! {
        función pelar_resultado() -> e32 {
            sea res: Resultado<e32, Cadena> = Bien(200);
            res.pelar()
        }
    }

    assert_eq!(pelar_resultado(), 200);
}

#[test]
fn test_result_expect() {
    rustico::rustico! {
        función confiar_resultado() -> e32 {
            sea res: Resultado<e32, Cadena> = Bien(300);
            res.confia("debería funcionar")
        }
    }

    assert_eq!(confiar_resultado(), 300);
}

#[test]
fn test_option_match() {
    rustico::rustico! {
        función procesar_opcion(opt: Opcion<e32>) -> e32 {
            machea opt {
                Alguno(val) => val * 2,
                Ninguno => 0
            }
        }
    }

    assert_eq!(procesar_opcion(Some(10)), 20);
    assert_eq!(procesar_opcion(None), 0);
}

#[test]
fn test_result_match() {
    rustico::rustico! {
        función procesar_resultado(res: Resultado<e32, Cadena>) -> e32 {
            machea res {
                Bien(val) => val,
                Error(_) => -1
            }
        }
    }

    assert_eq!(procesar_resultado(Ok(42)), 42);
    assert_eq!(procesar_resultado(Err(String::from("error"))), -1);
}
