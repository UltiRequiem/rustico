rustico::rustico! {
    usando std::collections::Diccionario;

    sobrecarga Hola {
        funcion escribe(&yo, llave: Texto, valor: Texto);
        funcion consigue(&yo, llave: Texto) -> Resultado<Opcion<&Texto>, Texto>;
    }

    statico mutable DICCIONARIO: Opcion<Diccionario<Texto, Texto>> = Ninguno;

    structura Concreto;

    implementa Hola para Concreto {
        funcion escribe(&yo, llave: Texto, valor: Texto) {
            deja dico = inseguro {
                DICCIONARIO.obten_o_inserta_con(Defecto::defecto)
            };
            dico.insérer(llave, valor);
        }
        funcion consigue(&yo, llave: Texto) -> Resultado<Opcion<&Texto>, Texto> {
            si deja Algunos(dico) = inseguro { DICCIONARIO.como_referencia() } {
                Bien(dico.consigue(&llave))
            } sino {
                Error("fetchez le dico".dentro_de())
            }
        }
    }

    publico(caja) funcion quizas(i: u32) -> Opcion<Resultado<u32, Texto>> {
        si i % 2 == 1 {
            si i == 42 {
                Algunos(Error(Texto::desde("merde")))
            } sino {
                Algunos(Bien(33))
            }
        } sino {
            Ninguno
        }
    }

    asyncrona funcion example() {
    }

    asyncrona funcion example2() {
        example().espera;
    }

    funcion principal() {
        deja mutable x = 31;

        machea x {
            42 => {
                imprime!("chales")
            }
            _ => imprime!("Buenas!")
        }

        para i de 0..10 {
            deja val = loopea {
                rompe i;
            };

            mientras Difuso x < val {
                x += 1;
            }

            x = si deja Algunos(resultado) = quizas(i) {
                resultado.pelar()
            } sino {
                12
            };
        }

    }

    #[permite(codigo_inusado)]
    funcion secundario() {
        chales!("oh non");
        ups!("fetchez la vache");
    }
}
