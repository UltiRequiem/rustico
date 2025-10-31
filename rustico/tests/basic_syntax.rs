// Test basic syntax translations

#[test]
fn test_basic_function() {
    rustico::rustico! {
        función prueba() -> e32 {
            retorna 42;
        }
    }
    
    assert_eq!(prueba(), 42);
}

#[test]
fn test_let_and_mut() {
    rustico::rustico! {
        función prueba_variables() -> e32 {
            sea x = 5;
            sea mutable y = 10;
            y = y + x;
            retorna y;
        }
    }
    
    assert_eq!(prueba_variables(), 15);
}

#[test]
fn test_if_else() {
    rustico::rustico! {
        función prueba_condicional(val: e32) -> e32 {
            si val > 10 {
                retorna 1;
            } sino {
                retorna 0;
            }
        }
    }
    
    assert_eq!(prueba_condicional(15), 1);
    assert_eq!(prueba_condicional(5), 0);
}

#[test]
fn test_match() {
    rustico::rustico! {
        función prueba_macheo(val: e32) -> e32 {
            machea val {
                1 => 10,
                2 => 20,
                _ => 0
            }
        }
    }
    
    assert_eq!(prueba_macheo(1), 10);
    assert_eq!(prueba_macheo(2), 20);
    assert_eq!(prueba_macheo(99), 0);
}

#[test]
fn test_for_loop() {
    rustico::rustico! {
        función prueba_bucle_para() -> e32 {
            sea mutable suma = 0;
            para i de 0..5 {
                suma = suma + i;
            }
            retorna suma;
        }
    }
    
    assert_eq!(prueba_bucle_para(), 10);
}

#[test]
fn test_while_loop() {
    rustico::rustico! {
        función prueba_mientras() -> e32 {
            sea mutable contador = 0;
            mientras contador < 5 {
                contador = contador + 1;
            }
            retorna contador;
        }
    }
    
    assert_eq!(prueba_mientras(), 5);
}

#[test]
fn test_loop_and_break() {
    rustico::rustico! {
        función prueba_bucle_infinito() -> e32 {
            sea mutable x = 0;
            bucle {
                x = x + 1;
                si x == 10 {
                    rompe;
                }
            }
            retorna x;
        }
    }
    
    assert_eq!(prueba_bucle_infinito(), 10);
}
