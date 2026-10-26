#![allow(dead_code, static_mut_refs)]

rustico::rustico! {
    usar estd::colecciones::Diccionario;

    rasgo Hola {
        función escribe(&yo, llave: Cadena, valor: Cadena);
        // `fn` alias sin tilde
        funcion consigue(&mismo, llave: Cadena) -> Resultado<Opción<&Cadena>, Cadena>;
    }

    estático mutable DICCIONARIO: Opcion<Diccionario<Cadena, Cadena>> = Ninguno;

    estructura Concreto;

    implementa Hola para Concreto {
        funcion escribe(&yo, llave: Cadena, valor: Cadena) {
            sea dic = inseguro {
                DICCIONARIO.obten_o_inserta_con(Defecto::defecto)
            };
            dic.insertar(llave, valor);
        }
        funcion consigue(&yo, llave: Cadena) -> Resultado<Opcion<&Cadena>, Cadena> {
            si deja Alguno(dic) = inseguro { DICCIONARIO.como_referencia() } {
                Bien(dic.consigue(&llave))
            } sino {
                Error("llave no existe en diccionario".dentro_de())
            }
        }
    }

    púb(jaula) funcion quizas(i: n32) -> Opcion<Resultado<n32, Cadena>> {
        si i % 2 == 1 {
            si i == 42 {
                Alguno(Error(Cadena::desde("caca")))
            } sino {
                Alguno(Bien(33))
            }
        } sino {
            Ninguno
        }
    }

    asinc función ejemplo() {}

    asínc funcion ejemplo2() {
        ejemplo().espera;
    }

    funcion principal() {
        sea mutable x = 31;

        machea x {
            42 => {
                imprimeln!("chales")
            }
            _ => imprimeln!("Buenas!")
        }

        para i de 0..10 {
            sea val = bucle {
                rompe i;
            };

            mientras Difuso x < val {
                x += 1;
            }

            x = si deja Alguno(resultado) = quizas(i) {
                resultado.pelar()
            } sino {
                12
            };
        }

    }

    #[permite(código_inalcanzable)]
    funcion secundario() {
        chales!("ay no!");
        ups!("cuando las vacas vuelen");
    }
}
