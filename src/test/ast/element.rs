use parser::Parse;
use predef::*;
use env::{ Path, PushID };
use envs::*;
use ast::{ Type, Exp, Element, ToID, ToIDMut, ErrAst };
use id::renamed::TypeID;


#[test]
fn struct_empty() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let lens = env.lens();

    Element::parse(
        "struct Test;"
    ).to_id_mut(&mut space).unwrap().define(&mut env).unwrap();

    let e_id = space.get_exp(&Path::parse("Test")).unwrap();
    let ty_id = Type::parse("Test").to_id(&space).unwrap();
    let mut type_val = TypeVal::new(vec!());
    type_val.push_atom(e_id);

    assert_eq!(env.exp[e_id], ExpVal::new_empty(ty_id.push_id(1), 0));
    assert_eq!(space.get_type(&Path::parse("Test")).map(|id|&env.ty[id]), Ok(&type_val));

    assert_eq!(env.lens(), (lens.0+1, lens.1+1, lens.2));
}

#[test]
fn struct_tuple() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let lens = env.lens();

    Element::parse("struct A;").to_id_mut(&mut space).unwrap().define(&mut env).unwrap();
    Element::parse("struct B;").to_id_mut(&mut space).unwrap().define(&mut env).unwrap();
    Element::parse("struct Test(A, B);").to_id_mut(&mut space).unwrap().define(&mut env).unwrap();

    let e_id = space.get_exp(&Path::parse("Test")).unwrap();
    let ty_id = ::predef::func(Type::parse("(A, B)").to_id(&space).unwrap(), Type::parse("Test").to_id(&space).unwrap());
    let mut type_val = TypeVal::new(vec!());
    type_val.push_comp(e_id);

    assert_eq!(env.exp[e_id], ExpVal::new_empty(ty_id.push_id(1), 0));
    assert_eq!(space.get_type(&Path::parse("Test")).map(|id|&env.ty[id]), Ok(&type_val));

    assert_eq!(env.lens(), (lens.0+3, lens.1+3, lens.2));
}

#[test]
fn enum_option() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let lens = env.lens();
    
    Element::parse(
        "enum Option<T> { Some(T), None }"
    ).to_id_mut(&mut space).unwrap().define(&mut env).unwrap();

    assert_eq!(space.get_exp(&Path::parse("None")), Err(ErrAst::UnknownVar(Path::parse("None"))));
    assert_eq!(space.get_exp(&Path::parse("Some")), Err(ErrAst::UnknownVar(Path::parse("Some"))));
    let option_id = space.get_type(&Path::parse("Option")).unwrap();
    let none_id = space.get_exp(&Path::parse("Option::None")).unwrap();
    let some_id = space.get_exp(&Path::parse("Option::Some")).unwrap();
    assert_eq!(env.exp[none_id].ty(none_id, &[type_id!(BOOL_ID)]), type_id!(option_id[BOOL_ID]));
    assert_eq!(env.exp[some_id].ty(some_id, &[type_id!(BOOL_ID)]), type_id!(FN_ID[BOOL_ID, option_id[BOOL_ID]]));

    assert_eq!(env.lens(), (lens.0+1, lens.1+2, lens.2));
}

#[test]
fn letting() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let lens = env.lens();

    Element::parse("enum Nat { Zero, Succ(Nat) }").to_id_mut(&mut space).unwrap().define(&mut env).unwrap();
    Element::parse("let two = Nat::Succ(Nat::Succ(Nat::Zero));").to_id_mut(&mut space).unwrap().define(&mut env).unwrap();
    Element::parse("let two_marked: Nat = Nat::Succ(Nat::Succ(Nat::Zero));").to_id_mut(&mut space).unwrap().define(&mut env).unwrap();

    assert_eq!(space.get_exp(&Path::parse("two")).map(|id|&env.exp[id]), space.get_exp(&Path::parse("two_marked")).map(|id|&env.exp[id]));

    assert_eq!(env.lens(), (lens.0+1, lens.1+4, lens.2));
}

#[test]
fn func() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let lens = env.lens();

    Element::parse(
        "enum Nat { Zero, Succ(Nat) }"
    ).to_id_mut(&mut space).unwrap().define(&mut env).unwrap();
    Element::parse(
        "fn add -> Nat {
            (a: Nat, Nat::Zero) => a,
            (a: Nat, Nat::Succ(p: Nat)) => Nat::Succ(add(a, p))
        }"
    ).to_id_mut(&mut space).unwrap().define(&mut env).unwrap();

    let nat_id = space.get_type(&Path::parse("Nat")).expect("Nat has not been named");
    let add_id = space.get_exp(&Path::parse("add")).expect("add has not been named");
    let add = &env.exp[add_id];
    let exp = Exp::parse(
        "{
            (a: Nat, Nat::Zero) => a,
            (a: Nat, Nat::Succ(p: Nat)) => Nat::Succ(add(a, p))
        }"
    ).to_id(&space).expect("Failed to build lambda");

    assert_eq!(add.val(add_id, &[]).expect("No expresion in add"), exp);
    assert_eq!(add.ty(add_id, &[]), type_id!(FN_ID[(nat_id, nat_id), nat_id]));
    
    assert_eq!(env.lens(), (lens.0+1, lens.1+3, lens.2));
}

#[test]
fn lists() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    
    Element::parse(
        "enum List<+T> { Nil, Cons(T, List<T>)}"
    ).to_id_mut(&mut space).unwrap().define(&mut env).unwrap();
    Element::parse(
        "fn prepend<T>(e: T, l: List<T>) -> List<T> = List::Cons<T>(e, l);"
    ).to_id_mut(&mut space).unwrap().define(&mut env).unwrap();
}
