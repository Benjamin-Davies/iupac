use std::{env, fs, path::PathBuf, process::Command};

use bevy::prelude::*;
use iupac::{graph::Graph, parser::parse, Element};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, bond_springs)
        .add_systems(FixedUpdate, atomic_repulsion)
        .run();
}

#[derive(Component, Default)]
struct Molecule;

#[derive(Bundle, Default)]
struct MoleculeBundle {
    molecule: Molecule,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
}

#[derive(Component)]
struct Atom {
    element: Element,
}

#[derive(Bundle)]
struct AtomBundle {
    text: Text2dBundle,
    atom: Atom,
}

impl AtomBundle {
    fn new(transform: Transform, element: Element, text_style: TextStyle) -> Self {
        AtomBundle {
            text: Text2dBundle {
                text: Text::from_section(element.symbol(), text_style)
                    .with_justify(JustifyText::Center),
                transform,
                ..Default::default()
            },
            atom: Atom { element },
        }
    }
}

#[derive(Component)]
struct Bond {
    atoms: [Entity; 2],
}

#[derive(Bundle)]
struct BondBundle {
    bond: Bond,
}

impl BondBundle {
    fn new(atoms: [Entity; 2]) -> Self {
        BondBundle {
            bond: Bond { atoms },
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_path = PathBuf::from("assets/fonts/FiraSans-Bold.ttf");
    if !font_path.exists() {
        fs::create_dir_all(font_path.parent().unwrap()).unwrap();
        let font_url = "https://raw.githubusercontent.com/bevyengine/bevy/latest/assets/fonts/FiraSans-Bold.ttf";
        Command::new("wget")
            .arg("-O")
            .arg(font_path)
            .arg(font_url)
            .status()
            .unwrap();
    }

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 32.0,
        color: Color::BLACK,
    };

    commands.spawn(Camera2dBundle::default());

    let name = env::args().nth(1).unwrap();
    let ast = parse(&name);
    let graph = Graph::from(&*ast);

    let molecule = commands.spawn(MoleculeBundle::default()).id();
    let mut atoms = Vec::new();
    for (i, &atom) in graph.atoms.iter().enumerate() {
        let x = i as f32 * 20.0;
        let y = if i % 2 == 0 { 0.0 } else { 20.0 };
        let transform = Transform::from_translation(Vec3::new(x, y, 0.0));
        let atom = commands
            .spawn(AtomBundle::new(transform, atom, text_style.clone()))
            .set_parent(molecule)
            .id();
        atoms.push(atom);
    }
    for &(i, j) in &graph.bonds {
        let atoms = [atoms[i], atoms[j]];
        commands.spawn(BondBundle::new(atoms)).set_parent(molecule);
    }
}

fn bond_springs(mut atoms: Query<(&Atom, &mut Transform)>, bonds: Query<&Bond>) {
    let bond_length = 50.0;
    let speed = 0.2;

    for bond in &bonds {
        let [a, b] = bond.atoms;
        let (_, a_transform) = atoms.get(a).unwrap();
        let (_, b_transform) = atoms.get(b).unwrap();

        let direction = b_transform.translation - a_transform.translation;
        let distance = direction.length();
        let target_distance = (1.0 - speed) * distance + speed * bond_length;
        let delta = direction * (target_distance - distance) / distance / 2.0;

        let (_, mut a_transform) = atoms.get_mut(a).unwrap();
        a_transform.translation -= delta;
        let (_, mut b_transform) = atoms.get_mut(b).unwrap();
        b_transform.translation += delta;
    }
}

fn atomic_repulsion(atoms: Query<(&Atom, Entity)>, mut transforms: Query<&mut Transform>) {
    let radius = 100.0;
    let repulsion = 2.0;

    for (atom_a, a) in &atoms {
        for (atom_b, b) in &atoms {
            if a == b {
                continue;
            }

            let a_transform = transforms.get(a).unwrap();
            let b_transform = transforms.get(b).unwrap();
            let direction = b_transform.translation - a_transform.translation;
            let distance = direction.length();
            let delta = direction * repulsion / distance / distance;

            if distance > radius {
                continue;
            }

            if atom_a.element == Element::Hydrogen || atom_b.element != Element::Hydrogen {
                let mut a_transform = transforms.get_mut(a).unwrap();
                a_transform.translation -= delta;
            }
            if atom_b.element == Element::Hydrogen || atom_a.element != Element::Hydrogen {
                let mut b_transform = transforms.get_mut(b).unwrap();
                b_transform.translation += delta;
            }
        }
    }
}
