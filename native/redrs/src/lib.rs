use std::sync::RwLock;
use std::sync::Arc;

use rustler::atoms;
use rustler::{Env, NifResult, Term, Atom};
use rustler::types::Encoder;
use rustler::resource::ResourceArc;

struct State {
    client: redis::Client
}

struct Conn {
    conn: Arc<RwLock<redis::Connection>>
}

mod atoms {
    rustler::atoms! {
        ok,
        error
    }
}

#[rustler::nif]
fn open<'a>(env: Env<'a>, url: &'a str) -> NifResult<Term<'a>> {
    match redis::Client::open(url) {
        Ok(client) => {
            let state = ResourceArc::new(State{
                client: client
            });
            Ok((atoms::ok(), state).encode(env))
        }
        Err(error) =>
            Ok((atoms::error(), format!("{}", error)).encode(env))
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn get_connection<'a>(env: Env<'a>, state: ResourceArc<State>) -> NifResult<Term<'a>> {
    match state.client.get_connection() {
        Ok(conn) => {
            let wrap = ResourceArc::new(Conn{conn: Arc::new(RwLock::new(conn))});
            Ok((atoms::ok(), wrap).encode(env))
        }
        Err(error) =>
            Ok((atoms::error(), format!("{}", error)).encode(env))
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
            Ok((atoms::ok(), value).encode(env))
        }
        Err(error) =>
            Ok((atoms::error(), format!("{}", error)).encode(env))
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn set<'a>(env: Env<'a>, wconn: ResourceArc<Conn>, key: &'a str, value: &'a str) -> NifResult<Term<'a>> {
    use redis::Commands;

    let mut conn = wconn.conn.write().unwrap();

    match conn.set(key, value) {
        Ok(()) => Ok(atoms::ok().encode(env)),
        Err(error) => Ok((atoms::error(), format!("{}", error)).encode(env))
    }
}

#[rustler::nif(schedule = "DirtyIo")]    
fn close<'a>(env: Env<'a>, state: ResourceArc<State>) -> NifResult<Term<'a>> {
  drop(state);

  Ok(atoms::ok().encode(env))
}

fn load(env: Env, _: Term) -> bool {
  rustler::resource!(State, env);
  rustler::resource!(Conn, env);
  true
}

rustler::init!("Elixir.RedRS", [open, close, get, set, get_connection], load=load);
