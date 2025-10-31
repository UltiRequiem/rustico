// Test type translations

#[test]
fn test_signed_integers() {
    rustico::rustico! {
        función prueba_e8() -> e8 { 127 }
        función prueba_e16() -> e16 { 32767 }
        función prueba_e32() -> e32 { 2147483647 }
        función prueba_e64() -> e64 { 9223372036854775807 }
    }

    assert_eq!(prueba_e8(), 127i8);
    assert_eq!(prueba_e16(), 32767i16);
    assert_eq!(prueba_e32(), 2147483647i32);
    assert_eq!(prueba_e64(), 9223372036854775807i64);
}

#[test]
fn test_unsigned_integers() {
    rustico::rustico! {
        función prueba_n8() -> n8 { 255 }
        función prueba_n16() -> n16 { 65535 }
        función prueba_n32() -> n32 { 4294967295 }
        función prueba_n64() -> n64 { 18446744073709551615 }
    }

    assert_eq!(prueba_n8(), 255u8);
    assert_eq!(prueba_n16(), 65535u16);
    assert_eq!(prueba_n32(), 4294967295u32);
    assert_eq!(prueba_n64(), 18446744073709551615u64);
}

#[test]
fn test_bool_type() {
    rustico::rustico! {
        función es_verdadero() -> bul {
            verdad
        }

        función es_falso() -> bul {
            falso
        }
    }

    assert_eq!(es_verdadero(), true);
    assert_eq!(es_falso(), false);
}

#[test]
fn test_string_type() {
    rustico::rustico! {
        función crear_cadena() -> Cadena {
            Cadena::desde("hola")
        }
    }

    assert_eq!(crear_cadena(), "hola");
}
