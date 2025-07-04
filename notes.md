# Notas de Aprendizaje de Rust

## Tema 1: Fundamentos de Rust
- Conceptos clave:
- Ejemplos de código:
- Dudas o preguntas:

---

## Tema 2: Ownership y Borrowing
- Conceptos clave:
- Ejemplos de código:
- Dudas o preguntas:
Tipo	Qué es	Cuándo usarlo
String	Cadena propietaria (heap)	Cuando vas a modificar, guardar o clonar
&str	Referencia a una cadena (inmutable)	Cuando solo leés o comparás
---

## Tema 3: Tipos de Datos y Estructuras
- Conceptos clave:
- Ejemplos de código:
- Dudas o preguntas:

---

## Tema 4: Manejo de Errores
- Conceptos clave:
- Ejemplos de código:
- Dudas o preguntas:

---

## Tema 5: Concurrencia y Paralelismo
- Conceptos clave:
- Ejemplos de código:
- Dudas o preguntas:

---

## Otros Temas
- Conceptos clave de colecciones:
  - `Vec<T>`: Vector dinámico, similar a un array, permite agregar y quitar elementos, El orden es importante
No te importa buscar por valor (no muy frecuente).
  - `HashMap<K, V>`: Mapa hash, asocia claves a valores, no mantiene orden. El orden no importa. La búsqueda por clave es importante.

  - `BTreeMap<K, V>`: Mapa basado en árbol B, mantiene las claves ordenadas Necesitás que las claves estén ordenadas Querés iterar de forma ordenada

  - `HashSet<T>`: Conjunto hash, almacena elementos únicos sin orden. Te importa evitar duplicados No necesitás asociar valores
  - `BTreeSet<T>`: Conjunto basado en árbol B, almacena elementos únicos en orden. Te importa evitar duplicados No necesitás asociar valores

---

> Agrega más temas o secciones según lo necesites.
