mod pessoa;

fn main()
{
    let mut usuario = pessoa::Pessoa{nome: String::from("Fernando Costa"), sobrenome: String::from("Migliorini")};

    usuario.qual_nome();
    usuario.nome_completo();

    usuario.nome = String::from("Fernando");

    usuario.qual_nome();
    usuario.nome_completo();
}
