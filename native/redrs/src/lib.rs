use std::sync::Arc;

use rustler::{Env, NifResult, Term, Atom};
use rustler::types::Encoder;
use rustler::resource::ResourceArc;

struct State {
}

#[rustler::nif]
fn open<'a>(env: Env<'a>, url: &'a str) -> NifResult<ResourceArc<State>> {
    let state = ResourceArc::new(State{
    });
    
    Ok(state)
}

#[rustler::nif(schedule = "DirtyIo")]
fn close<'a>(env: Env<'a>, state: ResourceArc<State>) -> NifResult<Term<'a>> {
  drop(state);

  Ok(atom_from_str(env, "ok").encode(env))
}


fn atom_from_str(env: Env, name: &str) -> Atom {
  Atom::from_str(env, name).unwrap()
}

fn load(env: Env, _: Term) -> bool {
  rustler::resource!(State, env);
  true
}

rustler::init!("Elixir.RedRS", [open, close], load=load);
