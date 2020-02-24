
use {
    crate::{
        actor::*,
        board::*,
        consts::*,
        pos::*,
    },
    super::{
        ink::*,
    },
};

/// an action which may be kept in a stack
/// for redoing from a saved board state
#[derive(Debug)]
pub enum DrawingAction {
    DotInk (Ink, Pos),
    LineInk (Ink, Pos, Pos),
    RectInk (Ink, Pos, Pos),
    DefaultCell(Cell),
}

/// apply a drop of ink at some pos of the board.
/// Take care of keeping only one Lapin and only
/// one actor or item at most per cell.
fn ink_pos(ink: Ink, pos: Pos, board: &mut Board) {
    match ink {
        Ink::EraseTerrain => {
            board.cells.unset(pos);
        }
        Ink::Terrain(cell) => {
            board.cells.set(pos, cell);
        }
        Ink::EraseItem => {
            board.items.remove(pos);
        }
        Ink::Item(item_kind) => {
            board.add_item_in(item_kind, pos.x, pos.y);
        }
        Ink::EraseActor => {
            board.actors.retain(|&actor| actor.pos != pos);
        }
        Ink::Actor(actor_kind) => {
            if actor_kind == ActorKind::Lapin {
                // we're just moving THE lapin
                board.actors[0].pos = pos;
                return;
            }
            // we check we're not removing the lapin
            if pos == board.lapin_pos() {
                return; // we make it a no-op
            }
            // we must now ensure any previous actor in pos is removed
            board.actors.retain(|&actor| actor.pos != pos);
            // and now we add the new actor
            board.add_actor_in(actor_kind, pos.x, pos.y);
        }
    }
}

/// draw a line between two pos.
/// based on Bresenham's algorithm.
/// Prevents crossing in quadrant dirs (diagonal possible)
fn ink_line(ink: Ink, a: Pos, b: Pos, board: &mut Board) {
    let Pos {mut x, mut y} = a;
    let Pos {x: xf, y: yf} = b;
    let dx = (xf - x).abs();
    let sx = if x<xf { 1 } else { -1 };
    let dy = -(yf - y).abs();
    let sy = if y<yf { 1 } else { -1 };
    let mut e = dx + dy;
    loop {
        ink_pos(ink, Pos::new(x, y), board);
        if x==xf && y==yf {
            break;
        }
        let e2 = 2*e;
        if e2 >= dy {
            e += dy;
            x += sx;
        }
        if e2 <= dx {
            e += dx;
            y += sy;
        }
    }
}

/// fill a rect given two corners
fn ink_rect(ink: Ink, a: Pos, b: Pos, board: &mut Board) {
    for x in a.x.min(b.x)..=a.x.max(b.x) {
        for y in a.y.min(b.y)..=a.y.max(b.y) {
            ink_pos(ink, Pos::new(x, y), board);
        }
    }
}

impl DrawingAction {
    pub fn apply_to(&self, board: &mut Board) {
        match self {
            DrawingAction::DotInk(ink, pos) => {
                ink_pos(*ink, *pos, board);
            }
            DrawingAction::LineInk(ink, a, b) => {
                ink_line(*ink, *a, *b, board);
            }
            DrawingAction::RectInk(ink, a, b) => {
                ink_rect(*ink, *a, *b, board);
            }
            DrawingAction::DefaultCell(cell) => {
                board.cells.default = *cell;
            }
        }
    }
}