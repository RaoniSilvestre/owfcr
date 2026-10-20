// Modelo Moderno de Relatório de Projeto em Typst
// Baseado em modern-report-template (algo assi), refeito em pt-br com o gemini

#let build-main-header(main-heading-content) = [
  #align(center, smallcaps(main-heading-content))
  #v(-4pt)
  #line(length: 100%, stroke: 0.5pt + luma(120))
]

#let build-secondary-header(main-heading-content, secondary-heading-content) = [
  #smallcaps(main-heading-content) #h(1fr) #emph(secondary-heading-content)
  #v(-4pt)
  #line(length: 100%, stroke: 0.5pt + luma(120))
]

#let is-after(sec-heading, main-heading) = {
  let sec-pos = sec-heading.location().position()
  let main-pos = main-heading.location().position()
  if sec-pos.page > main-pos.page {
    return true
  }
  if sec-pos.page == main-pos.page {
    return sec-pos.y > main-pos.y
  }
  return false
}

#let get-header() = context {
  let current-page = here().page()

  // Encontra se há um título de nível 1 na página atual
  let next-main-heading = query(selector(heading.where(level: 1)).after(here())).find(head-it => {
    head-it.location().page() == current-page
  })
  if next-main-heading != none {
    return build-main-header(next-main-heading.body)
  }

  // Encontra o último título de nível 1 anterior
  let previous-main-headings = query(selector(heading.where(level: 1)).before(here()))
  let last-main-heading = previous-main-headings.at(-1, default: none)

  if last-main-heading == none {
    return none
  }

  // Encontra o último título de nível > 1 antes da localização atual
  let previous-secondary-headings = query(
    selector(heading).before(here()),
  ).filter(head-it => head-it.level > 1)
  let last-secondary-heading = previous-secondary-headings.at(-1, default: none)

  // Encontra se o último título secundário existe e se está após o último título principal
  if last-secondary-heading != none and is-after(last-secondary-heading, last-main-heading) {
    return build-secondary-header(last-main-heading.body, last-secondary-heading.body)
  }

  return build-main-header(last-main-heading.body)
}

// Apelidos para compatibilidade com versões anteriores
#let buildMainHeader = build-main-header
#let buildSecondaryHeader = build-secondary-header
#let isAfter = is-after
#let getHeader = get-header

