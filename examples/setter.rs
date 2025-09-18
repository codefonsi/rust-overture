use rust_overture::{
    concat::concat_mut,
    setter::{mset, set},
    with::{with, with_mut},
};

#[derive(Debug, Clone)]
struct Repo {
    name: String,
    desc: Option<String>,
}

impl Repo {
    fn mock() -> Self {
        Repo {
            name: "Old".into(),
            desc: None,
        }
    }
}

fn main() {
    // Immutable setter
    let name_setter = |f: Box<dyn Fn(String) -> String>| {
        Box::new(move |mut r: Repo| {
            r.name = f(r.name);
            r
        }) as Box<dyn Fn(Repo) -> Repo>
    };

    let set_name = set(name_setter, "Nomadic Blob".to_string());
    let repo = Repo {
        name: "Old".into(),
        desc: None,
    };
    let updated = set_name(repo);
    println!("{:?}", updated);

    // Mutable setter
    let desc_setter = |mut f: Box<dyn FnMut(&mut Option<String>)>| {
        Box::new(move |r: &mut Repo| {
            (f)(&mut r.desc);
        }) as Box<dyn FnMut(&mut Repo)>
    };

    let mut set_desc = mset(desc_setter, Some("Where in the world is Blob?".to_string()));
    let mut repo2 = Repo {
        name: "Blob".into(),
        desc: None,
    };
    set_desc(&mut repo2);
    println!("{:?}", repo2);

    // Immutable setter (Repo -> Repo)
    let name_setter = |f: Box<dyn Fn(String) -> String>| {
        Box::new(move |mut r: Repo| {
            r.name = f(r.name);
            r
        }) as Box<dyn Fn(Repo) -> Repo>
    };

    let set_name = set(name_setter, "Nomadic Blob".to_string());
    let repo = Repo {
        name: "Old".into(),
        desc: None,
    };
    let updated = set_name(repo);
    println!("{:?}", updated);

    // Mutable setter (&mut Repo)
    let desc_setter = |mut f: Box<dyn FnMut(&mut Option<String>)>| {
        Box::new(move |r: &mut Repo| {
            (f)(&mut r.desc);
        }) as Box<dyn FnMut(&mut Repo)>
    };

    let mut set_desc = mset(desc_setter, Some("Where in the world is Blob?".to_string()));
    let mut repo2 = Repo {
        name: "Blob".into(),
        desc: None,
    };
    set_desc(&mut repo2);
    println!("{:?}", repo2);

    let mut transform = concat_mut(vec![
        mset(desc_setter, Some("Blob 1".into())),
        mset(desc_setter, Some("Blob 2".into())),
    ]);

    let name_setter = |f: Box<dyn Fn(String) -> String>| {
        Box::new(move |mut r: Repo| {
            r.name = f(r.name);
            r
        }) as Box<dyn Fn(Repo) -> Repo>
    };

    // plain function usage
    let r1 = with(Repo::mock(), |mut r| {
        r.name = "Nomadic Blob".into();
        r
    });
    println!("{:?}", r1);

    // macro usage
    let r2 = with(Repo::mock(), |mut r: Repo| {
        r.desc = Some("Where in the world is Blob?".into());
        r
    });
    println!("{:?}", r2);

    let desc_setter = |mut f: Box<dyn FnMut(&mut Option<String>)>| {
        Box::new(move |r: &mut Repo| {
            (f)(&mut r.desc);
        }) as Box<dyn FnMut(&mut Repo)>
    };

    // let repo1 = with(Repo::mock(), concat(vec![
    //     set!(name_setter, "Nomadic Blob".into()),
    //     set!(desc_setter, Some("Immutable style".into())),
    // ]));

    let repo2 = with_mut(
        Repo::mock(),
        concat_mut(vec![
            mset(desc_setter, Some("Blob story".into())),
            mset(desc_setter, Some("Final Blob".into())),
        ]),
    );

    println!("{:?}", repo2);
}
