# Fundamentos rust - Variáveis e Condicionais

## Variáveis

### Consatante

Não podem ser alteradas

```rust
const VARIAVEL:i8 = 2; // i8 é o tipo da variável ( 8 bits )

fn main() {
    println!("Constante: {}", VARIAVEL);
}
```

### Estáticas

Podem ser alteradas ( somente se o mut estiver presente )

```rust
static mut VARIAVEL_ESTATICA: i8 = 2;

fn main() {
    unsafe { // mecessário para mudar variável
        VARIAVEL_ESTATICA += 14;
        let valor = VARIAVEL_ESTATICA;

        println!("Estática: {}", valor);
    }
}

```

### Tipos primitivos

<table>
  <thead>
    <tr>
      <th align="left">Tipo</th>
      <th align="left" width="100%">Descrição</th>
      <th align="right">Tamanho (bits)</th>
    </tr>
  </thead>
  <tbody>
    <tr><td><code>i8</code></td><td>Inteiro com sinal</td><td align="right">8</td></tr>
    <tr><td><code>i16</code></td><td>Inteiro com sinal</td><td align="right">16</td></tr>
    <tr><td><code>i32</code></td><td>Inteiro com sinal (padrão)</td><td align="right">32</td></tr>
    <tr><td><code>i64</code></td><td>Inteiro com sinal</td><td align="right">64</td></tr>
    <tr><td><code>i128</code></td><td>Inteiro com sinal</td><td align="right">128</td></tr>
    <tr><td><code>isize</code></td><td>Inteiro com sinal (depende da arquitetura)</td><td align="right">32 ou 64</td></tr>
    <tr><td><code>u8</code></td><td>Inteiro sem sinal</td><td align="right">8</td></tr>
    <tr><td><code>u16</code></td><td>Inteiro sem sinal</td><td align="right">16</td></tr>
    <tr><td><code>u32</code></td><td>Inteiro sem sinal</td><td align="right">32</td></tr>
    <tr><td><code>u64</code></td><td>Inteiro sem sinal</td><td align="right">64</td></tr>
    <tr><td><code>u128</code></td><td>Inteiro sem sinal</td><td align="right">128</td></tr>
    <tr><td><code>usize</code></td><td>Inteiro sem sinal (depende da arquitetura)</td><td align="right">32 ou 64</td></tr>
    <tr><td><code>f32</code></td><td>Ponto flutuante (precisão simples)</td><td align="right">32</td></tr>
    <tr><td><code>f64</code></td><td>Ponto flutuante (precisão dupla)</td><td align="right">64</td></tr>
    <tr><td><code>bool</code></td><td>Verdadeiro ou falso</td><td align="right">8</td></tr>
    <tr><td><code>char</code></td><td>Caractere Unicode</td><td align="right">32</td></tr>
    <tr><td><code>&amp;str</code></td><td>String imutável (referência)</td><td align="right">depende</td></tr>
    <tr><td><code>String</code></td><td>String dinâmica (heap)</td><td align="right">depende</td></tr>
  </tbody>
</table>

### Estrutura de dados

<table width="100%">
  <thead>
    <tr>
      <th align="left">Tipo</th>
      <th align="left" width="65%">Descrição</th>
      <th align="right">Exemplo</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>struct</code></td>
      <td>Define uma estrutura personalizada com múltiplos campos (tipo um objeto)</td>
      <td align="right"><code>struct User { nome: String }</code></td>
    </tr>
    <tr>
      <td><code>tuple</code></td>
      <td>Grupo de valores de tipos diferentes, sem nome para os campos</td>
      <td align="right"><code>(i32, f64, char)</code></td>
    </tr>
    <tr>
      <td><code>array</code></td>
      <td>Coleção de tamanho fixo com elementos do mesmo tipo</td>
      <td align="right"><code>[1, 2, 3]</code></td>
    </tr>
    <tr>
      <td><code>slice</code></td>
      <td>Referência a uma parte de uma coleção (array ou vetor)</td>
      <td align="right"><code>&amp;arr[0..2]</code></td>
    </tr>
    <tr>
      <td><code>Vec&lt;T&gt;</code></td>
      <td>Vetor dinâmico (pode crescer ou diminuir)</td>
      <td align="right"><code>vec![1, 2, 3]</code></td>
    </tr>
    <tr>
      <td><code>enum</code></td>
      <td>Tipo que pode assumir diferentes variantes</td>
      <td align="right"><code>enum Cor { Vermelho, Azul }</code></td>
    </tr>
    <tr>
      <td><code>HashMap&lt;K, V&gt;</code></td>
      <td>Estrutura de chave-valor (tipo dicionário)</td>
      <td align="right"><code>HashMap::new()</code></td>
    </tr>
  </tbody>
</table>

## Condicionais

### If / Else If / Else

```rust
let number: i8 = 3;

if number < 5 { // condição booleana
    println!("condition was true");
} else {
    println!("condition was false");
}
```

#### Ternário

```rust
let condition = true;
let number: i8 = if condition { 5 } else { 6 };
```

### Match ( Switch Case )

```rust
let number: i8 = 1;

match number {
    1 => println!("Um"),
    2 => println!("Dois"),
    3 => println!("Três"),
    _ => print!("Outro número")
}
```