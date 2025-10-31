// Test trait definitions and implementations

#[test]
fn test_trait_definition() {
    rustico::rustico! {
        rasgo Saludador {
            función saludar(&yo) -> Cadena;
        }
        
        estructura Persona {
            nombre: Cadena,
        }
        
        implementa Saludador para Persona {
            función saludar(&yo) -> Cadena {
                Cadena::desde("¡Hola!")
            }
        }
    }
    
    let persona = Persona { nombre: String::from("Juan") };
    assert_eq!(persona.saludar(), "¡Hola!");
}

#[test]
fn test_trait_with_default() {
    rustico::rustico! {
        rasgo Calculador {
            función calcular(&yo) -> e32 {
                42
            }
        }
        
        estructura MiCalculador;
        
        implementa Calculador para MiCalculador {}
    }
    
    let calc = MiCalculador;
    assert_eq!(calc.calcular(), 42);
}

#[test]
fn test_self_type() {
    rustico::rustico! {
        estructura Constructor;
        
        implementa Constructor {
            función nuevo() -> Mismo {
                Constructor
            }
        }
    }
    
    let _ = Constructor::new();
}

#[test]
fn test_self_reference() {
    rustico::rustico! {
        estructura Contador {
            valor: e32,
        }
        
        implementa Contador {
            función incrementar(&mutable yo) {
                yo.valor = yo.valor + 1;
            }
            
            función obtener_valor(&mismo) -> e32 {
                mismo.valor
            }
        }
    }
    
    let mut contador = Contador { valor: 0 };
    contador.incrementar();
    assert_eq!(contador.obtener_valor(), 1);
}
