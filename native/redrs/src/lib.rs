use std::sync::RwLock;
use std::sync::Arc;

use rustler::{Env, NifResult, Term};
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
fn do_command<'a>(env: Env<'a>, wconn: ResourceArc<Conn>, args: Term) -> NifResult<Term<'a>> {
    let mut conn = wconn.conn.write().unwrap();

    let mut eargs = args.decode::<rustler::ListIterator>()?;
    let cmd : String = eargs.next().ok_or(rustler::Error::BadArg)?.decode()?;

    let mut query = redis::cmd(cmd.as_str());
    for earg in eargs {
        let arg : String = earg.decode()?;
        query.arg(arg);
    }
    
    match query.query(&mut conn) {
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
fn close<'a>(env: Env<'a>, state: ResourceArc<State>) -> NifResult<Term<'a>> {
  drop(state);

  Ok(atoms::ok().encode(env))
}

fn load(env: Env, _: Term) -> bool {
  rustler::resource!(State, env);
  rustler::resource!(Conn, env);
  true
}

rustler::init!("Elixir.RedRS", [open, close, get_connection, do_command], load=load);
