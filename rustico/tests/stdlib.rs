// Test standard library imports

#[test]
fn test_std_imports() {
    rustico::rustico! {
        usar estd::colecciones::Diccionario;

        función crear_diccionario() -> Diccionario<Cadena, e32> {
            sea mutable mapa = Diccionario::nuevo();
            mapa.insertar(Cadena::desde("uno"), 1);
            mapa.insertar(Cadena::desde("dos"), 2);
            mapa
        }
    }

    let mapa = crear_diccionario();
    assert_eq!(mapa.get("uno"), Some(&1));
    assert_eq!(mapa.get("dos"), Some(&2));
}

#[test]
fn test_hashset_imports() {
    rustico::rustico! {
        usar estd::colecciones::Conjunto;

        función crear_conjunto() -> Conjunto<e32> {
            sea mutable set = Conjunto::nuevo();
            set.insertar(1);
            set.insertar(2);
            set.insertar(3);
            set
        }
    }

    let set = crear_conjunto();
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(!set.contains(&4));
}

#[test]
fn test_vec_type() {
    rustico::rustico! {
        función crear_vector() -> Vec<e32> {
            sea mutable v = Vec::nuevo();
            v.push(1);
            v.push(2);
            v.push(3);
            v
        }
    }

    let v = crear_vector();
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 2);
    assert_eq!(v[2], 3);
}

#[test]
fn test_box_type() {
    rustico::rustico! {
        función crear_caja() -> Caja<e32> {
            Caja::nuevo(42)
        }
    }

    let boxed = crear_caja();
    assert_eq!(*boxed, 42);
}
