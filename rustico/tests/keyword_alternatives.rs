// Test various keyword alternatives

#[test]
fn test_function_alternatives() {
    rustico::rustico! {
        // función with tilde
        función con_tilde() -> e32 { 1 }
        
        // funcion without tilde
        funcion sin_tilde() -> e32 { 2 }
    }
    
    assert_eq!(con_tilde(), 1);
    assert_eq!(sin_tilde(), 2);
}

#[test]
fn test_let_alternatives() {
    rustico::rustico! {
        función prueba_deja() -> e32 {
            deja x = 10;
            x
        }
        
        función prueba_sea() -> e32 {
            sea y = 20;
            y
        }
    }
    
    assert_eq!(prueba_deja(), 10);
    assert_eq!(prueba_sea(), 20);
}

#[test]
fn test_return_alternatives() {
    rustico::rustico! {
        función usa_retorna() -> e32 {
            retorna 1;
        }
        
        función usa_devuelve() -> e32 {
            devuelve 2;
        }
    }
    
    assert_eq!(usa_retorna(), 1);
    assert_eq!(usa_devuelve(), 2);
}

#[test]
fn test_loop_alternatives() {
    rustico::rustico! {
        función usa_bucle() -> e32 {
            sea mutable x = 0;
            bucle {
                x = x + 1;
                si x == 5 {
                    rompe;
                }
            }
            x
        }
        
        función usa_ciclo() -> e32 {
            sea mutable y = 0;
            ciclo {
                y = y + 1;
                si y == 3 {
                    romper;
                }
            }
            y
        }
    }
    
    assert_eq!(usa_bucle(), 5);
    assert_eq!(usa_ciclo(), 3);
}

#[test]
fn test_unwrap_alternatives() {
    rustico::rustico! {
        función usa_pelar() -> e32 {
            Alguno(10).pelar()
        }
        
        función usa_desenvolver() -> e32 {
            Alguno(20).desenvolver()
        }
        
        función usa_destapar() -> e32 {
            Alguno(30).destapar()
        }
    }
    
    assert_eq!(usa_pelar(), 10);
    assert_eq!(usa_desenvolver(), 20);
    assert_eq!(usa_destapar(), 30);
}

#[test]
fn test_self_alternatives() {
    rustico::rustico! {
        estructura UsaYo {
            valor: e32,
        }
        
        implementa UsaYo {
            función con_yo(&yo) -> e32 {
                yo.valor
            }
        }
        
        estructura UsaMismo {
            valor: e32,
        }
        
        implementa UsaMismo {
            función con_mismo(&mismo) -> e32 {
                mismo.valor
            }
        }
    }
    
    let obj1 = UsaYo { valor: 100 };
    let obj2 = UsaMismo { valor: 200 };
    assert_eq!(obj1.con_yo(), 100);
    assert_eq!(obj2.con_mismo(), 200);
}

#[test]
fn test_pub_alternatives() {
    rustico::rustico! {
        púb función con_tilde() -> e32 { 1 }
        publico función sin_tilde() -> e32 { 2 }
    }
    
    assert_eq!(con_tilde(), 1);
    assert_eq!(sin_tilde(), 2);
}

#[test]
fn test_match_alternatives() {
    rustico::rustico! {
        función usa_machea(x: e32) -> e32 {
            machea x {
                1 => 10,
                _ => 0,
            }
        }
        
        función usa_encaja(x: e32) -> e32 {
            encaja x {
                2 => 20,
                _ => 0,
            }
        }
    }
    
    assert_eq!(usa_machea(1), 10);
    assert_eq!(usa_encaja(2), 20);
}

#[test]
fn test_for_alternatives() {
    rustico::rustico! {
        función usa_para() -> e32 {
            sea mutable suma = 0;
            para i de 0..3 {
                suma = suma + i;
            }
            suma
        }
        
        función usa_por() -> e32 {
            sea mutable suma = 0;
            por i de 0..3 {
                suma = suma + i;
            }
            suma
        }
    }
    
    assert_eq!(usa_para(), 3);
    assert_eq!(usa_por(), 3);
}

#[test]
fn test_expect_alternatives() {
    rustico::rustico! {
        función usa_confia() -> e32 {
            Alguno(42).confia("debe existir")
        }
        
        función usa_asume() -> e32 {
            Alguno(43).asume("debe existir")
        }
    }
    
    assert_eq!(usa_confia(), 42);
    assert_eq!(usa_asume(), 43);
}
