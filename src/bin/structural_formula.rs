use std::{
    env, fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy::prelude::*;
use iupac::{graph::Graph, parser::parse, Element};

const FONT_SIZE: f32 = 32.0;
const BOND_DRAWING_EXCLUSION_RADIUS: f32 = 0.5 * FONT_SIZE;
const ATOM_ADDITION_RATE: f32 = 10.0;
const ITER_PER_FIXED_UPDATE: usize = 100;
const HIDE_HYDROGENS: bool = true;

const STEP_SIZE: f32 = 0.5;
const BOND_STIFFNESS: f32 = 1.0;
const HYDROGEN_BOND_TARGET_LENGTH: f32 = 1.5 * FONT_SIZE;
const BOND_TARGET_LENGTH: f32 = 2.0 * FONT_SIZE;
const ATOM_REPULSION: f32 = 10.0;
const BOND_REPULSION: f32 = 2000.0;
const CENTER_PULL: f32 = 0.2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(Startup, setup)
        .add_systems(Update, draw)
        .add_systems(FixedUpdate, gradient_descent)
        .run();
}

#[derive(Component, Default)]
struct Molecule {
    graph: Graph,
    atoms: Vec<Entity>,
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
struct Atom;

#[derive(Bundle)]
struct AtomBundle {
    text: Text2dBundle,
    atom: Atom,
}

impl AtomBundle {
    fn new(element: Element, text_style: TextStyle) -> Self {
        AtomBundle {
            text: Text2dBundle {
                text: Text::from_section(element.symbol(), text_style)
                    .with_justify(JustifyText::Center),
                ..Default::default()
            },
            atom: Atom,
        }
    }
}

#[derive(Component)]
struct CostText;

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
        font_size: FONT_SIZE,
        color: Color::BLACK,
    };

    commands.spawn(Camera2dBundle::default());

    let name = env::args().nth(1).unwrap();
    let ast = parse(&name);
    let graph = Graph::from(&*ast);

    let mut atoms = Vec::new();
    for &atom in graph.atoms.iter() {
        let atom = commands
            .spawn(AtomBundle::new(atom, text_style.clone()))
            .id();
        atoms.push(atom);
    }
    commands
        .spawn(MoleculeBundle {
            molecule: Molecule {
                graph,
                atoms: atoms.clone(),
            },
            ..Default::default()
        })
        .push_children(&atoms);

    commands.spawn((
        CostText,
        TextBundle::from_section(
            "Cost: ...",
            TextStyle {
                font,
                font_size: FONT_SIZE,
                color: Color::BLACK,
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    ));
}

fn draw(
    mut gizmos: Gizmos,
    molecule: Query<&Molecule>,
    positions: Query<&Transform>,
    mut visibilities: Query<&mut Visibility>,
) {
    fn receed_endpoint(endpoint: &mut Vec2, other_endpoint: Vec2) {
        let displacement = other_endpoint - *endpoint;
        let distance = displacement.length();
        if distance > BOND_DRAWING_EXCLUSION_RADIUS {
            *endpoint += (displacement / distance) * BOND_DRAWING_EXCLUSION_RADIUS;
        }
    }

    let molecule = molecule.single();
    let g = &molecule.graph;

    for bond in &molecule.graph.bonds {
        let &(a, b) = bond;
        if skip(g, a) || skip(g, b) {
            continue;
        }
        let a = molecule.atoms[a];
        let b = molecule.atoms[b];

        let mut a_position = positions.get(a).unwrap().translation.xy();
        let mut b_position = positions.get(b).unwrap().translation.xy();
        receed_endpoint(&mut a_position, b_position);
        receed_endpoint(&mut b_position, a_position);

        gizmos.line_2d(a_position, b_position, Color::BLACK);
    }

    for (i, &atom) in molecule.atoms.iter().enumerate() {
        let mut visibility = visibilities.get_mut(atom).unwrap();
        if skip(g, i) {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Inherited;
        }
    }
}

static LAST_SHOWN_ATOM: AtomicUsize = AtomicUsize::new(0);

fn gradient_descent(
    molecule: Query<&Molecule>,
    mut atoms: Query<(&Atom, &mut Transform)>,
    mut cost_text: Query<(&CostText, &mut Text)>,
    time: Res<Time>,
) {
    let molecule = molecule.single();
    let g = &molecule.graph;
    let mut atom_positions = molecule
        .atoms
        .iter()
        .map(|&e| atoms.get(e).unwrap().1.translation.xy())
        .collect::<Vec<_>>();

    let max_index = (time.elapsed_seconds() * ATOM_ADDITION_RATE) as usize;
    LAST_SHOWN_ATOM.store(max_index, Ordering::SeqCst);

    let start_of_index = max_index as f32 / ATOM_ADDITION_RATE;
    let first_step_for_index = (time.elapsed_seconds() - start_of_index) < time.delta_seconds();
    if first_step_for_index {
        let new_i = max_index;
        if new_i < atom_positions.len() {
            atom_positions[new_i] =
                place_new_atom(&molecule.graph, &atom_positions, new_i, max_index);
        }
    }

    for _ in 0..ITER_PER_FIXED_UPDATE {
        let cost_gradient = cost_gradient(&molecule.graph, &atom_positions);
        for i in 0..atom_positions.len() {
            atom_positions[i] -= STEP_SIZE * cost_gradient[i];
        }
    }
    let cost = cost(&molecule.graph, &atom_positions);

    let (sum, count) = atom_positions
        .iter()
        .enumerate()
        .filter(|&(i, _)| !skip(g, i))
        .fold((Vec2::ZERO, 0), |(sum, count), (_, &position)| {
            (sum + position, count + 1)
        });
    let center = sum / count as f32;
    for position in atom_positions.iter_mut().take(max_index + 1) {
        *position -= CENTER_PULL * center;
    }

    for (i, &entity) in molecule.atoms.iter().enumerate() {
        let mut transform = atoms.get_mut(entity).unwrap().1;
        transform.translation.x = atom_positions[i].x;
        transform.translation.y = atom_positions[i].y;
    }

    let (_, mut cost_text) = cost_text.single_mut();
    cost_text.sections[0].value = format!("Cost: {}", cost.round());
}

fn place_new_atom(graph: &Graph, atom_positions: &[Vec2], new_i: usize, max_index: usize) -> Vec2 {
    let mut neighbor_position_sum = Vec2::ZERO;
    let mut neighbor_count = 0.0;
    let mut second_neighbor_position_sum = Vec2::ZERO;
    let mut second_neighbor_count = 0.0;
    for j in graph.neighbors(new_i) {
        if j > max_index {
            continue;
        }
        neighbor_position_sum += atom_positions[j];
        neighbor_count += 1.0;
        for k in graph.neighbors(j) {
            if k > max_index || k == new_i {
                continue;
            }
            second_neighbor_position_sum += atom_positions[k];
            second_neighbor_count += 1.0;
        }
    }
    let neighbor_mean = neighbor_position_sum / neighbor_count;
    let second_neighbor_mean = second_neighbor_position_sum / second_neighbor_count;

    let position = if !neighbor_mean.is_nan() && !second_neighbor_mean.is_nan() {
        let seed_direction = neighbor_mean - second_neighbor_mean;
        neighbor_mean + BOND_TARGET_LENGTH * seed_direction.normalize()
    } else {
        Vec2::ZERO
    };
    let predictable_noise = Vec2::from_angle(0.1 * new_i as f32);
    position + predictable_noise
}

fn cost(g: &Graph, atom_positions: &[Vec2]) -> f32 {
    // Model the cost as the potential energy of a mechanical system
    let mut energy = 0.0;

    // Model bonds as springs
    for bond in &g.bonds {
        let &(i, j) = bond;
        if skip(g, i) || skip(g, j) {
            continue;
        }

        let u_vec = atom_positions[j] - atom_positions[i];
        let u = u_vec.length();
        let target_length = if g.atoms[i] == Element::Hydrogen || g.atoms[j] == Element::Hydrogen {
            HYDROGEN_BOND_TARGET_LENGTH
        } else {
            BOND_TARGET_LENGTH
        };
        let x = u - target_length;
        energy += 0.5 * BOND_STIFFNESS * x.powi(2);
    }

    // Model atoms as repelling charges
    for i in 0..g.atoms.len() {
        for j in i + 1..g.atoms.len() {
            if skip(g, i) || skip(g, j) {
                continue;
            }

            let r_vec = atom_positions[j] - atom_positions[i];
            let r = r_vec.length();
            energy += ATOM_REPULSION / r;
        }
    }

    // Model bonds as repelling charges
    foreach_adjacent_bond_pair(g, |(a, b), (c, d)| {
        if skip(g, a) || skip(g, b) || skip(g, c) || skip(g, d) {
            return;
        }

        let ab_center = 0.5 * (atom_positions[a] + atom_positions[b]);
        let cd_center = 0.5 * (atom_positions[c] + atom_positions[d]);
        let r_vec = cd_center - ab_center;
        let r = r_vec.length();
        if r <= f32::EPSILON {
            return;
        }

        energy += BOND_REPULSION / r;
    });

    energy
}

fn cost_gradient(g: &Graph, atom_positions: &[Vec2]) -> Vec<Vec2> {
    let mut energy_gradient = vec![Vec2::ZERO; atom_positions.len()];

    for bond in &g.bonds {
        let &(i, j) = bond;
        if skip(g, i) || skip(g, j) {
            continue;
        }

        let u_vec = atom_positions[j] - atom_positions[i];
        let u = u_vec.length();
        let target_length = if g.atoms[i] == Element::Hydrogen || g.atoms[j] == Element::Hydrogen {
            HYDROGEN_BOND_TARGET_LENGTH
        } else {
            BOND_TARGET_LENGTH
        };
        let x = u - target_length;
        let dx_by_du_vec = u_vec / u;
        let denergy_by_du_vec = BOND_STIFFNESS * x * dx_by_du_vec;
        energy_gradient[j] += denergy_by_du_vec;
        energy_gradient[i] -= denergy_by_du_vec;
    }

    for i in 0..g.atoms.len() {
        for j in i + 1..g.atoms.len() {
            if skip(g, i) || skip(g, j) {
                continue;
            }

            let r_vec = atom_positions[j] - atom_positions[i];
            let r = r_vec.length();
            let dr_by_dr_vec = r_vec / r;
            let denergy_by_dr_vec = -ATOM_REPULSION / r.powi(2) * dr_by_dr_vec;
            energy_gradient[j] += denergy_by_dr_vec;
            energy_gradient[i] -= denergy_by_dr_vec;
        }
    }

    foreach_adjacent_bond_pair(g, |(a, b), (c, d)| {
        if skip(g, a) || skip(g, b) || skip(g, c) || skip(g, d) {
            return;
        }

        let ab_center = 0.5 * (atom_positions[a] + atom_positions[b]);
        let cd_center = 0.5 * (atom_positions[c] + atom_positions[d]);
        let r_vec = cd_center - ab_center;
        let r = r_vec.length();
        let dr_by_dr_vec = r_vec / r;
        if r <= f32::EPSILON {
            return;
        }

        let denergy_by_dr_vec = -BOND_REPULSION / r.powi(2) * dr_by_dr_vec;
        let denergy_by_dposition = 0.5 * denergy_by_dr_vec;
        energy_gradient[a] -= denergy_by_dposition;
        energy_gradient[b] -= denergy_by_dposition;
        energy_gradient[c] += denergy_by_dposition;
        energy_gradient[d] += denergy_by_dposition;
    });

    energy_gradient
}

fn foreach_adjacent_bond_pair(g: &Graph, mut callback: impl FnMut((usize, usize), (usize, usize))) {
    for i in 0..g.bonds.len() {
        for j in i + 1..g.bonds.len() {
            let (a, b) = g.bonds[i];
            let (c, d) = g.bonds[j];
            if a == c || a == d || b == c || b == d {
                callback(g.bonds[i], g.bonds[j]);
            }
        }
    }
}

fn skip(g: &Graph, index: usize) -> bool {
    let max_index = LAST_SHOWN_ATOM.load(Ordering::SeqCst);
    if index > max_index {
        return true;
    }

    let element = g.atoms[index];
    if HIDE_HYDROGENS && element == Element::Hydrogen {
        return true;
    }

    false
}