// Função Principal do Modelo de Projeto
#let project(
  title: "",
  authors: (),
  subtitle: "",
  department: "",
  institute: "",
  address: "",
  logo: none,
  abstract: none,
  subject: "",
  degree: "Bacharelado em Tecnologia",
  stream: "Tecnologia da Informação",
  guide: (),
  body,
) = {
  // Configuração dos metadados do documento
  set document(
    title: title,
    author: authors.map(a => if type(a) == dictionary { a.at("name", default: "") } else { str(a) }),
  )

  // Configuração inicial da página para a Capa
  set page(
    paper: "a4",
    margin: (top: 1in, bottom: 1in, left: 1in, right: 1in),
  )
  set text(font: ("New Computer Modern", "Libertinus Serif", "Times New Roman"), lang: "pt", region: "BR", size: 11pt)
  set par(justify: true, leading: 0.65em)

  // Bloco do Título
  v(0.1fr)
  align(center)[
    #if subject != "" [
      #text(12pt, weight: "bold", smallcaps(subject)) \
    ]
    #text(12pt, weight: "bold", smallcaps("Relatório de Projeto")) \ \
    #text(28pt, weight: 900, smallcaps(title)) \
    #if subtitle != "" [
      #text(14pt, weight: 200, subtitle) \
    ]
    \
    #emph(text(11pt, weight: 200, "Submetido como requisito parcial para")) \
    #emph(text(11pt, weight: 200, "a avaliação da disciplina de")) \
    #if subject != "" [
      #emph(text(11pt, weight: 200, subject))
    ]
  ]

  // Informações do Curso / Grau
  align(center)[
    #text(12pt, weight: "bold", degree) \
    #text(12pt, weight: "bold", "em") \
    #text(12pt, weight: "bold", stream)
    #v(1cm)
    #text(12pt, weight: "bold", "Elaborado por,")
  ]

  // Grade de Autores
  pad(
    top: 1em,
    {
      let chunk-size = 3
      let author-chunks = range(int(calc.ceil(authors.len() / chunk-size))).map(i => {
        authors.slice(i * chunk-size, calc.min((i + 1) * chunk-size, authors.len()))
      })
      for (idx, chunk) in author-chunks.enumerate() {
        grid(
          columns: (1fr,) * chunk.len(),
          gutter: 12pt,
          ..chunk.map(author => align(center, {
            text(11pt, weight: "bold", author.at("name", default: ""))
            if "rollno" in author [ \ Nº: #author.rollno ]
            if "regno" in author [ \ Matrícula: #author.regno ]
            if "department" in author [ \ #author.department ]
            if "email" in author [ \ #link("mailto:" + author.email) ]
          }))
        )
        if idx < author-chunks.len() - 1 {
          v(16pt, weak: true)
        }
      }
    },
  )

  // Seção do Orientador
  if guide != () and guide != none {
    v(1cm)
    align(center)[
      #text(11pt, "Sob a orientação de,") \
      #text(13pt, weight: "bold", smallcaps(guide.at("name", default: ""))) \
      #if "designation" in guide [ #text(11pt, smallcaps(guide.designation + ",")) \ ]
      #if "department" in guide [ #text(11pt, smallcaps(guide.department)) ]
    ]
  }

  // Detalhes Institucionais e Logotipo
  v(0.75fr)
  align(center)[
    #if logo != none [
      #image(logo, width: 26%) \
    ]
    #if department != "" [
      #text(11pt, weight: "bold", smallcaps(department)) \
    ]
    #if institute != "" [
      #text(13pt, institute) \
    ]
    #if address != "" [
      #text(10pt, address)
    ]
  ]

  // Margens do Corpo do Texto (com margem maior à esquerda para encadernação)
  set page(margin: (top: 1in, bottom: 1in, left: 1.5in, right: 1in))

  // Página do Resumo
  if abstract != none {
    pagebreak()
    align(right)[
      #text(28pt, weight: "bold", smallcaps("Resumo"))
    ]
    v(1em)
    set par(justify: true)
    abstract
  }

  // Sumário / Índice
  pagebreak()
  outline(depth: 3, indent: auto)

  // Estilos dos Títulos
  show heading: set block(above: 1.2em, below: 0.6em)

  show heading.where(level: 1): it => {
    pagebreak(weak: true)
    v(1cm)
    align(right)[
      #block[
        #text(28pt, weight: "bold", smallcaps(it.body))
        #v(0.4em)
        #line(length: 100%, stroke: 1.5pt)
      ]
    ]
    v(1.5em)
  }

  show heading.where(level: 2): it => block(
    above: 1.4em,
    below: 0.7em,
    text(18pt, weight: "bold")[
      #if it.numbering != none {
        counter(heading).display()
        h(0.5em)
      }
      #smallcaps(it.body)
    ],
  )

  show heading.where(level: 3): it => block(
    above: 1.2em,
    below: 0.6em,
    text(15pt, weight: "bold")[
      #if it.numbering != none {
        counter(heading).display()
        h(0.5em)
      }
      #smallcaps(it.body)
    ],
  )

  show heading.where(level: 4): it => block(
    above: 1em,
    below: 0.5em,
    text(13pt, weight: "bold", smallcaps(it.body)),
  )

  show heading.where(level: 5): it => block(
    above: 1em,
    below: 0.5em,
    text(11pt, weight: "bold", smallcaps(it.body)),
  )

  // Configuração do Corpo Principal
  set par(justify: true)
  set heading(numbering: "1.1")
  counter(page).update(1)
  set page(
    header: get-header(),
    numbering: "1",
    number-align: center,
  )

  body
}
