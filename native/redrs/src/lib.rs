use std::sync::RwLock;
use std::sync::Arc;

use rustler::{Env, NifResult, Term, Atom};
use rustler::types::Encoder;
use rustler::resource::ResourceArc;

struct State {
    client: redis::Client
}

struct Conn {
    conn: Arc<RwLock<redis::Connection>>
}

#[rustler::nif]
fn open<'a>(env: Env<'a>, url: &'a str) -> NifResult<Term<'a>> {
    match redis::Client::open(url) {
        Ok(client) => {
            let state = ResourceArc::new(State{
                client: client
            });
            Ok((atom_from_str(env, "ok"), state).encode(env))
        }
        Err(error) =>
            Ok((atom_from_str(env, "error"), format!("{}", error)).encode(env))
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn get_connection<'a>(env: Env<'a>, state: ResourceArc<State>) -> NifResult<Term<'a>> {
    match state.client.get_connection() {
        Ok(conn) => {
            let wrap = ResourceArc::new(Conn{conn: Arc::new(RwLock::new(conn))});
            Ok((atom_from_str(env, "ok"), wrap).encode(env))
        }
        Err(error) =>
            Ok((atom_from_str(env, "error"), format!("{}", error)).encode(env))
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn get<'a>(env: Env<'a>, wconn: ResourceArc<Conn>, key: &'a str) -> NifResult<Term<'a>> {
    use redis::Commands;
    let mut conn = wconn.conn.write().unwrap();

    match conn.get(key) {
        Ok(result) => {
            // TODO: how can we support more types?
            let value : Option<String> = result;
            Ok((atom_from_str(env, "ok"), value).encode(env))
        }
        Err(error) =>
            Ok((atom_from_str(env, "error"), format!("{}", error)).encode(env))
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn set<'a>(env: Env<'a>, wconn: ResourceArc<Conn>, key: &'a str, value: &'a str) -> NifResult<Term<'a>> {
    use redis::Commands;

    let mut conn = wconn.conn.write().unwrap();

    match conn.set(key, value) {
        Ok(()) => Ok(atom_from_str(env, "ok").encode(env)),
        Err(error) => Ok((atom_from_str(env, "error"), format!("{}", error)).encode(env))
    }
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
  rustler::resource!(Conn, env);
  true
}

rustler::init!("Elixir.RedRS", [open, close, get, set, get_connection], load=load);
