use std::{env, fs, path::PathBuf, process::Command};

use bevy::prelude::*;
use iupac::{graph::Graph, parser::parse, Element};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, gradient_descent)
        .run();
}

#[derive(Component, Default)]
struct Molecule {
    graph: Graph,
    atoms: Vec<Entity>,
    bonds: Vec<Entity>,
}

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
    // TODO: draw gizmos
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

    let mut atoms = Vec::new();
    for (i, &atom) in graph.atoms.iter().enumerate() {
        let x = i as f32 * 20.0;
        let y = if i % 2 == 0 { 0.0 } else { 20.0 };
        let transform = Transform::from_translation(Vec3::new(x, y, 0.0));
        let atom = commands
            .spawn(AtomBundle::new(transform, atom, text_style.clone()))
            .id();
        atoms.push(atom);
    }
    let mut bonds = Vec::new();
    for &(i, j) in &graph.bonds {
        let atoms = [atoms[i], atoms[j]];
        let bond = commands.spawn(BondBundle::new(atoms)).id();
        bonds.push(bond);
    }
    commands
        .spawn(MoleculeBundle {
            molecule: Molecule {
                graph,
                atoms: atoms.clone(),
                bonds: bonds.clone(),
            },
            ..Default::default()
        })
        .push_children(&atoms)
        .push_children(&bonds);
}

const STEP_SIZE: f32 = 0.1;
const BOND_STIFFNESS: f32 = 1.0;
const BOND_TARGET_LENGTH: f32 = 40.0;
const ATOM_REPULSION: f32 = 100.0;

fn gradient_descent(molecule: Query<&Molecule>, mut atoms: Query<(&Atom, &mut Transform)>) {
    let molecule = molecule.single();
    let mut atom_positions = molecule
        .atoms
        .iter()
        .map(|&e| atoms.get(e).unwrap().1.translation.xy())
        .collect::<Vec<_>>();

    let cost_gradient = cost_gradient(&molecule.graph, &atom_positions);
    for i in 0..atom_positions.len() {
        atom_positions[i] -= STEP_SIZE * cost_gradient[i];
    }

    for (i, &entity) in molecule.atoms.iter().enumerate() {
        let mut transform = atoms.get_mut(entity).unwrap().1;
        transform.translation.x = atom_positions[i].x;
        transform.translation.y = atom_positions[i].y;
    }
}

fn cost(graph: &Graph, atom_positions: &[Vec2]) -> f32 {
    // Model the cost as the potential energy of a mechanical system
    let mut energy = 0.0;

    // Model bonds as springs
    for bond in &graph.bonds {
        let &(i, j) = bond;
        let u_vec = atom_positions[j] - atom_positions[i];
        let u = u_vec.length();
        let x = u - BOND_TARGET_LENGTH;
        energy += 0.5 * BOND_STIFFNESS * x.powi(2);
    }

    // Model atoms as repelling charges
    for i in 0..atom_positions.len() {
        for j in i + 1..atom_positions.len() {
            let r_vec = atom_positions[j] - atom_positions[i];
            let r = r_vec.length();
            energy += ATOM_REPULSION / r;
        }
    }

    energy
}

fn cost_gradient(graph: &Graph, atom_positions: &[Vec2]) -> Vec<Vec2> {
    let mut energy_gradient = vec![Vec2::ZERO; atom_positions.len()];

    for bond in &graph.bonds {
        let &(i, j) = bond;
        let u_vec = atom_positions[j] - atom_positions[i];
        let u = u_vec.length();
        let x = u - BOND_TARGET_LENGTH;
        let dx_by_du_vec = u_vec / u;
        let denergy_by_du_vec = BOND_STIFFNESS * x * dx_by_du_vec;
        energy_gradient[j] += denergy_by_du_vec;
        energy_gradient[i] -= denergy_by_du_vec;
    }

    for i in 0..atom_positions.len() {
        for j in i + 1..atom_positions.len() {
            let r_vec = atom_positions[j] - atom_positions[i];
            let r = r_vec.length();
            let dr_by_dr_vec = r_vec / r;
            let denergy_by_dr_vec = -ATOM_REPULSION / r.powi(2) * dr_by_dr_vec;
            energy_gradient[j] += denergy_by_dr_vec;
            energy_gradient[i] -= denergy_by_dr_vec;
        }
    }

    energy_gradient
}
