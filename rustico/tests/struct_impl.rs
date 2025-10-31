// Test struct and impl

#[test]
fn test_struct_definition() {
    rustico::rustico! {
        estructura Punto {
            x: e32,
            y: e32,
        }

        función crear_punto() -> Punto {
            Punto { x: 10, y: 20 }
        }
    }

    let p = crear_punto();
    assert_eq!(p.x, 10);
    assert_eq!(p.y, 20);
}

#[test]
fn test_impl_methods() {
    rustico::rustico! {
        estructura Rectangulo {
            ancho: e32,
            alto: e32,
        }

        implementa Rectangulo {
            función nuevo(ancho: e32, alto: e32) -> Rectangulo {
                Rectangulo { ancho, alto }
            }

            función area(&yo) -> e32 {
                yo.ancho * yo.alto
            }
        }
    }

    let rect = Rectangulo::new(5, 10);
    assert_eq!(rect.area(), 50);
}

#[test]
fn test_pub_struct() {
    rustico::rustico! {
        púb estructura Publico {
            púb campo: e32,
        }

        función crear_publico() -> Publico {
            Publico { campo: 42 }
        }
    }

    let p = crear_publico();
    assert_eq!(p.campo, 42);
}

#[test]
fn test_enum_definition() {
    rustico::rustico! {
        enumerado Color {
            Rojo,
            Verde,
            Azul,
        }

        función obtener_color() -> Color {
            Color::Rojo
        }
    }

    let _ = obtener_color();
}

#[test]
fn test_enum_with_values() {
    rustico::rustico! {
        enumerado Mensaje {
            Texto(Cadena),
            Numero(e32),
        }

        función procesar_mensaje(msg: Mensaje) -> e32 {
            machea msg {
                Mensaje::Numero(n) => n,
                Mensaje::Texto(_) => 0,
            }
        }
    }

    assert_eq!(procesar_mensaje(Mensaje::Numero(42)), 42);
    assert_eq!(procesar_mensaje(Mensaje::Texto(String::from("hola"))), 0);
}
